import { NAV_ITEMS } from "@/shared/navigation";
import { useNavigationStore } from "@/lib/store/navigation";

interface SidebarProps {
  /** Width in pixels when expanded; ignored when collapsed. */
  width: number;
  /** Whether to render the compact icon rail. */
  collapsed: boolean;
}

/** Fixed width of the collapsed icon rail, in pixels. */
const RAIL_WIDTH = 56;

/**
 * Primary navigation sidebar. Renders the five feature views as a vertical
 * list, or as an icon-only rail when collapsed. Holds no domain data; it
 * only reflects and updates navigation state.
 */
export function Sidebar({ width, collapsed }: SidebarProps) {
  const activeView = useNavigationStore((state) => state.activeView);
  const setActiveView = useNavigationStore((state) => state.setActiveView);

  return (
    <nav
      aria-label="Primary"
      style={{ width: collapsed ? RAIL_WIDTH : width }}
      className="flex shrink-0 flex-col gap-1 overflow-hidden border-r border-border bg-surface p-2"
    >
      {NAV_ITEMS.map((item) => {
        const Icon = item.icon;
        const isActive = item.id === activeView;
        return (
          <button
            key={item.id}
            type="button"
            onClick={() => setActiveView(item.id)}
            title={collapsed ? item.label : undefined}
            aria-current={isActive ? "page" : undefined}
            className={`flex items-center gap-3 rounded-md px-2.5 py-2 text-sm outline-none transition-colors focus-visible:ring-2 focus-visible:ring-accent ${
              collapsed ? "justify-center" : ""
            } ${
              isActive
                ? "bg-accent text-accent-fg"
                : "text-muted hover:bg-elevated hover:text-fg"
            }`}
          >
            <Icon size={18} strokeWidth={2} className="shrink-0" />
            {!collapsed && <span className="truncate">{item.label}</span>}
          </button>
        );
      })}
    </nav>
  );
}
