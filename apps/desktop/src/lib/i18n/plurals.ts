import { interpolate } from "./interpolate";

/**
 * Count-based strings that need grammatical plural agreement. English has two
 * forms (one / other); Russian has three (one / few / many). We pick the right
 * form with `Intl.PluralRules` instead of hardcoding suffix logic per language.
 */

/** Keys of count-based (pluralized) strings. */
export type PluralKey = "commits" | "copies" | "lines";

/**
 * Plural forms for one string. `one`/`other` are required (they cover English);
 * `few`/`many` are optional and used by languages like Russian. Each form is a
 * template that may contain `{count}`.
 */
export interface PluralForms {
  one: string;
  few?: string;
  many?: string;
  other: string;
}

type PluralDictionary = Record<PluralKey, PluralForms>;

/** English plural forms. */
export const enPlurals: PluralDictionary = {
  commits: { one: "{count} commit", other: "{count} commits" },
  copies: { one: "{count} copy", other: "{count} copies" },
  lines: { one: "{count} line", other: "{count} lines" },
};

/** Russian plural forms (one / few / many). */
export const ruPlurals: PluralDictionary = {
  commits: { one: "{count} коммит", few: "{count} коммита", many: "{count} коммитов", other: "{count} коммитов" },
  copies: { one: "{count} копия", few: "{count} копии", many: "{count} копий", other: "{count} копий" },
  lines: { one: "{count} строка", few: "{count} строки", many: "{count} строк", other: "{count} строк" },
};

const PLURALS: Record<string, PluralDictionary> = { en: enPlurals, ru: ruPlurals };
const LOCALES: Record<string, string> = { en: "en-US", ru: "ru-RU" };

/**
 * Resolves the correct plural form for `count` in `language` and fills in
 * `{count}`. Falls back to English and to the `other` form when a specific
 * category is missing.
 */
export function plural(language: string, key: PluralKey, count: number): string {
  const forms = (PLURALS[language] ?? enPlurals)[key];
  const category = new Intl.PluralRules(LOCALES[language] ?? "en-US").select(count);
  const template = forms[category as keyof PluralForms] ?? forms.other;
  return interpolate(template, { count });
}
