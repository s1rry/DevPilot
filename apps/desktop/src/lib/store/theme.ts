import { create } from "zustand";
import { persist } from "zustand/middleware";

/** Available color themes. */
export type Theme = "dark" | "light";

interface ThemeState {
  /** Currently active theme. */
  theme: Theme;
  /** Switches between dark and light. */
  toggleTheme: () => void;
  /** Sets a specific theme. */
  setTheme: (theme: Theme) => void;
}

/**
 * Applies the theme to the document root so the CSS variables in
 * `styles/globals.css` take effect. Safe to call outside React.
 */
function applyTheme(theme: Theme): void {
  document.documentElement.setAttribute("data-theme", theme);
}

/**
 * Theme store, persisted to localStorage under `devpilot-theme`.
 *
 * The default is dark. The persisted value is re-applied to the DOM on
 * rehydration so a reload keeps the chosen theme without a flash.
 */
export const useThemeStore = create<ThemeState>()(
  persist(
    (set, get) => ({
      theme: "dark",
      toggleTheme: () => {
        const next: Theme = get().theme === "dark" ? "light" : "dark";
        applyTheme(next);
        set({ theme: next });
      },
      setTheme: (theme) => {
        applyTheme(theme);
        set({ theme });
      },
    }),
    {
      name: "devpilot-theme",
      onRehydrateStorage: () => (state) => {
        applyTheme(state?.theme ?? "dark");
      },
    },
  ),
);
