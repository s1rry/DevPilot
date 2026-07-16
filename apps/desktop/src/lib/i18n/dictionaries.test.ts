import { describe, expect, it } from "vitest";

import { en } from "./en";
import { ru } from "./ru";
import { interpolate } from "./interpolate";
import { plural } from "./plurals";

/** Extracts the set of `{placeholder}` names used in a template. */
function placeholders(template: string): Set<string> {
  const names = new Set<string>();
  for (const match of template.matchAll(/\{(\w+)\}/g)) {
    names.add(match[1]);
  }
  return names;
}

describe("dictionaries", () => {
  it("have exactly the same keys in both languages", () => {
    // TypeScript already enforces this via the `Dictionary` type, but a
    // runtime check guards against any future drift and documents intent.
    expect(Object.keys(ru).sort()).toEqual(Object.keys(en).sort());
  });

  it("use the same placeholders in every translated value", () => {
    // The compiler cannot see inside string values, so a forgotten `{count}`
    // in a translation would slip through. This catches that class of bug.
    for (const key of Object.keys(en) as (keyof typeof en)[]) {
      const enPlaceholders = [...placeholders(en[key])].sort();
      const ruPlaceholders = [...placeholders(ru[key])].sort();
      expect(ruPlaceholders, `placeholders mismatch for "${key}"`).toEqual(enPlaceholders);
    }
  });

  it("have no empty translations", () => {
    for (const key of Object.keys(ru) as (keyof typeof ru)[]) {
      expect(ru[key].trim().length, `empty translation for "${key}"`).toBeGreaterThan(0);
    }
  });
});

describe("interpolate", () => {
  it("returns the template unchanged when no params are given", () => {
    expect(interpolate("Hello world")).toBe("Hello world");
  });

  it("substitutes named placeholders", () => {
    expect(interpolate("{count} files", { count: 3 })).toBe("3 files");
    expect(interpolate("API-ключ {provider}", { provider: "openai" })).toBe("API-ключ openai");
  });

  it("substitutes multiple and repeated placeholders", () => {
    expect(interpolate("{a}+{b}={a}{b}", { a: 1, b: 2 })).toBe("1+2=12");
  });

  it("leaves unknown placeholders untouched", () => {
    expect(interpolate("{known} {unknown}", { known: "ok" })).toBe("ok {unknown}");
  });
});

describe("plural", () => {
  it("uses English one/other forms", () => {
    expect(plural("en", "commits", 1)).toBe("1 commit");
    expect(plural("en", "commits", 5)).toBe("5 commits");
  });

  it("uses Russian one/few/many forms", () => {
    expect(plural("ru", "commits", 1)).toBe("1 коммит");
    expect(plural("ru", "commits", 2)).toBe("2 коммита");
    expect(plural("ru", "commits", 5)).toBe("5 коммитов");
    expect(plural("ru", "commits", 21)).toBe("21 коммит");
  });

  it("falls back to English for an unknown language", () => {
    expect(plural("de", "lines", 2)).toBe("2 lines");
  });
});
