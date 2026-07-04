import { useEffect, useRef } from "react";

import type { ChatMessage } from "@/lib/ipc/chat";
import { Markdown } from "@/features/ai-chat/components/Markdown";

interface MessageListProps {
  /** Conversation messages, oldest first. */
  messages: ChatMessage[];
  /** Whether the last assistant message is still streaming. */
  streaming: boolean;
}

/**
 * Renders the conversation: user messages as plain right-aligned bubbles,
 * assistant messages as Markdown. Auto-scrolls to the latest message.
 */
export function MessageList({ messages, streaming }: MessageListProps) {
  const bottom = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottom.current?.scrollIntoView({ block: "end" });
  }, [messages]);

  return (
    <div className="flex flex-col gap-4">
      {messages.map((message, index) => {
        const isUser = message.role === "user";
        const isLast = index === messages.length - 1;
        return (
          <div key={index} className={`flex ${isUser ? "justify-end" : "justify-start"}`}>
            <div
              className={`max-w-[85%] rounded-lg px-3 py-2 ${
                isUser
                  ? "bg-accent text-accent-fg"
                  : "border border-border bg-surface text-fg"
              }`}
            >
              {isUser ? (
                <p className="whitespace-pre-wrap text-sm">{message.content}</p>
              ) : message.content ? (
                <Markdown content={message.content} />
              ) : (
                streaming && isLast && <span className="text-sm text-muted">Thinking…</span>
              )}
            </div>
          </div>
        );
      })}
      <div ref={bottom} />
    </div>
  );
}
