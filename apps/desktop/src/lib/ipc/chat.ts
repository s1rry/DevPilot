import { Channel, invoke } from "@tauri-apps/api/core";

/** Role of a chat message, matching the core `Role` (serialized lowercase). */
export type ChatRole = "system" | "user" | "assistant";

/** One chat message. */
export interface ChatMessage {
  role: ChatRole;
  content: string;
}

/**
 * Runs one repository-aware chat turn, invoking `onToken` for each streamed
 * reply token. Resolves when the reply is complete.
 *
 * @param path - Project folder to reason about.
 * @param messages - Full conversation so far, oldest first, ending with the
 *   user's latest message.
 * @param onToken - Called with each reply token as it arrives.
 */
export function chat(
  path: string,
  messages: ChatMessage[],
  onToken: (token: string) => void,
): Promise<void> {
  const channel = new Channel<string>();
  channel.onmessage = onToken;
  return invoke<void>("chat", { path, messages, onToken: channel });
}
