import { create } from "zustand";

import { chat, type ChatMessage } from "@/lib/ipc/chat";
import { pickFolder } from "@/lib/ipc/dialog";

interface ChatState {
  /** Project folder the chat reasons about. */
  projectPath: string | null;
  /** Conversation, oldest first. */
  messages: ChatMessage[];
  /** Whether a reply is currently streaming. */
  streaming: boolean;
  /** Last error message, if a turn failed. */
  error: string | null;

  /** Opens the folder picker and sets the project. */
  pickProject: () => Promise<void>;
  /** Sends a user message and streams the reply. */
  send: (text: string) => Promise<void>;
  /** Clears the conversation. */
  clear: () => void;
}

function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/** AI Chat store. Holds the conversation and drives streaming replies. */
export const useChatStore = create<ChatState>((set, get) => ({
  projectPath: null,
  messages: [],
  streaming: false,
  error: null,

  pickProject: async () => {
    const path = await pickFolder().catch(() => null);
    if (path) {
      set({ projectPath: path, error: null });
    }
  },

  send: async (text: string) => {
    const trimmed = text.trim();
    const path = get().projectPath;
    if (!trimmed || get().streaming) {
      return;
    }
    if (!path) {
      set({ error: "Choose a project folder to chat about first." });
      return;
    }

    const userMessage: ChatMessage = { role: "user", content: trimmed };
    const outgoing = [...get().messages, userMessage];
    // Append the user turn plus an empty assistant turn to stream into.
    set({
      messages: [...outgoing, { role: "assistant", content: "" }],
      streaming: true,
      error: null,
    });

    const appendToken = (token: string) => {
      set((state) => {
        const messages = state.messages.slice();
        const last = messages[messages.length - 1];
        messages[messages.length - 1] = { ...last, content: last.content + token };
        return { messages };
      });
    };

    try {
      await chat(path, outgoing, appendToken);
      set({ streaming: false });
    } catch (error) {
      set((state) => {
        // Drop the empty assistant placeholder on failure.
        const messages = state.messages.slice();
        if (messages.length && messages[messages.length - 1].content === "") {
          messages.pop();
        }
        return { messages, streaming: false, error: messageOf(error) };
      });
    }
  },

  clear: () => set({ messages: [], error: null }),
}));

// Dev-only debug handle for seeding the store from preview tooling. Stripped
// from production builds.
if (import.meta.env.DEV && typeof window !== "undefined") {
  (window as unknown as { __chatStore?: typeof useChatStore }).__chatStore = useChatStore;
}
