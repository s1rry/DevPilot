import { useCallback, useEffect, useRef, useState } from "react";

interface UseResizablePanelOptions {
  /** Initial width in pixels. */
  initialWidth: number;
  /** Smallest allowed width in pixels. */
  minWidth: number;
  /** Largest allowed width in pixels. */
  maxWidth: number;
  /** Step in pixels for keyboard arrow resizing. */
  keyboardStep?: number;
}

interface UseResizablePanelResult {
  /** Current width in pixels, always within `[minWidth, maxWidth]`. */
  width: number;
  /** Whether a drag is in progress, for styling the handle. */
  isDragging: boolean;
  /** Props to spread onto the drag handle element. */
  handleProps: {
    onMouseDown: (event: React.MouseEvent) => void;
    onKeyDown: (event: React.KeyboardEvent) => void;
    role: "separator";
    tabIndex: 0;
    "aria-orientation": "vertical";
    "aria-valuenow": number;
    "aria-valuemin": number;
    "aria-valuemax": number;
  };
}

/** Clamps a value into the inclusive range `[min, max]`. */
function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

/**
 * Drives a horizontally resizable panel via a drag handle, without any
 * external dependency.
 *
 * Supports mouse dragging and keyboard resizing (left/right arrows) so the
 * handle stays accessible. Width is clamped to `[minWidth, maxWidth]`.
 */
export function useResizablePanel(options: UseResizablePanelOptions): UseResizablePanelResult {
  const { initialWidth, minWidth, maxWidth, keyboardStep = 16 } = options;
  const [width, setWidth] = useState(() => clamp(initialWidth, minWidth, maxWidth));
  const [isDragging, setIsDragging] = useState(false);
  const frame = useRef<number | null>(null);

  const onMouseDown = useCallback((event: React.MouseEvent) => {
    event.preventDefault();
    setIsDragging(true);
  }, []);

  const onKeyDown = useCallback(
    (event: React.KeyboardEvent) => {
      if (event.key === "ArrowLeft") {
        event.preventDefault();
        setWidth((current) => clamp(current - keyboardStep, minWidth, maxWidth));
      } else if (event.key === "ArrowRight") {
        event.preventDefault();
        setWidth((current) => clamp(current + keyboardStep, minWidth, maxWidth));
      }
    },
    [keyboardStep, minWidth, maxWidth],
  );

  useEffect(() => {
    if (!isDragging) {
      return;
    }

    const onMove = (event: MouseEvent) => {
      // Throttle to one update per animation frame to keep dragging smooth.
      if (frame.current !== null) {
        return;
      }
      frame.current = requestAnimationFrame(() => {
        frame.current = null;
        setWidth(clamp(event.clientX, minWidth, maxWidth));
      });
    };
    const onUp = () => setIsDragging(false);

    document.body.style.cursor = "col-resize";
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);

    return () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
      document.body.style.cursor = "";
      if (frame.current !== null) {
        cancelAnimationFrame(frame.current);
        frame.current = null;
      }
    };
  }, [isDragging, minWidth, maxWidth]);

  return {
    width,
    isDragging,
    handleProps: {
      onMouseDown,
      onKeyDown,
      role: "separator",
      tabIndex: 0,
      "aria-orientation": "vertical",
      "aria-valuenow": Math.round(width),
      "aria-valuemin": minWidth,
      "aria-valuemax": maxWidth,
    },
  };
}
