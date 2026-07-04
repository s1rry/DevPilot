import { invoke } from "@tauri-apps/api/core";

/**
 * Typed wrappers and types for the Architecture Engine commands. Types mirror
 * the `devpilot-core` graph model as it serializes over IPC.
 */

/** Kind of a graph node. */
export type NodeKind = "File" | "Directory" | "Module" | "Function" | "External";

/** A node in an architecture graph. */
export interface GraphNode {
  id: string;
  label: string;
  kind: NodeKind;
}

/** Kind of a graph edge. */
export type EdgeKind = "Contains" | "Imports" | "DependsOn" | "Calls";

/** A directed edge between two nodes. */
export interface GraphEdge {
  from: string;
  to: string;
  kind: EdgeKind;
}

/** A directed graph. */
export interface Graph {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

/** The complete architecture model: four related graphs. */
export interface ArchitectureModel {
  folder_graph: Graph;
  dependency_graph: Graph;
  module_graph: Graph;
  call_graph: Graph;
}

/** Analyzes a project's architecture into its four graphs. */
export function analyzeArchitecture(path: string): Promise<ArchitectureModel> {
  return invoke<ArchitectureModel>("analyze_architecture", { path });
}

/** Analyzes and writes the architecture model to `outPath` as JSON. */
export function exportArchitecture(path: string, outPath: string): Promise<string> {
  return invoke<string>("export_architecture", { path, outPath });
}
