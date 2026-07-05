use serde::{Deserialize, Serialize};

/// A dependency cycle: a set of nodes that mutually depend on each other.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cycle {
    /// Node ids forming the cycle, in a deterministic order.
    pub nodes: Vec<String>,
}

/// A function that appears to be unused (best-effort, heuristic).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeadSymbol {
    /// Function name.
    pub name: String,
    /// File the function is defined in.
    pub file: String,
    /// 1-based line the function starts on.
    pub line: usize,
}

/// One location of a duplicated block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuplicationLocation {
    /// File containing the block.
    pub file: String,
    /// 1-based first line of the block.
    pub start_line: usize,
    /// 1-based last line of the block.
    pub end_line: usize,
}

/// A group of identical code blocks found in two or more places.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuplicationGroup {
    /// Number of (normalized) lines in the block.
    pub line_count: usize,
    /// Where the block occurs.
    pub occurrences: Vec<DuplicationLocation>,
}

/// A code-search result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchHit {
    /// File the hit is in.
    pub path: String,
    /// Matching symbol name, when the hit is a symbol rather than a path.
    pub symbol: Option<String>,
    /// 1-based line of the hit (the symbol's line, or 1 for a path hit).
    pub line: usize,
    /// Relevance score; higher is better.
    pub score: usize,
}

/// The deterministic code-intelligence report for a repository.
///
/// All findings are computed from the AST and architecture graphs; the dead
/// code and duplication detectors are heuristic and may have false positives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeIntelligenceReport {
    /// Dependency cycles between modules/files.
    pub cyclic_dependencies: Vec<Cycle>,
    /// Functions with no detected callers.
    pub dead_code: Vec<DeadSymbol>,
    /// Groups of duplicated code blocks.
    pub duplication: Vec<DuplicationGroup>,
}
