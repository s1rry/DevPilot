import { Circle, GitBranch } from "lucide-react";

import { APP_VERSION } from "@/lib/version";
import { navItem } from "@/shared/navigation";
import { useNavigationStore } from "@/lib/store/navigation";
import { useT } from "@/lib/store/i18n";

/**
 * Bottom status bar. Shows the current view and connection placeholder on the
 * left and the app version on the right. Values are static in this shell.
 */
export function StatusBar() {
  const activeView = useNavigationStore((state) => state.activeView);
  const current = navItem(activeView);
  const t = useT();

  return (
    <footer className="flex h-6 shrink-0 items-center justify-between border-t border-border bg-surface px-3 text-xs text-muted">
      <div className="flex items-center gap-3">
        <span className="flex items-center gap-1">
          <GitBranch size={12} strokeWidth={2} />
          {t("status.noRepository")}
        </span>
        <span>{t(current.label)}</span>
      </div>
      <div className="flex items-center gap-3">
        <span className="flex items-center gap-1">
          <Circle size={8} strokeWidth={0} className="fill-accent" />
          {t("status.ready")}
        </span>
        <span>v{APP_VERSION}</span>
      </div>
    </footer>
  );
}
