import { AlertCircle, Copy, FolderOpen, Loader2, Search, Skull, Waypoints } from "lucide-react";

import { Button } from "@/shared/ui/Button";
import type { SearchHit } from "@/lib/ipc/intel";
import { useInsightsStore } from "@/features/insights/store";
import { useChatStore } from "@/features/ai-chat/store";
import { useNavigationStore } from "@/lib/store/navigation";
import { useT } from "@/lib/store/i18n";

/** A titled section with a leading icon and a count badge. */
function Section({
  icon: Icon,
  title,
  count,
  children,
}: {
  icon: typeof Waypoints;
  title: string;
  count: number;
  children: React.ReactNode;
}) {
  const t = useT();
  return (
    <section className="flex flex-col gap-2 rounded-lg border border-border bg-canvas p-4">
      <h3 className="flex items-center gap-2 text-sm font-semibold text-fg">
        <Icon size={16} strokeWidth={2} className="text-muted" />
        {title}
        <span className="rounded-full border border-border px-2 py-0.5 text-xs text-muted">
          {count}
        </span>
      </h3>
      {count === 0 ? <p className="text-sm text-muted">{t("common.noneFound")}</p> : children}
    </section>
  );
}

/**
 * Insights: deterministic Code Intelligence over a project — cyclic
 * dependencies, dead code and duplication — plus a code search that answers
 * "where is X" and can hand a symbol to AI Chat to explain.
 */
export function InsightsView() {
  const projectPath = useInsightsStore((state) => state.projectPath);
  const report = useInsightsStore((state) => state.report);
  const analyzing = useInsightsStore((state) => state.analyzing);
  const query = useInsightsStore((state) => state.query);
  const hits = useInsightsStore((state) => state.hits);
  const searching = useInsightsStore((state) => state.searching);
  const error = useInsightsStore((state) => state.error);
  const pickProject = useInsightsStore((state) => state.pickProject);
  const analyze = useInsightsStore((state) => state.analyze);
  const setQuery = useInsightsStore((state) => state.setQuery);
  const search = useInsightsStore((state) => state.search);
  const t = useT();

  const explain = (hit: SearchHit) => {
    if (!projectPath) {
      return;
    }
    const target = hit.symbol ?? hit.path;
    useChatStore.setState({ projectPath });
    useNavigationStore.getState().setActiveView("ai-chat");
    void useChatStore.getState().send(`Explain \`${target}\` in ${hit.path}.`);
  };

  return (
    <div className="mx-auto flex h-full w-full max-w-3xl flex-col gap-5 overflow-auto p-6">
      <section className="flex flex-col gap-3">
        <div className="flex items-center gap-3">
          <Button icon={FolderOpen} onClick={() => void pickProject()}>
            {projectPath ? t("common.changeProject") : t("common.chooseProject")}
          </Button>
          {projectPath && (
            <span className="min-w-0 flex-1 truncate text-xs text-muted">{projectPath}</span>
          )}
          <Button
            variant="primary"
            icon={analyzing ? Loader2 : undefined}
            onClick={() => void analyze()}
            disabled={!projectPath || analyzing}
          >
            {analyzing ? t("common.analyzing") : t("common.analyze")}
          </Button>
        </div>

        <form
          onSubmit={(event) => {
            event.preventDefault();
            void search();
          }}
          className="flex gap-2"
        >
          <div className="flex h-9 flex-1 items-center gap-2 rounded-md border border-border bg-canvas px-3">
            <Search size={15} strokeWidth={2} className="shrink-0 text-muted" />
            <input
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder={t("insights.searchPlaceholder")}
              className="w-full bg-transparent text-sm text-fg outline-none placeholder:text-muted"
            />
          </div>
          <Button type="submit" disabled={!projectPath || searching}>
            {searching ? "…" : t("insights.search")}
          </Button>
        </form>
      </section>

      {error && (
        <div className="flex items-start gap-2 rounded-md border border-border bg-surface px-3 py-2 text-sm text-fg">
          <AlertCircle size={16} strokeWidth={2} className="mt-0.5 shrink-0 text-accent" />
          <span className="min-w-0 break-words">{error}</span>
        </div>
      )}

      {hits.length > 0 && (
        <section className="flex flex-col gap-2 rounded-lg border border-border bg-canvas p-4">
          <h3 className="text-sm font-semibold text-fg">{t("insights.searchResults")}</h3>
          <ul className="flex flex-col gap-1">
            {hits.map((hit, index) => (
              <li
                key={`${hit.path}-${hit.line}-${index}`}
                className="group flex items-center gap-2 rounded-md px-2 py-1.5 hover:bg-surface"
              >
                <span className="min-w-0 flex-1">
                  {hit.symbol && <span className="text-sm text-fg">{hit.symbol}</span>}
                  <span className="block truncate text-xs text-muted">
                    {hit.path}:{hit.line}
                  </span>
                </span>
                <button
                  type="button"
                  onClick={() => explain(hit)}
                  className="shrink-0 rounded border border-border px-2 py-1 text-xs text-muted opacity-0 transition-opacity hover:bg-elevated hover:text-fg focus-visible:opacity-100 group-hover:opacity-100"
                >
                  {t("insights.explain")}
                </button>
              </li>
            ))}
          </ul>
        </section>
      )}

      {report && (
        <div className="flex flex-col gap-4">
          <Section
            icon={Waypoints}
            title={t("insights.cyclicDependencies")}
            count={report.cyclic_dependencies.length}
          >
            <ul className="flex flex-col gap-1 text-sm text-fg">
              {report.cyclic_dependencies.map((cycle, index) => (
                <li key={index} className="truncate">
                  {cycle.nodes.join(" → ")}
                </li>
              ))}
            </ul>
          </Section>

          <Section icon={Skull} title={t("insights.deadCode")} count={report.dead_code.length}>
            <ul className="flex flex-col gap-1 text-sm">
              {report.dead_code.map((symbol, index) => (
                <li key={index} className="flex items-center gap-2">
                  <span className="text-fg">{symbol.name}</span>
                  <span className="truncate text-xs text-muted">
                    {symbol.file}:{symbol.line}
                  </span>
                </li>
              ))}
            </ul>
          </Section>

          <Section icon={Copy} title={t("insights.duplication")} count={report.duplication.length}>
            <ul className="flex flex-col gap-2 text-sm">
              {report.duplication.map((group, index) => (
                <li key={index} className="flex flex-col gap-0.5">
                  <span className="text-xs text-muted">
                    {t("insights.duplicationSummary", {
                      copies: group.occurrences.length,
                      lines: group.line_count,
                    })}
                  </span>
                  {group.occurrences.map((occurrence, occurrenceIndex) => (
                    <span key={occurrenceIndex} className="truncate text-xs text-fg">
                      {occurrence.file}:{occurrence.start_line}–{occurrence.end_line}
                    </span>
                  ))}
                </li>
              ))}
            </ul>
          </Section>
        </div>
      )}

      {!report && !analyzing && hits.length === 0 && !error && (
        <div className="flex flex-1 flex-col items-center justify-center gap-3 p-8 text-center">
          <div className="flex h-14 w-14 items-center justify-center rounded-xl border border-border bg-surface text-muted">
            <Waypoints size={26} strokeWidth={1.75} />
          </div>
          <p className="max-w-sm text-sm text-muted">{t("insights.emptyHint")}</p>
        </div>
      )}
    </div>
  );
}
