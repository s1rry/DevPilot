import { GitBranch, GitCommitHorizontal, HardDrive, Files } from "lucide-react";

import { formatBytes } from "@/shared/format";
import type { ProjectMetadata } from "@/lib/ipc/repository";

interface ProjectMetadataPanelProps {
  /** Metadata of the opened project. */
  metadata: ProjectMetadata;
}

/** A single labeled stat chip. */
function Stat({
  icon: Icon,
  label,
  value,
}: {
  icon: typeof GitBranch;
  label: string;
  value: string;
}) {
  return (
    <div className="flex items-center gap-2 rounded-md border border-border bg-surface px-3 py-2">
      <Icon size={16} strokeWidth={2} className="shrink-0 text-muted" />
      <span className="min-w-0">
        <span className="block text-xs text-muted">{label}</span>
        <span className="block truncate text-sm text-fg">{value}</span>
      </span>
    </div>
  );
}

/**
 * Shows the metadata of the currently opened project: identity, git facts and
 * a language breakdown. Read-only.
 */
export function ProjectMetadataPanel({ metadata }: ProjectMetadataPanelProps) {
  const totalFiles = metadata.languages.reduce((sum, stat) => sum + stat.file_count, 0);

  return (
    <div className="flex flex-col gap-4 rounded-lg border border-border bg-canvas p-4">
      <div className="min-w-0">
        <h3 className="truncate text-base font-semibold text-fg">{metadata.name}</h3>
        <p className="truncate text-xs text-muted">{metadata.local_path}</p>
      </div>

      <div className="grid grid-cols-2 gap-2">
        <Stat icon={GitBranch} label="Branch" value={metadata.branch} />
        <Stat
          icon={GitCommitHorizontal}
          label="Commits"
          value={metadata.commit_count.toLocaleString()}
        />
        <Stat icon={Files} label="Files" value={metadata.file_count.toLocaleString()} />
        <Stat icon={HardDrive} label="Size" value={formatBytes(metadata.total_size_bytes)} />
      </div>

      {metadata.languages.length > 0 && (
        <div className="flex flex-col gap-2">
          <span className="text-xs font-medium text-muted">Languages</span>
          <div className="flex flex-col gap-1.5">
            {metadata.languages.map((stat) => {
              const percent = totalFiles > 0 ? (stat.file_count / totalFiles) * 100 : 0;
              return (
                <div key={stat.language} className="flex items-center gap-2">
                  <span className="w-24 shrink-0 truncate text-xs text-fg">{stat.language}</span>
                  <div className="h-2 flex-1 overflow-hidden rounded-full bg-elevated">
                    <div className="h-full rounded-full bg-accent" style={{ width: `${percent}%` }} />
                  </div>
                  <span className="w-10 shrink-0 text-right text-xs text-muted">
                    {stat.file_count}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
