import { useEffect } from "react";
import { AlertCircle } from "lucide-react";

import { OpenActions } from "@/features/repository/components/OpenActions";
import { ProjectMetadataPanel } from "@/features/repository/components/ProjectMetadataPanel";
import { RecentProjectsList } from "@/features/repository/components/RecentProjectsList";
import { useRepositoryStore } from "@/features/repository/store";
import { isTauri } from "@/lib/ipc/env";
import { useT } from "@/lib/store/i18n";

/**
 * Repository Manager. Lets the user open a local folder or clone a remote
 * repository, browse recent projects, and view the opened project's metadata.
 * All backend access goes through the repository store and the typed IPC
 * layer; this component holds no business logic.
 */
export function RepositoryView() {
  const metadata = useRepositoryStore((state) => state.metadata);
  const error = useRepositoryStore((state) => state.error);
  const loadRecent = useRepositoryStore((state) => state.loadRecent);
  const t = useT();

  // Load the recent-projects list once when the view first mounts. Skipped
  // outside the desktop shell, where there is no backend to answer.
  useEffect(() => {
    if (isTauri()) {
      void loadRecent();
    }
  }, [loadRecent]);

  return (
    <div className="mx-auto flex h-full w-full max-w-3xl flex-col gap-6 p-6">
      <section className="flex flex-col gap-3">
        <h2 className="text-sm font-medium text-muted">{t("repo.openProject")}</h2>
        <OpenActions />
      </section>

      {error && (
        <div className="flex items-start gap-2 rounded-md border border-border bg-surface px-3 py-2 text-sm text-fg">
          <AlertCircle size={16} strokeWidth={2} className="mt-0.5 shrink-0 text-accent" />
          <span className="min-w-0 break-words">{error}</span>
        </div>
      )}

      {metadata && <ProjectMetadataPanel metadata={metadata} />}

      <section className="flex min-h-0 flex-col gap-3">
        <h2 className="text-sm font-medium text-muted">{t("repo.recentProjects")}</h2>
        <RecentProjectsList />
      </section>
    </div>
  );
}
