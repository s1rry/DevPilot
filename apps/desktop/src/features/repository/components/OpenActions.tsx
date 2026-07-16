import { useState } from "react";
import { FolderOpen, Github, Loader2 } from "lucide-react";

import { Button } from "@/shared/ui/Button";
import { useRepositoryStore } from "@/features/repository/store";
import { useT } from "@/lib/store/i18n";

/**
 * The primary actions of the Repository Manager: open a local folder via the
 * native picker, or clone a remote repository by URL.
 */
export function OpenActions() {
  const status = useRepositoryStore((state) => state.status);
  const openFolderDialog = useRepositoryStore((state) => state.openFolderDialog);
  const clone = useRepositoryStore((state) => state.clone);
  const t = useT();
  const [url, setUrl] = useState("");

  const busy = status !== "idle";
  const canClone = url.trim().length > 0 && !busy;

  const submitClone = (event: React.FormEvent) => {
    event.preventDefault();
    if (canClone) {
      void clone(url.trim());
    }
  };

  return (
    <div className="flex flex-col gap-3">
      <Button
        variant="primary"
        icon={status === "opening" ? Loader2 : FolderOpen}
        onClick={() => void openFolderDialog()}
        disabled={busy}
      >
        {t("repo.openFolder")}
      </Button>

      <form onSubmit={submitClone} className="flex gap-2">
        <div className="flex h-9 flex-1 items-center gap-2 rounded-md border border-border bg-canvas px-3">
          <Github size={15} strokeWidth={2} className="shrink-0 text-muted" />
          <input
            value={url}
            onChange={(event) => setUrl(event.target.value)}
            placeholder={t("repo.cloneUrlPlaceholder")}
            disabled={busy}
            className="w-full bg-transparent text-sm text-fg outline-none placeholder:text-muted disabled:cursor-not-allowed"
          />
        </div>
        <Button
          type="submit"
          icon={status === "cloning" ? Loader2 : undefined}
          disabled={!canClone}
        >
          {status === "cloning" ? t("repo.cloning") : t("repo.clone")}
        </Button>
      </form>
    </div>
  );
}
