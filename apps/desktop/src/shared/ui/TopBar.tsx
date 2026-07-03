import { Moon, PanelLeft, Search, Sun } from "lucide-react";

import { IconButton } from "@/shared/ui/IconButton";
import { useNavigationStore } from "@/lib/store/navigation";
import { useThemeStore } from "@/lib/store/theme";

/**
 * Application top bar: brand, a disabled search affordance, and global
 * actions (sidebar toggle, theme toggle). The search field is presentational
 * only in this shell.
 */
export function TopBar() {
  const toggleSidebar = useNavigationStore((state) => state.toggleSidebar);
  const theme = useThemeStore((state) => state.theme);
  const toggleTheme = useThemeStore((state) => state.toggleTheme);

  return (
    <header className="flex h-12 shrink-0 items-center gap-3 border-b border-border bg-surface px-3">
      <IconButton icon={PanelLeft} label="Toggle sidebar" onClick={toggleSidebar} />

      <div className="flex items-center gap-2">
        <span className="text-sm font-semibold tracking-tight text-fg">DevPilot</span>
      </div>

      <div className="flex flex-1 justify-center">
        <div className="flex h-8 w-full max-w-md items-center gap-2 rounded-md border border-border bg-canvas px-3 text-muted">
          <Search size={15} strokeWidth={2} />
          <span className="truncate text-sm">Search — coming soon</span>
        </div>
      </div>

      <IconButton
        icon={theme === "dark" ? Sun : Moon}
        label={theme === "dark" ? "Switch to light theme" : "Switch to dark theme"}
        onClick={toggleTheme}
      />
    </header>
  );
}
