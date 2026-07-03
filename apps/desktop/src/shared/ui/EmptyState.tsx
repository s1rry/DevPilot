import type { LucideIcon } from "lucide-react";

interface EmptyStateProps {
  /** Icon representing the view. */
  icon: LucideIcon;
  /** Title, usually the view name. */
  title: string;
  /** One-line description of what the view will do. */
  hint: string;
  /** Roadmap phase badge, e.g. "Phase 2". */
  phase: string;
}

/**
 * Centered placeholder shown by a feature view that has no functionality
 * yet. Keeps the shell honest: every view states what it will become and
 * when, instead of showing a blank screen.
 */
export function EmptyState({ icon: Icon, title, hint, phase }: EmptyStateProps) {
  return (
    <div className="flex h-full flex-col items-center justify-center gap-3 p-8 text-center">
      <div className="flex h-14 w-14 items-center justify-center rounded-xl border border-border bg-surface text-muted">
        <Icon size={26} strokeWidth={1.75} />
      </div>
      <h2 className="text-lg font-semibold text-fg">{title}</h2>
      <p className="max-w-sm text-sm text-muted">{hint}</p>
      <span className="rounded-full border border-border px-2.5 py-0.5 text-xs text-muted">
        {phase}
      </span>
    </div>
  );
}
