import { EmptyState } from "@/shared/ui/EmptyState";
import { navItem } from "@/shared/navigation";

/**
 * Settings view. Placeholder in this shell; provider configuration and
 * appearance options arrive with the settings feature.
 */
export function SettingsView() {
  const item = navItem("settings");
  return <EmptyState icon={item.icon} title={item.label} hint={item.hint} phase={item.phase} />;
}
