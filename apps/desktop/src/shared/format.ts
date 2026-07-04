/** Formatting helpers shared across the UI. */

/** Formats a byte count as a human-readable size (e.g. `1.4 MB`). */
export function formatBytes(bytes: number): string {
  if (bytes < 1024) {
    return `${bytes} B`;
  }
  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes / 1024;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${value.toFixed(1)} ${units[unit]}`;
}

/**
 * Formats a Unix-seconds timestamp as a short relative time
 * (e.g. `just now`, `5m ago`, `3d ago`). Falls back to a date for old
 * timestamps.
 */
export function formatRelativeTime(unixSeconds: number): string {
  const deltaSeconds = Math.floor(Date.now() / 1000) - unixSeconds;
  if (deltaSeconds < 60) {
    return "just now";
  }
  const minutes = Math.floor(deltaSeconds / 60);
  if (minutes < 60) {
    return `${minutes}m ago`;
  }
  const hours = Math.floor(minutes / 60);
  if (hours < 24) {
    return `${hours}h ago`;
  }
  const days = Math.floor(hours / 24);
  if (days < 30) {
    return `${days}d ago`;
  }
  return new Date(unixSeconds * 1000).toLocaleDateString();
}
