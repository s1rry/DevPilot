import { EmptyState } from "@/shared/ui/EmptyState";
import { navItem } from "@/shared/navigation";

/**
 * Analysis view. Placeholder in this shell; metrics and structure rendering
 * arrives with the analysis feature.
 */
export function AnalysisView() {
  const item = navItem("analysis");
  return <EmptyState icon={item.icon} title={item.label} hint={item.hint} phase={item.phase} />;
}
