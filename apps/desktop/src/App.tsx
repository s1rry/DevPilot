import { useEffect } from "react";

import { ContentArea } from "@/shared/ui/ContentArea";
import { ResizeHandle } from "@/shared/ui/ResizeHandle";
import { Sidebar } from "@/shared/ui/Sidebar";
import { StatusBar } from "@/shared/ui/StatusBar";
import { TopBar } from "@/shared/ui/TopBar";
import { useMediaQuery } from "@/shared/hooks/useMediaQuery";
import { useResizablePanel } from "@/shared/hooks/useResizablePanel";
import { useNavigationStore } from "@/lib/store/navigation";

/** Below this window width the sidebar collapses to an icon rail. */
const NARROW_QUERY = "(max-width: 900px)";

/**
 * Root component and layout composition of DevPilot.
 *
 * Structure: a top bar, a middle row of resizable sidebar plus content, and a
 * status bar. On narrow windows the sidebar auto-collapses. This component
 * wires the shell together and contains no domain logic.
 */
export default function App() {
  const collapsed = useNavigationStore((state) => state.sidebarCollapsed);
  const setSidebarCollapsed = useNavigationStore((state) => state.setSidebarCollapsed);
  const isNarrow = useMediaQuery(NARROW_QUERY);

  const { width, isDragging, handleProps } = useResizablePanel({
    initialWidth: 240,
    minWidth: 180,
    maxWidth: 420,
  });

  // Collapse the sidebar automatically on narrow windows.
  useEffect(() => {
    setSidebarCollapsed(isNarrow);
  }, [isNarrow, setSidebarCollapsed]);

  return (
    <div className="flex h-full flex-col bg-canvas text-fg">
      <TopBar />

      <div className="flex min-h-0 flex-1">
        <Sidebar width={width} collapsed={collapsed} />
        {!collapsed && <ResizeHandle isDragging={isDragging} handleProps={handleProps} />}
        <ContentArea />
      </div>

      <StatusBar />
    </div>
  );
}
