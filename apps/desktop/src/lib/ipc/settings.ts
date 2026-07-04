import { invoke } from "@tauri-apps/api/core";

/** Typed wrappers and types for AI settings, mirroring `AiSettings`. */

/** The selectable providers. */
export type ProviderKind = "Ollama" | "Claude" | "OpenAI" | "Gemini";

/** All providers, in display order. */
export const PROVIDERS: ProviderKind[] = ["Ollama", "Claude", "OpenAI", "Gemini"];

/** User AI settings. */
export interface AiSettings {
  provider: ProviderKind;
  model: string;
  /** API key per provider; absent for providers not configured. */
  api_keys: Partial<Record<ProviderKind, string>>;
}

/** Whether a provider requires an API key. */
export function needsApiKey(provider: ProviderKind): boolean {
  return provider !== "Ollama";
}

/** Loads the current AI settings (defaults when none saved). */
export function getAiSettings(): Promise<AiSettings> {
  return invoke<AiSettings>("get_ai_settings");
}

/** Persists AI settings. */
export function setAiSettings(settings: AiSettings): Promise<void> {
  return invoke<void>("set_ai_settings", { settings });
}
