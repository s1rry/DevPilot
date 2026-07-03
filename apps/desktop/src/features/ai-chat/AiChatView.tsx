import { EmptyState } from "@/shared/ui/EmptyState";
import { navItem } from "@/shared/navigation";

/**
 * AI chat view. Placeholder in this shell; the streaming chat over LLM
 * providers arrives with the AI feature.
 */
export function AiChatView() {
  const item = navItem("ai-chat");
  return <EmptyState icon={item.icon} title={item.label} hint={item.hint} phase={item.phase} />;
}
