//! Code-intelligence detectors: pure, deterministic functions over the AST
//! and architecture graphs.
//!
//! - [`find_cycles`] finds dependency cycles (graph SCCs of size ≥ 2, plus
//!   self-loops).
//! - [`find_dead_code`] flags functions with no detected callers that are not
//!   exported or entry points. Heuristic — dynamic dispatch, trait methods
//!   and reflection can cause false positives.
//! - [`find_duplication`] finds identical normalized line blocks across files.
//! - [`search_code`] ranks files and symbols by overlap with a query.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::entities::{
    Cycle, DeadSymbol, DuplicationGroup, DuplicationLocation, EdgeKind, FileAst, Graph, SearchHit,
};

// --- Cyclic dependencies ----------------------------------------------

/// Finds dependency cycles in a directed graph using Tarjan's SCC algorithm.
/// Returns strongly connected components of size ≥ 2, plus any self-loops.
pub fn find_cycles(graph: &Graph) -> Vec<Cycle> {
    let index_of: HashMap<&str, usize> = graph
        .nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.id.as_str(), i))
        .collect();

    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); graph.nodes.len()];
    let mut self_loops: BTreeSet<usize> = BTreeSet::new();
    for edge in &graph.edges {
        if let (Some(&from), Some(&to)) = (
            index_of.get(edge.from.as_str()),
            index_of.get(edge.to.as_str()),
        ) {
            if from == to {
                self_loops.insert(from);
            } else {
                adjacency[from].push(to);
            }
        }
    }

    let mut tarjan = Tarjan::new(&adjacency);
    let mut cycles: Vec<Cycle> = tarjan
        .run()
        .into_iter()
        .filter(|component| component.len() >= 2)
        .map(|mut component| {
            component.sort();
            Cycle {
                nodes: component
                    .into_iter()
                    .map(|i| graph.nodes[i].id.clone())
                    .collect(),
            }
        })
        .collect();

    for node in self_loops {
        cycles.push(Cycle {
            nodes: vec![graph.nodes[node].id.clone()],
        });
    }

    cycles.sort_by(|a, b| a.nodes.cmp(&b.nodes));
    cycles
}

/// Iterative Tarjan strongly-connected-components.
struct Tarjan<'a> {
    adjacency: &'a [Vec<usize>],
    index: usize,
    indices: Vec<Option<usize>>,
    low: Vec<usize>,
    on_stack: Vec<bool>,
    stack: Vec<usize>,
    components: Vec<Vec<usize>>,
}

impl<'a> Tarjan<'a> {
    fn new(adjacency: &'a [Vec<usize>]) -> Self {
        let n = adjacency.len();
        Self {
            adjacency,
            index: 0,
            indices: vec![None; n],
            low: vec![0; n],
            on_stack: vec![false; n],
            stack: Vec::new(),
            components: Vec::new(),
        }
    }

    fn run(&mut self) -> Vec<Vec<usize>> {
        for v in 0..self.adjacency.len() {
            if self.indices[v].is_none() {
                self.strong_connect(v);
            }
        }
        std::mem::take(&mut self.components)
    }

