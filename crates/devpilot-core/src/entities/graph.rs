use serde::{Deserialize, Serialize};

/// Kind of a graph node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// A source file.
    File,
    /// A directory.
    Directory,
    /// A module (a directory grouping of files).
    Module,
    /// A function or method.
    Function,
    /// An external dependency (unresolved import target).
    External,
}

/// A node in an architecture graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphNode {
    /// Stable, unique identifier within the graph.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// What the node represents.
    pub kind: NodeKind,
}

/// Kind of a graph edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    /// A directory contains a file or subdirectory.
    Contains,
    /// A file imports another file or an external dependency.
    Imports,
    /// A module depends on another module.
    DependsOn,
    /// A function calls another function.
    Calls,
}

/// A directed edge between two nodes, referenced by id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// What the edge represents.
    pub kind: EdgeKind,
}

/// A directed graph of nodes and edges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Graph {
    /// Nodes, in a deterministic order.
    pub nodes: Vec<GraphNode>,
    /// Edges, in a deterministic order.
    pub edges: Vec<GraphEdge>,
}

/// The complete architecture model of a repository: four related graphs
/// derived from the AST and file tree. Serializes to the exported JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchitectureModel {
    /// Directory containment graph.
    pub folder_graph: Graph,
    /// File-to-file import graph.
    pub dependency_graph: Graph,
    /// Module-to-module dependency graph (directories).
    pub module_graph: Graph,
    /// Function-to-function call graph.
    pub call_graph: Graph,
}
