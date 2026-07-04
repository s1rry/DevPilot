import { create } from "zustand";

import {
  cloneRepository,
  listRecentProjects,
  openFolder,
  removeRecentProject,
  type ProjectMetadata,
  type RecentProject,
} from "@/lib/ipc/repository";
import { pickFolder } from "@/lib/ipc/dialog";

/** What the view is currently doing. */
export type RepositoryStatus = "idle" | "opening" | "cloning";

interface RepositoryState {
  /** Current async status, for spinners and disabling inputs. */
  status: RepositoryStatus;
  /** Metadata of the currently opened project, if any. */
  metadata: ProjectMetadata | null;
  /** Recent projects, most recent first. */
  recent: RecentProject[];
  /** Last error message, if the most recent action failed. */
  error: string | null;

  /** Loads the recent-projects list. */
  loadRecent: () => Promise<void>;
  /** Opens the native folder picker, then opens the chosen folder. */
  openFolderDialog: () => Promise<void>;
  /** Clones a remote repository by URL and opens it. */
  clone: (url: string) => Promise<void>;
  /** Reopens a recent project by its stored path. */
  reopen: (path: string) => Promise<void>;
  /** Removes a project from the recent list. */
  remove: (id: string) => Promise<void>;
}

/** Extracts a readable message from an unknown thrown value. */
function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/**
 * Repository Manager store. Holds the opened project, the recent list and the
 * async status, and delegates every backend call to the typed IPC layer.
 */
export const useRepositoryStore = create<RepositoryState>((set, get) => ({
  status: "idle",
  metadata: null,
  recent: [],
  error: null,

  loadRecent: async () => {
    try {
      const recent = await listRecentProjects();
      set({ recent });
    } catch (error) {
      set({ error: messageOf(error) });
    }
  },

  openFolderDialog: async () => {
    const path = await pickFolder().catch(() => null);
    if (!path) {
      return; // user cancelled
    }
    set({ status: "opening", error: null });
    try {
      const metadata = await openFolder(path);
      set({ metadata, status: "idle" });
      await get().loadRecent();
    } catch (error) {
      set({ status: "idle", error: messageOf(error) });
    }
  },

  clone: async (url: string) => {
    set({ status: "cloning", error: null });
    try {
      const metadata = await cloneRepository(url);
      set({ metadata, status: "idle" });
      await get().loadRecent();
    } catch (error) {
      set({ status: "idle", error: messageOf(error) });
    }
  },

  reopen: async (path: string) => {
    set({ status: "opening", error: null });
    try {
      const metadata = await openFolder(path);
      set({ metadata, status: "idle" });
      await get().loadRecent();
    } catch (error) {
      set({ status: "idle", error: messageOf(error) });
    }
  },

  remove: async (id: string) => {
    try {
      await removeRecentProject(id);
      set({ recent: get().recent.filter((project) => project.id !== id) });
    } catch (error) {
      set({ error: messageOf(error) });
    }
  },
}));

// Dev-only debug handle for inspecting and seeding the store from the browser
// console or preview tooling. Stripped from production builds.
if (import.meta.env.DEV && typeof window !== "undefined") {
  (window as unknown as { __repositoryStore?: typeof useRepositoryStore }).__repositoryStore =
    useRepositoryStore;
}
