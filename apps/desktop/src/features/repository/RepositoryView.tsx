import { EmptyState } from "@/shared/ui/EmptyState";
import { navItem } from "@/shared/navigation";

/**
 * Repository view. In this shell it is a placeholder; opening repositories
 * and rendering the file tree arrives in a later roadmap phase.
 */
export function RepositoryView() {
  const item = navItem("repository");
  return <EmptyState icon={item.icon} title={item.label} hint={item.hint} phase={item.phase} />;
}
