import { create } from "zustand";
import { persist } from "zustand/middleware";

import { en, type Dictionary, type TranslationKey } from "@/lib/i18n/en";
import { ru } from "@/lib/i18n/ru";
import { interpolate, type TranslationParams } from "@/lib/i18n/interpolate";
import { plural, type PluralKey } from "@/lib/i18n/plurals";

/** Supported interface languages. */
export type Language = "ru" | "en";

/** Dictionary lookup by language code. */
const DICTIONARIES: Record<Language, Dictionary> = { ru, en };

interface I18nState {
  /** Currently active language. */
  language: Language;
  /** Switches between Russian and English. */
  toggleLanguage: () => void;
  /** Sets a specific language. */
  setLanguage: (language: Language) => void;
}

/**
 * Reflects the active language on the document root (`lang` attribute) so the
 * browser and assistive tech know the page language. Safe to call outside React.
 */
function applyLanguage(language: Language): void {
  document.documentElement.setAttribute("lang", language);
}

/**
 * Language store, persisted to localStorage under `devpilot-language`.
 *
 * The default is Russian. The persisted value is re-applied to the DOM on
 * rehydration so a reload keeps the chosen language.
 */
export const useI18nStore = create<I18nState>()(
  persist(
    (set, get) => ({
      language: "ru",
      toggleLanguage: () => {
        const next: Language = get().language === "ru" ? "en" : "ru";
        applyLanguage(next);
        set({ language: next });
      },
      setLanguage: (language) => {
        applyLanguage(language);
        set({ language });
      },
    }),
    {
      name: "devpilot-language",
      onRehydrateStorage: () => (state) => {
        applyLanguage(state?.language ?? "ru");
      },
    },
  ),
);

/**
 * Returns a translation function `t(key, params?)` bound to the active
 * language.
 *
 * Keys are checked at compile time against the reference dictionary, so a
 * typo or a missing key is a build error rather than a runtime surprise.
 * Optional `params` fill `{name}` placeholders, e.g.
 * `t("scan.topLevelDirs", { count: 3 })`. Use inside components:
 * `const t = useT(); ... t("nav.settings.label")`.
 */
export function useT(): (key: TranslationKey, params?: TranslationParams) => string {
  const language = useI18nStore((state) => state.language);
  const dictionary = DICTIONARIES[language] ?? en;
  return (key: TranslationKey, params?: TranslationParams) =>
    interpolate(dictionary[key] ?? en[key], params);
}

/**
 * Returns a plural function `tn(key, count)` bound to the active language.
 *
 * Picks the grammatically correct form for `count` (e.g. Russian
 * one/few/many) via `Intl.PluralRules`. Use for count-based strings:
 * `const tn = useTn(); ... tn("commits", n)`.
 */
export function useTn(): (key: PluralKey, count: number) => string {
  const language = useI18nStore((state) => state.language);
  return (key: PluralKey, count: number) => plural(language, key, count);
}
