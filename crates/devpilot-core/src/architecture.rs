//! Architecture graph construction: pure, deterministic functions that turn
//! parsed [`FileAst`]s and a [`FileTree`] into an [`ArchitectureModel`].
//!
//! Import and call resolution are best-effort and name-based — there is no
//! semantic resolution. Specifically:
//!
//! - An import is resolved by matching its final path segment (after `/` or
//!   `::`) to a parsed source file's stem. Unresolved targets become
//!   `External` nodes.
//! - A call is resolved to a function of the same name, preferring one in the
//!   same file, then a unique match across the repository; ambiguous or
//!   unknown callees are dropped.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::entities::{
    ArchitectureModel, EdgeKind, FileAst, FileNode, FileTree, Graph, GraphEdge, GraphNode, NodeKind,
};

/// A list of directed `(from, to)` edges by node id.
type Edges = Vec<(String, String)>;

/// Builds the full architecture model from parsed files and the file tree.
pub fn build(asts: &[FileAst], tree: &FileTree) -> ArchitectureModel {
    let (internal, external) = resolved_imports(asts);
    ArchitectureModel {
        folder_graph: folder_graph(tree),
        dependency_graph: dependency_graph(asts, &internal, &external),
        module_graph: module_graph(&internal),
        call_graph: call_graph(asts),
    }
}

/// Normalizes a path to a `/`-separated string for use as a node id.
fn path_str(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Parent directory of a `/`-separated path, or `"."` for the root.
fn dir_of(path: &str) -> String {
    match path.rfind('/') {
        Some(index) => path[..index].to_string(),
        None => ".".to_string(),
    }
}

/// Final segment of an import source, splitting on `/` and `::`.
fn last_segment(source: &str) -> &str {
    source
        .split(['/', ':'])
        .rfind(|part| !part.is_empty())
        .unwrap_or(source)
}

// --- Folder graph ------------------------------------------------------

/// Builds the directory containment graph.
fn folder_graph(tree: &FileTree) -> Graph {
    let mut graph = Graph::default();
    folder_visit(&tree.root, &mut graph);
    sort_graph(&mut graph);
    graph
}

/// Adds a directory node and edges to its child directories, recursively.
fn folder_visit(node: &FileNode, graph: &mut Graph) {
    let FileNode::Directory { path, children } = node else {
        return;
    };
    let id = {
        let raw = path_str(path);
        if raw.is_empty() {
            ".".to_string()
        } else {
            raw
        }
    };
    let label = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(".")
        .to_string();
    push_node(graph, &id, &label, NodeKind::Directory);

    for child in children {
        if let FileNode::Directory {
            path: child_path, ..
        } = child
        {
            let child_id = path_str(child_path);
            graph.edges.push(GraphEdge {
                from: id.clone(),
                to: child_id,
                kind: EdgeKind::Contains,
            });
            folder_visit(child, graph);
        }
    }
}

// --- Import resolution -------------------------------------------------

/// Resolves every file's imports into internal file-to-file edges and
/// external file-to-dependency edges.
fn resolved_imports(asts: &[FileAst]) -> (Edges, Edges) {
    // Index parsed files by their stem for name-based resolution.
    let mut by_stem: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ast in asts {
        if let Some(stem) = ast.path.file_stem().and_then(|stem| stem.to_str()) {
            by_stem
                .entry(stem.to_string())
                .or_default()
                .push(path_str(&ast.path));
        }
    }

    let mut internal = Vec::new();
    let mut external = Vec::new();
    for ast in asts {
        let from = path_str(&ast.path);
        for import in &ast.imports {
            let segment = last_segment(&import.source);
            match by_stem.get(segment) {
                Some(targets) if targets.len() == 1 && targets[0] != from => {
                    internal.push((from.clone(), targets[0].clone()));
                }
                _ => external.push((from.clone(), import.source.clone())),
            }
        }
    }
    (internal, external)
}

// --- Dependency graph --------------------------------------------------

/// Builds the file-to-file import graph, with external dependency nodes.
fn dependency_graph(
    asts: &[FileAst],
    internal: &[(String, String)],
    external: &[(String, String)],
) -> Graph {
    let mut graph = Graph::default();

    for ast in asts {
        let id = path_str(&ast.path);
        let label = ast
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&id)
            .to_string();
        push_node(&mut graph, &id, &label, NodeKind::File);
    }

    for (from, to) in internal {
        graph.edges.push(GraphEdge {
            from: from.clone(),
            to: to.clone(),
            kind: EdgeKind::Imports,
        });
    }
    for (from, source) in external {
        let id = format!("ext:{source}");
        push_node(&mut graph, &id, source, NodeKind::External);
        graph.edges.push(GraphEdge {
            from: from.clone(),
            to: id,
            kind: EdgeKind::Imports,
        });
    }

    sort_graph(&mut graph);
    graph
}

