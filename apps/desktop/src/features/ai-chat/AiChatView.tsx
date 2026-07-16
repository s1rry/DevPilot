import { AlertCircle, FolderOpen, MessageSquare, Trash2 } from "lucide-react";

import { Button } from "@/shared/ui/Button";
import { ChatInput } from "@/features/ai-chat/components/ChatInput";
import { MessageList } from "@/features/ai-chat/components/MessageList";
import { useChatStore } from "@/features/ai-chat/store";
import { useT } from "@/lib/store/i18n";

/**
 * AI Chat: a repository-aware conversation with the configured LLM provider.
 * Streams replies token by token, renders Markdown with code blocks, and
 * keeps the conversation history. The provider and model come from Settings.
 */
export function AiChatView() {
  const projectPath = useChatStore((state) => state.projectPath);
  const messages = useChatStore((state) => state.messages);
  const streaming = useChatStore((state) => state.streaming);
  const error = useChatStore((state) => state.error);
  const pickProject = useChatStore((state) => state.pickProject);
  const send = useChatStore((state) => state.send);
  const clear = useChatStore((state) => state.clear);
  const t = useT();

  return (
    <div className="flex h-full flex-col">
      <div className="flex shrink-0 items-center gap-3 border-b border-border px-4 py-2">
        <Button icon={FolderOpen} onClick={() => void pickProject()}>
          {projectPath ? t("chat.changeProject") : t("chat.chooseProject")}
        </Button>
        {projectPath && (
          <span className="min-w-0 flex-1 truncate text-xs text-muted">{projectPath}</span>
        )}
        {messages.length > 0 && (
          <Button icon={Trash2} onClick={clear}>
            {t("chat.clear")}
          </Button>
        )}
      </div>

      <div className="min-h-0 flex-1 overflow-auto">
        {messages.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center gap-3 p-8 text-center">
            <div className="flex h-14 w-14 items-center justify-center rounded-xl border border-border bg-surface text-muted">
              <MessageSquare size={26} strokeWidth={1.75} />
            </div>
            <p className="max-w-sm text-sm text-muted">
              {projectPath ? t("chat.emptyWithProject") : t("chat.emptyNoProject")}
            </p>
          </div>
        ) : (
          <div className="mx-auto w-full max-w-3xl p-4">
            <MessageList messages={messages} streaming={streaming} />
          </div>
        )}
      </div>

      {error && (
        <div className="mx-4 mb-2 flex items-start gap-2 rounded-md border border-border bg-surface px-3 py-2 text-sm text-fg">
          <AlertCircle size={16} strokeWidth={2} className="mt-0.5 shrink-0 text-accent" />
          <span className="min-w-0 break-words">{error}</span>
        </div>
      )}

      <div className="mx-auto w-full max-w-3xl">
        <ChatInput disabled={streaming} onSend={(text) => void send(text)} />
      </div>
    </div>
  );
}
