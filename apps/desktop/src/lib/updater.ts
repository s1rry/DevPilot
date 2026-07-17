import { ask } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

import { isTauri } from "@/lib/ipc/env";
import type { TranslationKey } from "@/lib/i18n/en";
import type { TranslationParams } from "@/lib/i18n/interpolate";

/** Translation function, matching the shape returned by `useT()`. */
type Translate = (key: TranslationKey, params?: TranslationParams) => string;

/**
 * Checks the GitHub Release updater manifest for a newer signed build and, if
 * one exists, asks the user to install it. On confirmation the update is
 * downloaded, installed and the app relaunched.
 *
 * Silent when up to date or when running outside the desktop shell. Any error
 * (offline, manifest missing) is swallowed — a failed update check must never
 * block startup.
 */
export async function checkForUpdates(t: Translate): Promise<void> {
  if (!isTauri()) {
    return;
  }
  try {
    const update = await check();
    if (!update) {
      return;
    }
    const confirmed = await ask(t("updater.available", { version: update.version }), {
      title: t("updater.title"),
      kind: "info",
      okLabel: t("updater.install"),
      cancelLabel: t("updater.later"),
    });
    if (!confirmed) {
      return;
    }
    await update.downloadAndInstall();
    await relaunch();
  } catch {
    // Update checks are best-effort; ignore failures.
  }
}
