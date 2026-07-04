import { open } from "@tauri-apps/plugin-dialog";

/**
 * Opens the native folder picker and returns the chosen absolute path, or
 * `null` if the user cancelled.
 *
 * This is the one place the UI invokes the dialog plugin; feature code calls
 * this wrapper rather than the plugin directly.
 */
export async function pickFolder(): Promise<string | null> {
  const selected = await open({ directory: true, multiple: false });
  // `open` returns `string | string[] | null`; with `multiple: false` it is
  // a single path or null.
  return typeof selected === "string" ? selected : null;
}