// --- Module graph ------------------------------------------------------

/// Builds the directory-to-directory dependency graph by aggregating internal
/// file imports to their parent directories.
fn module_graph(internal: &[(String, String)]) -> Graph {
    let mut graph = Graph::default();
    let mut modules: BTreeSet<String> = BTreeSet::new();
    let mut edges: BTreeSet<(String, String)> = BTreeSet::new();

    for (from, to) in internal {
        let from_dir = dir_of(from);
        let to_dir = dir_of(to);
        modules.insert(from_dir.clone());
        modules.insert(to_dir.clone());
        if from_dir != to_dir {
            edges.insert((from_dir, to_dir));
        }
    }

    for module in modules {
        let label = module.rsplit('/').next().unwrap_or(&module).to_string();
        push_node(&mut graph, &module, &label, NodeKind::Module);
    }
    for (from, to) in edges {
        graph.edges.push(GraphEdge {
            from,
            to,
            kind: EdgeKind::DependsOn,
        });
    }

    sort_graph(&mut graph);
    graph
}

// --- Call graph --------------------------------------------------------

/// Builds the function-to-function call graph.
fn call_graph(asts: &[FileAst]) -> Graph {
    let mut graph = Graph::default();

    // Index every function id by its name for cross-file resolution, and each
    // file's own functions for same-file resolution.
    let mut by_name: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for ast in asts {
        let file = path_str(&ast.path);
        for function in &ast.functions {
            let id = format!("{file}::{}", function.name);
            push_node(&mut graph, &id, &function.name, NodeKind::Function);
            by_name.entry(function.name.as_str()).or_default().push(id);
        }
    }

    let mut edges: BTreeSet<(String, String)> = BTreeSet::new();
    for ast in asts {
        let file = path_str(&ast.path);
        for function in &ast.functions {
            let from = format!("{file}::{}", function.name);
            for callee in &function.calls {
                let same_file = format!("{file}::{callee}");
                let target = if by_name
                    .get(callee.as_str())
                    .is_some_and(|ids| ids.contains(&same_file))
                {
                    Some(same_file)
                } else {
                    match by_name.get(callee.as_str()) {
                        Some(ids) if ids.len() == 1 => Some(ids[0].clone()),
                        _ => None, // unknown or ambiguous
                    }
                };
                if let Some(to) = target {
                    edges.insert((from.clone(), to));
                }
            }
        }
    }
    for (from, to) in edges {
        graph.edges.push(GraphEdge {
            from,
            to,
            kind: EdgeKind::Calls,
        });
    }

    sort_graph(&mut graph);
    graph
}

// --- Helpers -----------------------------------------------------------

/// Adds a node if the graph does not already contain its id.
fn push_node(graph: &mut Graph, id: &str, label: &str, kind: NodeKind) {
    if graph.nodes.iter().all(|node| node.id != id) {
        graph.nodes.push(GraphNode {
            id: id.to_string(),
            label: label.to_string(),
            kind,
        });
    }
}

