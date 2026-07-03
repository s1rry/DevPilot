interface ResizeHandleProps {
  /** Whether a drag is currently in progress. */
  isDragging: boolean;
  /** Props from `useResizablePanel`, spread onto the handle. */
  handleProps: React.HTMLAttributes<HTMLDivElement> & { tabIndex: number };
}

/**
 * A thin vertical drag handle between two panels. The hit area is wider than
 * the visible line so it is easy to grab; the line highlights on hover,
 * focus and while dragging.
 */
export function ResizeHandle({ isDragging, handleProps }: ResizeHandleProps) {
  return (
    <div
      {...handleProps}
      className="group relative w-1 shrink-0 cursor-col-resize outline-none"
    >
      {/* Widened invisible hit area. */}
      <span className="absolute inset-y-0 -left-1 -right-1" />
      {/* Visible line. */}
      <span
        className={`absolute inset-y-0 left-0 w-px transition-colors ${
          isDragging
            ? "bg-accent"
            : "bg-border group-hover:bg-accent group-focus-visible:bg-accent"
        }`}
      />
    </div>
  );
}
