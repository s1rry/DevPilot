import { BarChart3, FolderGit2, Lightbulb, MessageSquare, Network, Settings } from "lucide-react";
import type { LucideIcon } from "lucide-react";

import type { TranslationKey } from "@/lib/i18n/en";
import type { ViewId } from "@/lib/store/navigation";

/** A primary navigation entry shown in the sidebar. */
export interface NavItem {
  /** Stable view identifier. */
  id: ViewId;
  /** Translation key for the label shown in the sidebar and content header. */
  label: TranslationKey;
  /** Icon rendered in the sidebar rail. */
  icon: LucideIcon;
  /** Translation key for the one-line hint shown in the view's empty state. */
  hint: TranslationKey;
  /** Roadmap phase that will fill this view with real functionality. */
  phase: string;
}

/**
 * The five primary views, one per feature slice. This is the single source
 * of truth for navigation; both the sidebar and the content area read it.
 */
export const NAV_ITEMS: readonly NavItem[] = [
  {
    id: "repository",
    label: "nav.repository.label",
    icon: FolderGit2,
    hint: "nav.repository.hint",
    phase: "Phase 2",
  },
  {
    id: "analysis",
    label: "nav.analysis.label",
    icon: BarChart3,
    hint: "nav.analysis.hint",
    phase: "Phase 2",
  },
  {
    id: "architecture",
    label: "nav.architecture.label",
    icon: Network,
    hint: "nav.architecture.hint",
    phase: "Phase 5",
  },
  {
    id: "ai-chat",
    label: "nav.ai-chat.label",
    icon: MessageSquare,
    hint: "nav.ai-chat.hint",
    phase: "Phase 4",
  },
  {
    id: "insights",
    label: "nav.insights.label",
    icon: Lightbulb,
    hint: "nav.insights.hint",
    phase: "Phase 5",
  },
  {
    id: "settings",
    label: "nav.settings.label",
    icon: Settings,
    hint: "nav.settings.hint",
    phase: "Phase 3",
  },
];

/** Looks up a navigation item by its view id. */
export function navItem(id: ViewId): NavItem {
  const found = NAV_ITEMS.find((item) => item.id === id);
  if (!found) {
    throw new Error(`Unknown view id: ${id}`);
  }
  return found;
}
