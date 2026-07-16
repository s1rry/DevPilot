/**
 * Pure string interpolation for translations. Kept free of any store or DOM
 * dependency so it is trivially unit-testable.
 */

/** Named substitution values for a translated string, e.g. `{ count: 3 }`. */
export type TranslationParams = Record<string, string | number>;

/** Replaces `{name}` placeholders in a template with the given values. */
export function interpolate(template: string, params?: TranslationParams): string {
  if (!params) {
    return template;
  }
  return template.replace(/\{(\w+)\}/g, (match, name: string) =>
    name in params ? String(params[name]) : match,
  );
}
