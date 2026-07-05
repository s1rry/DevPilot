import { useEffect, useMemo, useRef, useState } from "react";

import type { Graph, NodeKind } from "@/lib/ipc/architecture";
import { computeLayout, MAX_LAYOUT_NODES, type Position } from "@/features/architecture/layout";

/** Fill color per node kind. */
const NODE_COLOR: Record<NodeKind, string> = {
  File: "#6366f1",
  Directory: "#64748b",
  Module: "#a855f7",
  Function: "#22c55e",
  External: "#475569",
};

interface ViewBox {
  x: number;
  y: number;
  w: number;
  h: number;
}

interface GraphCanvasProps {
  /** The graph to render. */
  graph: Graph;
}

/**
 * Renders a graph as an interactive SVG: force-laid-out nodes colored by kind,
 * directed edges, pan (drag background), zoom (wheel) and node dragging.
 * Labels show for small graphs; every node has a hover tooltip.
 */
export function GraphCanvas({ graph }: GraphCanvasProps) {
  const layout = useMemo(() => computeLayout(graph), [graph]);
  const svgRef = useRef<SVGSVGElement>(null);

  const [positions, setPositions] = useState<Map<string, Position>>(layout.positions);
  const [view, setView] = useState<ViewBox>({ x: 0, y: 0, w: layout.width, h: layout.height });
  const drag = useRef<{ mode: "pan" | "node"; id?: string; x: number; y: number } | null>(null);

  // Reset when the graph (and thus layout) changes.
  useEffect(() => {
    setPositions(new Map(layout.positions));
    setView({ x: 0, y: 0, w: layout.width, h: layout.height });
  }, [layout]);

  const nodesById = useMemo(
    () => new Map(graph.nodes.map((node) => [node.id, node])),
    [graph.nodes],
  );
  const showLabels = positions.size <= 60;

  /** Converts a client point to SVG user-space coordinates. */
  const toSvg = (clientX: number, clientY: number) => {
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect) {
      return { x: 0, y: 0 };
    }
    return {
      x: view.x + ((clientX - rect.left) / rect.width) * view.w,
      y: view.y + ((clientY - rect.top) / rect.height) * view.h,
    };
  };

  const onWheel = (event: React.WheelEvent) => {
    event.preventDefault();
    const factor = event.deltaY > 0 ? 1.1 : 0.9;
    const point = toSvg(event.clientX, event.clientY);
    setView((current) => ({
      x: point.x - (point.x - current.x) * factor,
      y: point.y - (point.y - current.y) * factor,
      w: current.w * factor,
      h: current.h * factor,
    }));
  };

  const onMouseDown = (event: React.MouseEvent, id?: string) => {
    const point = toSvg(event.clientX, event.clientY);
    drag.current = id
      ? { mode: "node", id, x: point.x, y: point.y }
      : { mode: "pan", x: event.clientX, y: event.clientY };
  };

  const onMouseMove = (event: React.MouseEvent) => {
    const state = drag.current;
    if (!state) {
      return;
    }
    if (state.mode === "pan") {
      const scaleX = view.w / (svgRef.current?.getBoundingClientRect().width ?? 1);
      const scaleY = view.h / (svgRef.current?.getBoundingClientRect().height ?? 1);
      const dx = (event.clientX - state.x) * scaleX;
      const dy = (event.clientY - state.y) * scaleY;
      setView((current) => ({ ...current, x: current.x - dx, y: current.y - dy }));
      drag.current = { ...state, x: event.clientX, y: event.clientY };
    } else if (state.id) {
      const point = toSvg(event.clientX, event.clientY);
      const id = state.id;
      setPositions((current) => {
        const next = new Map(current);
        next.set(id, { x: point.x, y: point.y });
        return next;
      });
    }
  };

  const endDrag = () => {
    drag.current = null;
  };

  if (graph.nodes.length === 0) {
    return (
      <div className="flex h-full items-center justify-center text-sm text-muted">
        This graph is empty.
      </div>
    );
  }

  return (
    <div className="relative h-full w-full">
      {graph.nodes.length > MAX_LAYOUT_NODES && (
        <span className="absolute left-2 top-2 z-10 rounded bg-surface/80 px-2 py-1 text-xs text-muted">
          Showing the {MAX_LAYOUT_NODES} most connected of {graph.nodes.length} nodes
        </span>
      )}
      <svg
        ref={svgRef}
        viewBox={`${view.x} ${view.y} ${view.w} ${view.h}`}
        className="h-full w-full cursor-grab touch-none select-none active:cursor-grabbing"
        onWheel={onWheel}
        onMouseDown={(event) => onMouseDown(event)}
        onMouseMove={onMouseMove}
        onMouseUp={endDrag}
        onMouseLeave={endDrag}
      >
        <defs>
          <marker
            id="arrow"
            viewBox="0 0 10 10"
            refX="18"
            refY="5"
            markerWidth="5"
            markerHeight="5"
            orient="auto-start-reverse"
          >
            <path d="M 0 0 L 10 5 L 0 10 z" fill="var(--border)" />
          </marker>
        </defs>

        <g>
          {graph.edges.map((edge, index) => {
            const from = positions.get(edge.from);
            const to = positions.get(edge.to);
            if (!from || !to) {
              return null;
            }
            return (
              <line
                key={index}
                x1={from.x}
                y1={from.y}
                x2={to.x}
                y2={to.y}
                stroke="var(--border)"
                strokeWidth={1}
                markerEnd="url(#arrow)"
              />
            );
          })}
        </g>

        <g>
          {[...positions].map(([id, position]) => {
            const node = nodesById.get(id);
            if (!node) {
              return null;
            }
            const radius = node.kind === "External" ? 4 : 6;
            return (
              <g
                key={id}
                transform={`translate(${position.x} ${position.y})`}
                onMouseDown={(event) => {
                  event.stopPropagation();
                  onMouseDown(event, id);
                }}
                className="cursor-pointer"
              >
                <title>{node.label}</title>
                <circle r={radius} fill={NODE_COLOR[node.kind]} stroke="var(--canvas)" strokeWidth={1.5} />
                {showLabels && (
                  <text
                    x={radius + 3}
                    y={3}
                    fontSize={10}
                    fill="var(--fg)"
                    className="pointer-events-none"
                  >
                    {node.label}
                  </text>
                )}
              </g>
            );
          })}
        </g>
      </svg>
    </div>
  );
}
