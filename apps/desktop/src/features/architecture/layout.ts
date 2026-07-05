import type { Graph } from "@/lib/ipc/architecture";

/** A laid-out node position. */
export interface Position {
  x: number;
  y: number;
}

/** Result of a layout: positions keyed by node id, and the bounding box. */
export interface Layout {
  positions: Map<string, Position>;
  width: number;
  height: number;
}

/** Maximum nodes laid out; larger graphs are truncated for responsiveness. */
export const MAX_LAYOUT_NODES = 250;

/** Simulation parameters, tuned for an abstract ~1000px canvas. */
const ITERATIONS = 300;
const AREA = 1000;
const REPULSION = 9_000;
const SPRING = 0.02;
const SPRING_LENGTH = 90;
const GRAVITY = 0.015;
const DAMPING = 0.85;

/**
 * Computes a deterministic force-directed layout for a graph (a compact
 * Fruchterman–Reingold variant). Runs a fixed number of iterations so the
 * result is stable and cheap; no animation and no dependencies.
 *
 * Graphs larger than {@link MAX_LAYOUT_NODES} are truncated, keeping the most
 * connected nodes.
 */
export function computeLayout(graph: Graph): Layout {
  const nodes = selectNodes(graph);
  const ids = nodes.map((node) => node.id);
  const idSet = new Set(ids);
  const edges = graph.edges.filter((edge) => idSet.has(edge.from) && idSet.has(edge.to));

  const count = ids.length;
  const x = new Float64Array(count);
  const y = new Float64Array(count);
  const vx = new Float64Array(count);
  const vy = new Float64Array(count);
  const index = new Map<string, number>();

  // Deterministic ring initialization.
  ids.forEach((id, i) => {
    const angle = (i / Math.max(1, count)) * Math.PI * 2;
    x[i] = Math.cos(angle) * AREA * 0.3;
    y[i] = Math.sin(angle) * AREA * 0.3;
    index.set(id, i);
  });

  const edgePairs = edges
    .map((edge) => [index.get(edge.from)!, index.get(edge.to)!] as const)
    .filter(([a, b]) => a !== b);

  for (let iter = 0; iter < ITERATIONS; iter += 1) {
    // Repulsion between every pair.
    for (let i = 0; i < count; i += 1) {
      for (let j = i + 1; j < count; j += 1) {
        let dx = x[i] - x[j];
        let dy = y[i] - y[j];
        let distSq = dx * dx + dy * dy;
        if (distSq < 0.01) {
          dx = (i - j) * 0.01 + 0.01;
          dy = 0.01;
          distSq = dx * dx + dy * dy;
        }
        const force = REPULSION / distSq;
        const dist = Math.sqrt(distSq);
        const fx = (dx / dist) * force;
        const fy = (dy / dist) * force;
        vx[i] += fx;
        vy[i] += fy;
        vx[j] -= fx;
        vy[j] -= fy;
      }
    }

    // Spring attraction along edges.
    for (const [a, b] of edgePairs) {
      const dx = x[b] - x[a];
      const dy = y[b] - y[a];
      const dist = Math.sqrt(dx * dx + dy * dy) || 0.01;
      const force = (dist - SPRING_LENGTH) * SPRING;
      const fx = (dx / dist) * force;
      const fy = (dy / dist) * force;
      vx[a] += fx;
      vy[a] += fy;
      vx[b] -= fx;
      vy[b] -= fy;
    }

    // Gravity to center and integration.
    for (let i = 0; i < count; i += 1) {
      vx[i] = (vx[i] - x[i] * GRAVITY) * DAMPING;
      vy[i] = (vy[i] - y[i] * GRAVITY) * DAMPING;
      x[i] += vx[i];
      y[i] += vy[i];
    }
  }

  // Normalize into a padded positive bounding box.
  const positions = new Map<string, Position>();
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (let i = 0; i < count; i += 1) {
    minX = Math.min(minX, x[i]);
    minY = Math.min(minY, y[i]);
    maxX = Math.max(maxX, x[i]);
    maxY = Math.max(maxY, y[i]);
  }
  const pad = 60;
  if (!Number.isFinite(minX)) {
    return { positions, width: AREA, height: AREA };
  }
  ids.forEach((id, i) => {
    positions.set(id, { x: x[i] - minX + pad, y: y[i] - minY + pad });
  });
  return {
    positions,
    width: maxX - minX + pad * 2,
    height: maxY - minY + pad * 2,
  };
}

/** Picks up to {@link MAX_LAYOUT_NODES} nodes, preferring the most connected. */
function selectNodes(graph: Graph) {
  if (graph.nodes.length <= MAX_LAYOUT_NODES) {
    return graph.nodes;
  }
  const degree = new Map<string, number>();
  for (const edge of graph.edges) {
    degree.set(edge.from, (degree.get(edge.from) ?? 0) + 1);
    degree.set(edge.to, (degree.get(edge.to) ?? 0) + 1);
  }
  return [...graph.nodes]
    .sort((a, b) => (degree.get(b.id) ?? 0) - (degree.get(a.id) ?? 0))
    .slice(0, MAX_LAYOUT_NODES);
}
