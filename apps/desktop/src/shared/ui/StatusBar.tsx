import { Circle, GitBranch } from "lucide-react";

import { APP_VERSION } from "@/lib/version";
import { navItem } from "@/shared/navigation";
import { useNavigationStore } from "@/lib/store/navigation";

/**
 * Bottom status bar. Shows the current view and connection placeholder on the
 * left and the app version on the right. Values are static in this shell.
 */
export function StatusBar() {
  const activeView = useNavigationStore((state) => state.activeView);
  const current = navItem(activeView);

  return (
    <footer className="flex h-6 shrink-0 items-center justify-between border-t border-border bg-surface px-3 text-xs text-muted">
      <div className="flex items-center gap-3">
        <span className="flex items-center gap-1">
          <GitBranch size={12} strokeWidth={2} />
          No repository
        </span>
        <span>{current.label}</span>
      </div>
      <div className="flex items-center gap-3">
        <span className="flex items-center gap-1">
          <Circle size={8} strokeWidth={0} className="fill-accent" />
          Ready
        </span>
        <span>v{APP_VERSION}</span>
      </div>
    </footer>
  );
}
