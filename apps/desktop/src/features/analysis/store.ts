import { create } from "zustand";

import { pickFolder } from "@/lib/ipc/dialog";
import { scanFolder, type ScanReport } from "@/lib/ipc/scan";

/** What the analysis view is currently doing. */
export type AnalysisStatus = "idle" | "scanning";

interface AnalysisState {
  /** Current async status. */
  status: AnalysisStatus;
  /** The most recent scan report, if any. */
  report: ScanReport | null;
  /** Path of the folder the report describes. */
  scannedPath: string | null;
  /** Last error message, if the most recent scan failed. */
  error: string | null;

  /** Opens the folder picker, then scans the chosen folder. */
  scanDialog: () => Promise<void>;
  /** Scans a specific folder path. */
  scan: (path: string) => Promise<void>;
}

/** Extracts a readable message from an unknown thrown value. */
function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/**
 * Repository Scanner store. Holds the current scan report and status, and
 * delegates scanning to the typed IPC layer.
 */
export const useAnalysisStore = create<AnalysisState>((set) => ({
  status: "idle",
  report: null,
  scannedPath: null,
  error: null,

  scanDialog: async () => {
    const path = await pickFolder().catch(() => null);
    if (!path) {
      return; // user cancelled
    }
    await useAnalysisStore.getState().scan(path);
  },

  scan: async (path: string) => {
    set({ status: "scanning", error: null });
    try {
      const report = await scanFolder(path);
      set({ report, scannedPath: path, status: "idle" });
    } catch (error) {
      set({ status: "idle", error: messageOf(error) });
    }
  },
}));

// Dev-only debug handle for seeding the store from preview tooling. Stripped
// from production builds.
if (import.meta.env.DEV && typeof window !== "undefined") {
  (window as unknown as { __analysisStore?: typeof useAnalysisStore }).__analysisStore =
    useAnalysisStore;
}
