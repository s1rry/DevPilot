use serde::{Deserialize, Serialize};

/// Metrics of a single function or method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionMetrics {
    /// Function name as written in source.
    pub name: String,
    /// 1-based line where the function starts.
    pub start_line: usize,
    /// Number of lines the function spans.
    pub line_count: usize,
    /// McCabe cyclomatic complexity: number of branch points plus one.
    ///
    /// Branch points are `if`, loops, `match`/`case` arms, `catch`,
    /// short-circuit operators and ternaries, as defined per language in
    /// `devpilot-analysis`.
    pub cyclomatic_complexity: u32,
    /// Deepest level of nested blocks inside the function body.
    pub nesting_depth: u32,
}

/// Line and structure metrics of a single source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMetrics {
    /// Total number of lines, including blank ones.
    pub lines_total: usize,
    /// Lines containing code.
    pub lines_code: usize,
    /// Lines consisting only of comments.
    pub lines_comment: usize,
    /// Per-function metrics; empty when AST analysis is unavailable
    /// for the file's language.
    pub functions: Vec<FunctionMetrics>,
}

impl FileMetrics {
    /// Share of comment lines relative to all lines, in `0.0..=1.0`.
    ///
    /// Returns `0.0` for empty files.
    pub fn comment_ratio(&self) -> f64 {
        if self.lines_total == 0 {
            0.0
        } else {
            self.lines_comment as f64 / self.lines_total as f64
        }
    }

    /// Highest cyclomatic complexity across functions, if any were analyzed.
    pub fn max_complexity(&self) -> Option<u32> {
        self.functions
            .iter()
            .map(|function| function.cyclomatic_complexity)
            .max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metrics(lines_total: usize, lines_comment: usize) -> FileMetrics {
        FileMetrics {
            lines_total,
            lines_code: lines_total - lines_comment,
            lines_comment,
            functions: vec![],
        }
    }

    #[test]
    fn comment_ratio_is_zero_for_empty_file() {
        assert_eq!(metrics(0, 0).comment_ratio(), 0.0);
    }

    #[test]
    fn comment_ratio_is_share_of_all_lines() {
        assert_eq!(metrics(10, 4).comment_ratio(), 0.4);
    }

    #[test]
    fn max_complexity_none_without_functions() {
        assert_eq!(metrics(5, 0).max_complexity(), None);
    }

    #[test]
    fn max_complexity_picks_largest() {
        let mut m = metrics(5, 0);
        m.functions = vec![
            FunctionMetrics {
                name: "a".into(),
                start_line: 1,
                line_count: 2,
                cyclomatic_complexity: 3,
                nesting_depth: 1,
            },
            FunctionMetrics {
                name: "b".into(),
                start_line: 3,
                line_count: 2,
                cyclomatic_complexity: 7,
                nesting_depth: 2,
            },
        ];
        assert_eq!(m.max_complexity(), Some(7));
    }
}
