import { Clock, FolderGit2, X } from "lucide-react";

import { formatRelativeTime } from "@/shared/format";
import { useRepositoryStore } from "@/features/repository/store";

/**
 * The recent-projects list. Each row reopens its project on click and can be
 * removed with the trailing button. Shows an empty state when there are none.
 */
export function RecentProjectsList() {
  const recent = useRepositoryStore((state) => state.recent);
  const reopen = useRepositoryStore((state) => state.reopen);
  const remove = useRepositoryStore((state) => state.remove);
  const busy = useRepositoryStore((state) => state.status !== "idle");

  if (recent.length === 0) {
    return (
      <div className="flex flex-col items-center gap-2 rounded-lg border border-dashed border-border p-8 text-center">
        <Clock size={22} strokeWidth={1.75} className="text-muted" />
        <p className="text-sm text-muted">No recent projects yet.</p>
      </div>
    );
  }

  return (
    <ul className="flex flex-col gap-1">
      {recent.map((project) => (
        <li key={project.id}>
          <div className="group flex items-center gap-3 rounded-md border border-transparent px-3 py-2 hover:border-border hover:bg-surface">
            <button
              type="button"
              onClick={() => void reopen(project.local_path)}
              disabled={busy}
              className="flex min-w-0 flex-1 items-center gap-3 text-left outline-none disabled:cursor-not-allowed"
            >
              <FolderGit2 size={16} strokeWidth={2} className="shrink-0 text-accent" />
              <span className="min-w-0">
                <span className="block truncate text-sm text-fg">{project.name}</span>
                <span className="block truncate text-xs text-muted">{project.local_path}</span>
              </span>
            </button>
            <span className="shrink-0 text-xs text-muted">
              {formatRelativeTime(project.last_opened)}
            </span>
            <button
              type="button"
              onClick={() => void remove(project.id)}
              title="Remove from recent"
              aria-label={`Remove ${project.name} from recent`}
              className="shrink-0 rounded p-1 text-muted opacity-0 outline-none transition-opacity hover:bg-elevated hover:text-fg focus-visible:opacity-100 focus-visible:ring-2 focus-visible:ring-accent group-hover:opacity-100"
            >
              <X size={14} strokeWidth={2} />
            </button>
          </div>
        </li>
      ))}
    </ul>
  );
}