/// Sorts nodes and edges into a deterministic order and removes duplicate
/// edges.
fn sort_graph(graph: &mut Graph) {
    graph.nodes.sort_by(|a, b| a.id.cmp(&b.id));
    graph.edges.sort_by(|a, b| {
        a.from
            .cmp(&b.from)
            .then_with(|| a.to.cmp(&b.to))
            .then_with(|| format!("{:?}", a.kind).cmp(&format!("{:?}", b.kind)))
    });
    graph
        .edges
        .dedup_by(|a, b| a.from == b.from && a.to == b.to && a.kind == b.kind);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{FunctionDef, ImportDecl, Language};
    use std::path::PathBuf;

    fn file(path: &str) -> FileNode {
        FileNode::File {
            path: PathBuf::from(path),
            size_bytes: 1,
            language: Language::Rust,
        }
    }

    fn dir(path: &str, children: Vec<FileNode>) -> FileNode {
        FileNode::Directory {
            path: PathBuf::from(path),
            children,
        }
    }

    /// A tree with `src/a.rs` and `lib/b.rs`.
    fn tree() -> FileTree {
        FileTree {
            root: dir(
                "",
                vec![
                    dir("lib", vec![file("lib/b.rs")]),
                    dir("src", vec![file("src/a.rs")]),
                ],
            ),
        }
    }

    fn function(name: &str, calls: &[&str]) -> FunctionDef {
        FunctionDef {
            name: name.to_string(),
            start_line: 1,
            end_line: 2,
            is_async: false,
            calls: calls.iter().map(|c| c.to_string()).collect(),
        }
    }

    /// `src/a.rs` imports `b` and its `foo` calls `bar`; `lib/b.rs` defines `bar`.
    fn asts() -> Vec<FileAst> {
        vec![
            FileAst {
                path: PathBuf::from("src/a.rs"),
                language: Language::Rust,
                functions: vec![function("foo", &["bar"])],
                imports: vec![ImportDecl {
                    source: "crate::b".to_string(),
                    line: 1,
                }],
                ..FileAst::default()
            },
            FileAst {
                path: PathBuf::from("lib/b.rs"),
                language: Language::Rust,
                functions: vec![function("bar", &[])],
                ..FileAst::default()
            },
        ]
    }

    fn has_edge(graph: &Graph, from: &str, to: &str, kind: EdgeKind) -> bool {
        graph
            .edges
            .iter()
            .any(|edge| edge.from == from && edge.to == to && edge.kind == kind)
    }

    #[test]
    fn folder_graph_has_directory_containment() {
        let model = build(&asts(), &tree());
        let ids: Vec<&str> = model
            .folder_graph
            .nodes
            .iter()
            .map(|n| n.id.as_str())
            .collect();
        assert!(ids.contains(&"."));
        assert!(ids.contains(&"src"));
        assert!(ids.contains(&"lib"));
        assert!(has_edge(
            &model.folder_graph,
            ".",
            "src",
            EdgeKind::Contains
        ));
    }

    #[test]
    fn dependency_graph_resolves_import_by_stem() {
        let model = build(&asts(), &tree());
        assert!(has_edge(
            &model.dependency_graph,
            "src/a.rs",
            "lib/b.rs",
            EdgeKind::Imports
        ));
    }

    #[test]
    fn module_graph_aggregates_to_directories() {
        let model = build(&asts(), &tree());
        assert!(has_edge(
            &model.module_graph,
            "src",
            "lib",
            EdgeKind::DependsOn
        ));
    }

    #[test]
    fn call_graph_links_functions_by_name() {
        let model = build(&asts(), &tree());
        assert!(has_edge(
            &model.call_graph,
            "src/a.rs::foo",
            "lib/b.rs::bar",
            EdgeKind::Calls
        ));
    }

    #[test]
    fn unresolved_import_becomes_external() {
        let ast = FileAst {
            path: PathBuf::from("src/a.rs"),
            language: Language::Rust,
            imports: vec![ImportDecl {
                source: "serde".to_string(),
                line: 1,
            }],
            ..FileAst::default()
        };
        let model = build(&[ast], &tree());
        assert!(model
            .dependency_graph
            .nodes
            .iter()
            .any(|n| n.kind == NodeKind::External && n.label == "serde"));
    }
}
