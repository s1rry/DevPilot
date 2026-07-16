import { Languages, Moon, PanelLeft, Search, Sun } from "lucide-react";

import { IconButton } from "@/shared/ui/IconButton";
import { useNavigationStore } from "@/lib/store/navigation";
import { useThemeStore } from "@/lib/store/theme";
import { useI18nStore, useT } from "@/lib/store/i18n";

/**
 * Application top bar: brand, a disabled search affordance, and global
 * actions (sidebar toggle, language toggle, theme toggle). The search field
 * is presentational only in this shell.
 */
export function TopBar() {
  const toggleSidebar = useNavigationStore((state) => state.toggleSidebar);
  const theme = useThemeStore((state) => state.theme);
  const toggleTheme = useThemeStore((state) => state.toggleTheme);
  const language = useI18nStore((state) => state.language);
  const toggleLanguage = useI18nStore((state) => state.toggleLanguage);
  const t = useT();

  const switchLanguageLabel =
    language === "ru" ? t("topbar.switchToEnglish") : t("topbar.switchToRussian");

  return (
    <header className="flex h-12 shrink-0 items-center gap-3 border-b border-border bg-surface px-3">
      <IconButton icon={PanelLeft} label={t("topbar.toggleSidebar")} onClick={toggleSidebar} />

      <div className="flex items-center gap-2">
        <span className="text-sm font-semibold tracking-tight text-fg">DevPilot</span>
      </div>

      <div className="flex flex-1 justify-center">
        <div className="flex h-8 w-full max-w-md items-center gap-2 rounded-md border border-border bg-canvas px-3 text-muted">
          <Search size={15} strokeWidth={2} />
          <span className="truncate text-sm">{t("topbar.searchPlaceholder")}</span>
        </div>
      </div>

      <button
        type="button"
        onClick={toggleLanguage}
        title={switchLanguageLabel}
        aria-label={switchLanguageLabel}
        className="flex h-9 items-center gap-1.5 rounded-md px-2 text-sm text-muted outline-none transition-colors hover:bg-elevated hover:text-fg focus-visible:ring-2 focus-visible:ring-accent"
      >
        <Languages size={18} strokeWidth={2} />
        <span className="font-medium uppercase">{language}</span>
      </button>

      <IconButton
        icon={theme === "dark" ? Sun : Moon}
        label={theme === "dark" ? t("topbar.switchToLight") : t("topbar.switchToDark")}
        onClick={toggleTheme}
      />
    </header>
  );
}
