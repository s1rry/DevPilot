import { create } from "zustand";

import {
  getAiSettings,
  setAiSettings,
  type AiSettings,
  type ProviderKind,
} from "@/lib/ipc/settings";

interface SettingsState {
  /** Current settings, or null until loaded. */
  settings: AiSettings | null;
  /** Whether a save is in progress. */
  saving: boolean;
  /** "saved" briefly after a successful save, for feedback. */
  savedAt: number | null;
  /** Last error message, if a load or save failed. */
  error: string | null;

  /** Loads settings from the backend. */
  load: () => Promise<void>;
  /** Updates fields locally without persisting. */
  update: (patch: Partial<AiSettings>) => void;
  /** Sets the API key for a provider locally. */
  setKey: (provider: ProviderKind, key: string) => void;
  /** Persists the current settings. */
  save: () => Promise<void>;
}

function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/** AI settings store. Backs the settings screen. */
export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: null,
  saving: false,
  savedAt: null,
  error: null,

  load: async () => {
    try {
      const settings = await getAiSettings();
      set({ settings, error: null });
    } catch (error) {
      set({ error: messageOf(error) });
    }
  },

  update: (patch) => {
    const current = get().settings;
    if (current) {
      set({ settings: { ...current, ...patch }, savedAt: null });
    }
  },

  setKey: (provider, key) => {
    const current = get().settings;
    if (current) {
      set({
        settings: { ...current, api_keys: { ...current.api_keys, [provider]: key } },
        savedAt: null,
      });
    }
  },

  save: async () => {
    const settings = get().settings;
    if (!settings) {
      return;
    }
    set({ saving: true, error: null });
    try {
      await setAiSettings(settings);
      set({ saving: false, savedAt: Date.now() });
    } catch (error) {
      set({ saving: false, error: messageOf(error) });
    }
  },
}));

// Dev-only debug handle for seeding the store from preview tooling. Stripped
// from production builds.
if (import.meta.env.DEV && typeof window !== "undefined") {
  (window as unknown as { __settingsStore?: typeof useSettingsStore }).__settingsStore =
    useSettingsStore;
}
