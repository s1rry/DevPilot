import { BarChart3, FolderGit2, Lightbulb, MessageSquare, Network, Settings } from "lucide-react";
import type { LucideIcon } from "lucide-react";

import type { ViewId } from "@/lib/store/navigation";

/** A primary navigation entry shown in the sidebar. */
export interface NavItem {
  /** Stable view identifier. */
  id: ViewId;
  /** Label shown in the sidebar and content header. */
  label: string;
  /** Icon rendered in the sidebar rail. */
  icon: LucideIcon;
  /** One-line description shown in the empty state of the view. */
  hint: string;
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
    label: "Repository",
    icon: FolderGit2,
    hint: "Open a local folder or a GitHub URL to explore its file tree.",
    phase: "Phase 2",
  },
  {
    id: "analysis",
    label: "Analysis",
    icon: BarChart3,
    hint: "Code metrics, complexity and structure from tree-sitter.",
    phase: "Phase 2",
  },
  {
    id: "architecture",
    label: "Architecture",
    icon: Network,
    hint: "Interactive dependency, module, folder and call graphs.",
    phase: "Phase 5",
  },
  {
    id: "ai-chat",
    label: "AI Chat",
    icon: MessageSquare,
    hint: "Ask questions about the codebase with Ollama, Claude, OpenAI or Gemini.",
    phase: "Phase 4",
  },
  {
    id: "insights",
    label: "Insights",
    icon: Lightbulb,
    hint: "Reports on hotspots, risks and code quality trends.",
    phase: "Phase 5",
  },
  {
    id: "settings",
    label: "Settings",
    icon: Settings,
    hint: "AI providers, API keys and appearance.",
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
