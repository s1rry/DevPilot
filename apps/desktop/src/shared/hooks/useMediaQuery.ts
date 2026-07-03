import { useEffect, useState } from "react";

/**
 * Tracks whether a CSS media query currently matches.
 *
 * Used for responsive behavior, for example collapsing the sidebar on narrow
 * windows. Returns `false` during server-side rendering or before the first
 * effect runs.
 *
 * @param query - A media query string, e.g. `"(max-width: 900px)"`.
 */
export function useMediaQuery(query: string): boolean {
  const [matches, setMatches] = useState(() =>
    typeof window !== "undefined" ? window.matchMedia(query).matches : false,
  );

  useEffect(() => {
    const mediaQueryList = window.matchMedia(query);
    const onChange = (event: MediaQueryListEvent) => setMatches(event.matches);

    setMatches(mediaQueryList.matches);
    mediaQueryList.addEventListener("change", onChange);
    return () => mediaQueryList.removeEventListener("change", onChange);
  }, [query]);

  return matches;
}
