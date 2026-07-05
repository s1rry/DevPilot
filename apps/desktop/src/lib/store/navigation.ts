import { create } from "zustand";

/** Identifier of a primary view, one per feature slice. */
export type ViewId =
  | "repository"
  | "analysis"
  | "architecture"
  | "ai-chat"
  | "insights"
  | "settings";

interface NavigationState {
  /** View currently shown in the content area. */
  activeView: ViewId;
  /** Whether the sidebar is collapsed to an icon rail. */
  sidebarCollapsed: boolean;
  /** Selects a view. */
  setActiveView: (view: ViewId) => void;
  /** Toggles the sidebar between full and collapsed. */
  toggleSidebar: () => void;
  /** Forces the collapsed state, used by responsive breakpoints. */
  setSidebarCollapsed: (collapsed: boolean) => void;
}

/**
 * UI navigation store: which view is active and whether the sidebar is
 * collapsed. Deliberately holds no domain data — it is pure shell state.
 */
export const useNavigationStore = create<NavigationState>((set) => ({
  activeView: "repository",
  sidebarCollapsed: false,
  setActiveView: (view) => set({ activeView: view }),
  toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
  setSidebarCollapsed: (collapsed) => set({ sidebarCollapsed: collapsed }),
}));
