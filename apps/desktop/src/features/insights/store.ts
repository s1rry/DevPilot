import { create } from "zustand";

import { pickFolder } from "@/lib/ipc/dialog";
import {
  analyzeCodeIntelligence,
  searchCode,
  type CodeIntelligenceReport,
  type SearchHit,
} from "@/lib/ipc/intel";

interface InsightsState {
  /** Project folder under analysis. */
  projectPath: string | null;
  /** The detector report, once analyzed. */
  report: CodeIntelligenceReport | null;
  /** Whether the detectors are running. */
  analyzing: boolean;
  /** Current search query. */
  query: string;
  /** Search results. */
  hits: SearchHit[];
  /** Whether a search is running. */
  searching: boolean;
  /** Last error message. */
  error: string | null;

  /** Opens the folder picker and sets the project. */
  pickProject: () => Promise<void>;
  /** Runs the cyclic/dead/duplication detectors. */
  analyze: () => Promise<void>;
  /** Sets the search query without running it. */
  setQuery: (query: string) => void;
  /** Runs the code search for the current query. */
  search: () => Promise<void>;
}

function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/** Code Intelligence (Insights) store. */
export const useInsightsStore = create<InsightsState>((set, get) => ({
  projectPath: null,
  report: null,
  analyzing: false,
  query: "",
  hits: [],
  searching: false,
  error: null,

  pickProject: async () => {
    const path = await pickFolder().catch(() => null);
    if (path) {
      set({ projectPath: path, report: null, hits: [], error: null });
    }
  },

  analyze: async () => {
    const path = get().projectPath;
    if (!path) {
      set({ error: "Choose a project folder first." });
      return;
    }
    set({ analyzing: true, error: null });
    try {
      const report = await analyzeCodeIntelligence(path);
      set({ report, analyzing: false });
    } catch (error) {
      set({ analyzing: false, error: messageOf(error) });
    }
  },

  setQuery: (query) => set({ query }),

  search: async () => {
    const path = get().projectPath;
    const query = get().query.trim();
    if (!path) {
      set({ error: "Choose a project folder first." });
      return;
    }
    if (!query) {
      set({ hits: [] });
      return;
    }
    set({ searching: true, error: null });
    try {
      const hits = await searchCode(path, query);
      set({ hits, searching: false });
    } catch (error) {
      set({ searching: false, error: messageOf(error) });
    }
  },
}));

// Dev-only debug handle for seeding the store from preview tooling. Stripped
// from production builds.
if (import.meta.env.DEV && typeof window !== "undefined") {
  (window as unknown as { __insightsStore?: typeof useInsightsStore }).__insightsStore =
    useInsightsStore;
}
