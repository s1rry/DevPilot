import type { LucideIcon } from "lucide-react";

interface IconButtonProps {
  /** Icon to render. */
  icon: LucideIcon;
  /** Accessible label; also used as the native tooltip. */
  label: string;
  /** Click handler. */
  onClick?: () => void;
  /** Renders the button in its active/selected state. */
  active?: boolean;
}

/**
 * A square, icon-only button used across the shell (top bar, sidebar rail).
 * Includes an accessible label and visible focus ring.
 */
export function IconButton({ icon: Icon, label, onClick, active = false }: IconButtonProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      title={label}
      aria-label={label}
      aria-pressed={active}
      className={`flex h-9 w-9 items-center justify-center rounded-md outline-none transition-colors focus-visible:ring-2 focus-visible:ring-accent ${
        active
          ? "bg-elevated text-fg"
          : "text-muted hover:bg-elevated hover:text-fg"
      }`}
    >
      <Icon size={18} strokeWidth={2} />
    </button>
  );
}
