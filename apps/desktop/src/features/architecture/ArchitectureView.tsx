import { AlertCircle, FolderOpen, Loader2, Network } from "lucide-react";

import { Button } from "@/shared/ui/Button";
import { GraphCanvas } from "@/features/architecture/components/GraphCanvas";
import { graphOf, useArchitectureStore, type GraphKind } from "@/features/architecture/store";
import type { TranslationKey } from "@/lib/i18n/en";
import { useT } from "@/lib/store/i18n";

/** The selectable graphs, in tab order. Labels are translation keys. */
const GRAPHS: { kind: GraphKind; label: TranslationKey }[] = [
  { kind: "dependency", label: "arch.graph.dependency" },
  { kind: "module", label: "arch.graph.module" },
  { kind: "folder", label: "arch.graph.folder" },
  { kind: "call", label: "arch.graph.call" },
];

/** Legend entries: node kind and color, matching `GraphCanvas`. */
const LEGEND: { label: TranslationKey; color: string }[] = [
  { label: "arch.legend.file", color: "#6366f1" },
  { label: "arch.legend.module", color: "#a855f7" },
  { label: "arch.legend.directory", color: "#64748b" },
  { label: "arch.legend.function", color: "#22c55e" },
  { label: "arch.legend.external", color: "#475569" },
];

/**
 * Architecture: interactive dependency, module, folder and call graphs built
 * from the AST. Pan, zoom and drag nodes. Backed by `analyze_architecture`.
 */
export function ArchitectureView() {
  const projectPath = useArchitectureStore((state) => state.projectPath);
  const model = useArchitectureStore((state) => state.model);
  const analyzing = useArchitectureStore((state) => state.analyzing);
  const activeGraph = useArchitectureStore((state) => state.activeGraph);
  const error = useArchitectureStore((state) => state.error);
  const pickProject = useArchitectureStore((state) => state.pickProject);
  const analyze = useArchitectureStore((state) => state.analyze);
  const setGraph = useArchitectureStore((state) => state.setGraph);
  const t = useT();

  const graph = model ? graphOf(model, activeGraph) : null;

  return (
    <div className="flex h-full flex-col">
      <div className="flex shrink-0 flex-wrap items-center gap-3 border-b border-border px-4 py-2">
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

      {model && (
        <div className="flex shrink-0 flex-wrap items-center justify-between gap-3 border-b border-border px-4 py-2">
          <div className="flex gap-1">
            {GRAPHS.map(({ kind, label }) => {
              const count = graphOf(model, kind).nodes.length;
              const active = kind === activeGraph;
              return (
                <button
                  key={kind}
                  type="button"
                  onClick={() => setGraph(kind)}
                  className={`rounded-md px-2.5 py-1 text-xs transition-all duration-200 ${
                    active
                      ? "dp-accent-surface dp-accent-glow text-accent-fg"
                      : "text-muted hover:bg-elevated hover:text-fg"
                  }`}
                >
                  {t(label)} <span className="opacity-70">{count}</span>
                </button>
              );
            })}
          </div>
          <div className="flex flex-wrap items-center gap-3">
            {LEGEND.map((entry) => (
              <span key={entry.label} className="flex items-center gap-1 text-xs text-muted">
                <span
                  className="inline-block h-2.5 w-2.5 rounded-full"
                  style={{ backgroundColor: entry.color }}
                />
                {t(entry.label)}
              </span>
            ))}
          </div>
        </div>
      )}

      {error && (
        <div className="mx-4 mt-2 flex items-start gap-2 rounded-md border border-border bg-surface px-3 py-2 text-sm text-fg">
          <AlertCircle size={16} strokeWidth={2} className="mt-0.5 shrink-0 text-accent" />
          <span className="min-w-0 break-words">{error}</span>
        </div>
      )}

      <div className="min-h-0 flex-1">
        {graph ? (
          <GraphCanvas graph={graph} />
        ) : (
          !analyzing && (
            <div className="flex h-full flex-col items-center justify-center gap-4 p-8 text-center">
              <div className="dp-empty-icon flex h-16 w-16 items-center justify-center rounded-2xl border border-border bg-elevated text-accent-strong">
                <Network size={28} strokeWidth={1.75} />
              </div>
              <p className="max-w-sm text-sm leading-relaxed text-muted">{t("arch.emptyHint")}</p>
            </div>
          )
        )}
      </div>
    </div>
  );
}
