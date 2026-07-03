import { EmptyState } from "@/shared/ui/EmptyState";
import { navItem } from "@/shared/navigation";

/**
 * Insights view. Placeholder in this shell; hotspot and quality reports
 * arrive with the insights feature.
 */
export function InsightsView() {
  const item = navItem("insights");
  return <EmptyState icon={item.icon} title={item.label} hint={item.hint} phase={item.phase} />;
}
