import { useState } from "react";
import { SendHorizontal } from "lucide-react";

import { useT } from "@/lib/store/i18n";

interface ChatInputProps {
  /** Whether input is disabled (a reply is streaming). */
  disabled: boolean;
  /** Sends the typed message. */
  onSend: (text: string) => void;
}

/**
 * Message composer. Enter sends; Shift+Enter inserts a newline.
 */
export function ChatInput({ disabled, onSend }: ChatInputProps) {
  const [text, setText] = useState("");
  const t = useT();

  const submit = () => {
    const trimmed = text.trim();
    if (trimmed && !disabled) {
      onSend(trimmed);
      setText("");
    }
  };

  return (
    <div className="flex items-end gap-2 border-t border-border bg-canvas p-3">
      <textarea
        value={text}
        onChange={(event) => setText(event.target.value)}
        onKeyDown={(event) => {
          if (event.key === "Enter" && !event.shiftKey) {
            event.preventDefault();
            submit();
          }
        }}
        rows={1}
        placeholder={t("chat.inputPlaceholder")}
        className="max-h-40 min-h-9 flex-1 resize-none rounded-md border border-border bg-surface px-3 py-2 text-sm text-fg outline-none placeholder:text-muted focus-visible:ring-2 focus-visible:ring-accent"
      />
      <button
        type="button"
        onClick={submit}
        disabled={disabled || text.trim().length === 0}
        title={t("chat.send")}
        aria-label={t("chat.sendMessage")}
        className="flex h-9 w-9 items-center justify-center rounded-md bg-accent text-accent-fg outline-none transition-opacity hover:opacity-90 focus-visible:ring-2 focus-visible:ring-accent disabled:cursor-not-allowed disabled:opacity-40"
      >
        <SendHorizontal size={16} strokeWidth={2} />
      </button>
    </div>
  );
}
