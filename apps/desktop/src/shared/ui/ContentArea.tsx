import { AiChatView } from "@/features/ai-chat/AiChatView";
import { AnalysisView } from "@/features/analysis/AnalysisView";
import { ArchitectureView } from "@/features/architecture/ArchitectureView";
import { InsightsView } from "@/features/insights/InsightsView";
import { RepositoryView } from "@/features/repository/RepositoryView";
import { SettingsView } from "@/features/settings/SettingsView";
import { navItem } from "@/shared/navigation";
import { useNavigationStore } from "@/lib/store/navigation";
import type { ViewId } from "@/lib/store/navigation";
import { useT } from "@/lib/store/i18n";

/** Maps a view id to its feature component. */
function renderView(view: ViewId) {
  switch (view) {
    case "repository":
      return <RepositoryView />;
    case "analysis":
      return <AnalysisView />;
    case "architecture":
      return <ArchitectureView />;
    case "ai-chat":
      return <AiChatView />;
    case "insights":
      return <InsightsView />;
    case "settings":
      return <SettingsView />;
  }
}

/**
 * The main content region: a panel header naming the current view above the
 * active feature slice. Fills the remaining space between sidebar and edges.
 */
export function ContentArea() {
  const activeView = useNavigationStore((state) => state.activeView);
  const current = navItem(activeView);
  const t = useT();

  return (
    <section className="flex min-w-0 flex-1 flex-col overflow-hidden bg-canvas">
      <div className="flex h-10 shrink-0 items-center border-b border-border px-4">
        <h1 className="text-sm font-medium text-fg">{t(current.label)}</h1>
      </div>
      <div className="min-h-0 flex-1 overflow-auto">{renderView(activeView)}</div>
    </section>
  );
}