    fn strong_connect(&mut self, v: usize) {
        self.indices[v] = Some(self.index);
        self.low[v] = self.index;
        self.index += 1;
        self.stack.push(v);
        self.on_stack[v] = true;

        for &w in &self.adjacency[v].clone() {
            match self.indices[w] {
                None => {
                    self.strong_connect(w);
                    self.low[v] = self.low[v].min(self.low[w]);
                }
                Some(w_index) if self.on_stack[w] => {
                    self.low[v] = self.low[v].min(w_index);
                }
                _ => {}
            }
        }

        if self.indices[v] == Some(self.low[v]) {
            let mut component = Vec::new();
            while let Some(w) = self.stack.pop() {
                self.on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            self.components.push(component);
        }
    }
}

// --- Dead code ---------------------------------------------------------

/// Flags functions with no incoming call edges that are not exported or named
/// `main`. `call_graph` is the architecture call graph; `asts` supplies
/// exports and definition lines.
pub fn find_dead_code(call_graph: &Graph, asts: &[FileAst]) -> Vec<DeadSymbol> {
    // Exported names are considered reachable (public API).
    let exported: BTreeSet<&str> = asts
        .iter()
        .flat_map(|ast| ast.exports.iter().map(|export| export.name.as_str()))
        .collect();

    // Definition line per "file::name" id.
    let mut line_of: HashMap<String, (String, String, usize)> = HashMap::new();
    for ast in asts {
        let file = ast.path.to_string_lossy().replace('\\', "/");
        for function in &ast.functions {
            let id = format!("{file}::{}", function.name);
            line_of.insert(
                id,
                (file.clone(), function.name.clone(), function.start_line),
            );
        }
    }

    // In-degree from call edges.
    let mut incoming: BTreeMap<&str, usize> = BTreeMap::new();
    for node in &call_graph.nodes {
        incoming.entry(node.id.as_str()).or_insert(0);
    }
    for edge in &call_graph.edges {
        if edge.kind == EdgeKind::Calls {
            *incoming.entry(edge.to.as_str()).or_insert(0) += 1;
        }
    }

    let mut dead: Vec<DeadSymbol> = incoming
        .into_iter()
        .filter(|(_, count)| *count == 0)
        .filter_map(|(id, _)| line_of.get(id))
        .filter(|(_, name, _)| name != "main" && !exported.contains(name.as_str()))
        .map(|(file, name, line)| DeadSymbol {
            name: name.clone(),
            file: file.clone(),
            line: *line,
        })
        .collect();

    dead.sort_by(|a, b| a.file.cmp(&b.file).then_with(|| a.line.cmp(&b.line)));
    dead
}

// --- Duplication -------------------------------------------------------

/// Minimum block size (normalized lines) for duplication detection.
const DUPLICATION_WINDOW: usize = 6;

/// Finds identical blocks of [`DUPLICATION_WINDOW`] normalized lines across
/// the given files. Blank lines are ignored; each surviving line keeps its
/// original line number.
pub fn find_duplication(files: &[(String, String)]) -> Vec<DuplicationGroup> {
    // hash of a block -> its locations.
    let mut blocks: BTreeMap<String, Vec<DuplicationLocation>> = BTreeMap::new();

    for (file, content) in files {
        // (normalized text, original 1-based line number)
        let lines: Vec<(String, usize)> = content
            .lines()
            .enumerate()
            .map(|(i, line)| (line.split_whitespace().collect::<Vec<_>>().join(" "), i + 1))
            .filter(|(text, _)| !text.is_empty())
            .collect();

        if lines.len() < DUPLICATION_WINDOW {
            continue;
        }
        for window in lines.windows(DUPLICATION_WINDOW) {
            let key = window
                .iter()
                .map(|(text, _)| text.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            blocks.entry(key).or_default().push(DuplicationLocation {
                file: file.clone(),
                start_line: window[0].1,
                end_line: window[window.len() - 1].1,
            });
        }
    }

    let mut groups: Vec<DuplicationGroup> = blocks
        .into_values()
        .filter(|locations| locations.len() >= 2)
        .map(|occurrences| DuplicationGroup {
            line_count: DUPLICATION_WINDOW,
            occurrences,
        })
        .collect();

    groups.sort_by(|a, b| {
        b.occurrences
            .len()
            .cmp(&a.occurrences.len())
            .then_with(|| a.occurrences[0].file.cmp(&b.occurrences[0].file))
            .then_with(|| {
                a.occurrences[0]
                    .start_line
                    .cmp(&b.occurrences[0].start_line)
            })
    });
    groups
}

// --- Code search -------------------------------------------------------

/// Maximum search hits returned.
const MAX_HITS: usize = 30;

/// Whether a query `word` matches a `haystack`, by substring or by a shared
/// 4-character prefix with one of the haystack's identifier tokens (so
/// "authentication" matches `authenticate` and `auth.rs`).
fn word_matches(word: &str, haystack: &str) -> bool {
    let haystack = haystack.to_lowercase();
    if haystack.contains(word) {
        return true;
    }
    let word_prefix: String = word.chars().take(4).collect();
    if word_prefix.len() < 4 {
        return false;
    }
    haystack
        .split(|c: char| !c.is_alphanumeric())
        .filter(|token| token.len() >= 4)
        .any(|token| token.starts_with(&word_prefix))
}

/// Ranks files and their symbols by overlap with `query` words. Symbol hits
/// (functions, classes, interfaces) score higher than path-only hits.
pub fn search_code(query: &str, asts: &[FileAst]) -> Vec<SearchHit> {
    let words: Vec<String> = query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| word.len() >= 3)
        .map(|word| word.to_lowercase())
        .collect();
    if words.is_empty() {
        return Vec::new();
    }

    let count = |haystack: &str| -> usize {
        words
            .iter()
            .filter(|word| word_matches(word, haystack))
            .count()
    };

    let mut hits: Vec<SearchHit> = Vec::new();
    for ast in asts {
        let path = ast.path.to_string_lossy().replace('\\', "/");

        for function in &ast.functions {
            let score = count(&function.name) * 3 + count(&path);
            if score > 0 {
                hits.push(SearchHit {
                    path: path.clone(),
                    symbol: Some(function.name.clone()),
                    line: function.start_line,
                    score,
                });
            }
        }
        for class in &ast.classes {
            let score = count(&class.name) * 3 + count(&path);
            if score > 0 {
                hits.push(SearchHit {
                    path: path.clone(),
                    symbol: Some(class.name.clone()),
                    line: class.start_line,
                    score,
                });
            }
        }
        for interface in &ast.interfaces {
            let score = count(&interface.name) * 3 + count(&path);
            if score > 0 {
                hits.push(SearchHit {
                    path: path.clone(),
                    symbol: Some(interface.name.clone()),
                    line: interface.start_line,
                    score,
                });
            }
        }

        let path_score = count(&path);
        if path_score > 0 {
            hits.push(SearchHit {
                path: path.clone(),
                symbol: None,
                line: 1,
                score: path_score,
            });
        }
    }

    hits.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.path.cmp(&b.path))
            .then_with(|| a.line.cmp(&b.line))
    });
    hits.truncate(MAX_HITS);
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        ExportDecl, ExportKind, FunctionDef, GraphEdge, GraphNode, Language, NodeKind,
    };
    use std::path::PathBuf;

    fn node(id: &str) -> GraphNode {
        GraphNode {
            id: id.to_string(),
            label: id.to_string(),
            kind: NodeKind::Module,
        }
    }

    fn edge(from: &str, to: &str, kind: EdgeKind) -> GraphEdge {
        GraphEdge {
            from: from.to_string(),
            to: to.to_string(),
            kind,
        }
    }

    #[test]
    fn find_cycles_detects_a_two_node_cycle() {
        let graph = Graph {
            nodes: vec![node("a"), node("b"), node("c")],
            edges: vec![
                edge("a", "b", EdgeKind::DependsOn),
                edge("b", "a", EdgeKind::DependsOn),
                edge("b", "c", EdgeKind::DependsOn),
            ],
        };
        let cycles = find_cycles(&graph);
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].nodes, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn find_cycles_ignores_acyclic_graph() {
        let graph = Graph {
            nodes: vec![node("a"), node("b")],
            edges: vec![edge("a", "b", EdgeKind::DependsOn)],
        };
        assert!(find_cycles(&graph).is_empty());
    }

    fn function(name: &str, line: usize) -> FunctionDef {
        FunctionDef {
            name: name.to_string(),
            start_line: line,
            end_line: line + 1,
            is_async: false,
            calls: vec![],
        }
    }

    #[test]
    fn find_dead_code_flags_uncalled_private_function() {
        let ast = FileAst {
            path: PathBuf::from("src/lib.rs"),
            language: Language::Rust,
            functions: vec![
                function("used", 1),
                function("unused", 5),
                function("main", 9),
            ],
            exports: vec![ExportDecl {
                name: "used".into(),
                kind: ExportKind::Function,
                line: 1,
            }],
            ..FileAst::default()
        };
        let call_graph = Graph {
            nodes: vec![
                node("src/lib.rs::used"),
                node("src/lib.rs::unused"),
                node("src/lib.rs::main"),
            ],
            edges: vec![edge(
                "src/lib.rs::main",
                "src/lib.rs::used",
                EdgeKind::Calls,
            )],
        };

        let dead = find_dead_code(&call_graph, &[ast]);
        // `used` is exported, `main` is an entry point; only `unused` is dead.
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0].name, "unused");
        assert_eq!(dead[0].line, 5);
    }

    #[test]
    fn find_duplication_detects_repeated_block() {
        let block = "let a = 1;\nlet b = 2;\nlet c = 3;\nlet d = 4;\nlet e = 5;\nlet f = 6;\n";
        let files = vec![
            ("a.rs".to_string(), block.to_string()),
            ("b.rs".to_string(), format!("// header\n{block}")),
        ];
        let groups = find_duplication(&files);
        assert!(!groups.is_empty());
        assert_eq!(groups[0].occurrences.len(), 2);
    }

    #[test]
    fn search_code_ranks_symbol_matches() {
        let ast = FileAst {
            path: PathBuf::from("src/auth.rs"),
            language: Language::Rust,
            functions: vec![function("authenticate", 10)],
            ..FileAst::default()
        };
        let hits = search_code("where is authentication", &[ast]);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].symbol.as_deref(), Some("authenticate"));
        assert_eq!(hits[0].line, 10);
    }
}
