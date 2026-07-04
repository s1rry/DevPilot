import type { LucideIcon } from "lucide-react";

interface ButtonProps {
  /** Button label. */
  children: React.ReactNode;
  /** Click handler. */
  onClick?: () => void;
  /** Visual weight. */
  variant?: "primary" | "secondary";
  /** Optional leading icon. */
  icon?: LucideIcon;
  /** Disables interaction. */
  disabled?: boolean;
  /** Native button type; defaults to `button`. */
  type?: "button" | "submit";
}

/** A labeled text button with an optional leading icon. */
export function Button({
  children,
  onClick,
  variant = "secondary",
  icon: Icon,
  disabled = false,
  type = "button",
}: ButtonProps) {
  const palette =
    variant === "primary"
      ? "bg-accent text-accent-fg hover:opacity-90"
      : "border border-border text-fg hover:bg-elevated";

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      className={`flex h-9 items-center gap-2 rounded-md px-3 text-sm font-medium outline-none transition-colors focus-visible:ring-2 focus-visible:ring-accent disabled:cursor-not-allowed disabled:opacity-50 ${palette}`}
    >
      {Icon && <Icon size={16} strokeWidth={2} />}
      {children}
    </button>
  );
}
