import { Files, FolderTree, GitBranch, GitCommitHorizontal, Package, Users } from "lucide-react";

import { formatRelativeTime } from "@/shared/format";
import type { ScanReport } from "@/lib/ipc/scan";
import { LanguageBars } from "@/features/analysis/components/LanguageBars";
import { useT, useTn } from "@/lib/store/i18n";

/** A titled section with a leading icon. */
function Section({
  icon: Icon,
  title,
  children,
}: {
  icon: typeof Files;
  title: string;
  children: React.ReactNode;
}) {
  return (
    <section className="flex flex-col gap-3 rounded-lg border border-border bg-canvas p-4">
      <h3 className="flex items-center gap-2 text-sm font-semibold text-fg">
        <Icon size={16} strokeWidth={2} className="text-muted" />
        {title}
      </h3>
      {children}
    </section>
  );
}

/** A compact labeled number card. */
function Card({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-md border border-border bg-surface px-3 py-2">
      <span className="block text-xs text-muted">{label}</span>
      <span className="block truncate text-sm font-medium text-fg">{value}</span>
    </div>
  );
}

interface ScanReportViewProps {
  /** The report to render. */
  report: ScanReport;
}

/** Renders a complete repository scan report. */
export function ScanReportView({ report }: ScanReportViewProps) {
  const { git, structure, frameworks, dependencies, languages } = report;
  const t = useT();
  const tn = useTn();

  return (
    <div className="flex flex-col gap-4">
      <div className="grid grid-cols-2 gap-2 sm:grid-cols-4">
        <Card label={t("scan.branch")} value={git.branch} />
        <Card label={t("scan.commits")} value={git.commit_count.toLocaleString()} />
        <Card label={t("scan.files")} value={structure.total_files.toLocaleString()} />
        <Card label={t("scan.directories")} value={structure.total_directories.toLocaleString()} />
      </div>

      <Section icon={Files} title={t("scan.languages")}>
        <LanguageBars languages={languages} />
      </Section>

      <Section icon={Package} title={`${t("scan.frameworks")} (${frameworks.length})`}>
        {frameworks.length === 0 ? (
          <p className="text-sm text-muted">{t("scan.noFrameworks")}</p>
        ) : (
          <div className="flex flex-wrap gap-2">
            {frameworks.map((framework) => (
              <span
                key={`${framework.name}-${framework.source}`}
                className="flex items-center gap-1.5 rounded-full border border-border bg-surface px-2.5 py-1 text-xs text-fg"
                title={`${framework.category} · ${framework.source}`}
              >
                {framework.name}
                <span className="text-muted">{framework.category}</span>
              </span>
            ))}
          </div>
        )}
      </Section>

      <Section icon={Package} title={`${t("scan.dependencies")} (${dependencies.length})`}>
        {dependencies.length === 0 ? (
          <p className="text-sm text-muted">{t("scan.noDependencies")}</p>
        ) : (
          <ul className="flex max-h-64 flex-col gap-1 overflow-auto">
            {dependencies.map((dependency) => (
              <li
                key={`${dependency.ecosystem}-${dependency.name}`}
                className="flex items-center gap-2 text-sm"
              >
                <span className="w-12 shrink-0 text-xs text-muted">{dependency.ecosystem}</span>
                <span className="min-w-0 flex-1 truncate text-fg">{dependency.name}</span>
                {dependency.version && (
                  <span className="shrink-0 text-xs text-muted">{dependency.version}</span>
                )}
              </li>
            ))}
          </ul>
        )}
      </Section>

      <Section icon={FolderTree} title={t("scan.structure")}>
        {structure.notable.length > 0 && (
          <div className="flex flex-wrap gap-2">
            {structure.notable.map((dir) => (
              <span
                key={dir}
                className="rounded-md border border-border bg-surface px-2 py-0.5 text-xs text-fg"
              >
                {dir}/
              </span>
            ))}
          </div>
        )}
        <p className="text-xs text-muted">
          {t("scan.topLevelDirs", { count: structure.top_level_dirs.length })}
        </p>
      </Section>

      <Section icon={GitBranch} title={t("scan.git")}>
        {git.last_commit && (
          <div className="flex items-start gap-2 text-sm">
            <GitCommitHorizontal size={16} strokeWidth={2} className="mt-0.5 shrink-0 text-muted" />
            <span className="min-w-0">
              <span className="block truncate text-fg">{git.last_commit.summary}</span>
              <span className="block text-xs text-muted">
                {git.last_commit.author_name} · {formatRelativeTime(git.last_commit.timestamp)}
              </span>
            </span>
          </div>
        )}
        {git.contributors.length > 0 && (
          <div className="flex flex-col gap-1.5">
            <span className="flex items-center gap-1.5 text-xs font-medium text-muted">
              <Users size={13} strokeWidth={2} />
              {t("scan.topContributors")}
            </span>
            {git.contributors.map((contributor) => (
              <div key={contributor.email} className="flex items-center gap-2 text-sm">
                <span className="min-w-0 flex-1 truncate text-fg">{contributor.name}</span>
                <span className="shrink-0 text-xs text-muted">
                  {tn("commits", contributor.commit_count)}
                </span>
              </div>
            ))}
          </div>
        )}
      </Section>
    </div>
  );
}
