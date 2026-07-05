import { create } from "zustand";

import { pickFolder } from "@/lib/ipc/dialog";
import { analyzeArchitecture, type ArchitectureModel, type Graph } from "@/lib/ipc/architecture";

/** Which of the four graphs is shown. */
export type GraphKind = "dependency" | "module" | "folder" | "call";

interface ArchitectureState {
  /** Project folder under analysis. */
  projectPath: string | null;
  /** The analyzed model, or null. */
  model: ArchitectureModel | null;
  /** Whether analysis is running. */
  analyzing: boolean;
  /** Which graph is active. */
  activeGraph: GraphKind;
  /** Last error message. */
  error: string | null;

  /** Opens the folder picker and sets the project. */
  pickProject: () => Promise<void>;
  /** Builds the architecture graphs. */
  analyze: () => Promise<void>;
  /** Switches the active graph. */
  setGraph: (kind: GraphKind) => void;
}

function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/** Returns the graph of the given kind from a model. */
export function graphOf(model: ArchitectureModel, kind: GraphKind): Graph {
  switch (kind) {
    case "dependency":
      return model.dependency_graph;
    case "module":
      return model.module_graph;
    case "folder":
      return model.folder_graph;
    case "call":
      return model.call_graph;
  }
}

/** Architecture graph store. */
export const useArchitectureStore = create<ArchitectureState>((set, get) => ({
  projectPath: null,
  model: null,
  analyzing: false,
  activeGraph: "dependency",
  error: null,

  pickProject: async () => {
    const path = await pickFolder().catch(() => null);
    if (path) {
      set({ projectPath: path, model: null, error: null });
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
      const model = await analyzeArchitecture(path);
      set({ model, analyzing: false });
    } catch (error) {
      set({ analyzing: false, error: messageOf(error) });
    }
  },

  setGraph: (kind) => set({ activeGraph: kind }),
}));

// Dev-only debug handle for seeding the store from preview tooling. Stripped
// from production builds.
if (import.meta.env.DEV && typeof window !== "undefined") {
  (window as unknown as { __architectureStore?: typeof useArchitectureStore }).__architectureStore =
    useArchitectureStore;
}
