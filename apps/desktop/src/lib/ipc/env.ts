/**
 * Whether the app is running inside the Tauri runtime (the desktop shell) as
 * opposed to a plain browser (e.g. the Vite dev preview).
 *
 * Feature code uses this to avoid invoking backend commands when there is no
 * backend to answer them.
 */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}
