//! # devpilot-analysis
//!
//! Code analysis adapter for DevPilot. Implements the [`CodeAnalyzer`] port
//! from `devpilot-core` on top of tree-sitter, parsing source files into the
//! structural [`FileAst`] model (functions, classes, interfaces, imports,
//! exports).
//!
//! Parsing is syntactic only — there is no semantic resolution and no AI.
//! Supported languages: Rust and TypeScript/JavaScript. Others yield
//! [`AnalysisError::UnsupportedLanguage`].
//!
//! ## Rules
//!
//! - tree-sitter types never leak out of this crate; the public API speaks
//!   `devpilot-core` entities only.
//! - A parse failure on one file is that file's error, never a panic.

use std::path::Path;

use async_trait::async_trait;
use devpilot_core::entities::{FileAst, Language, SourceFile};
use devpilot_core::errors::AnalysisError;
use devpilot_core::ports::CodeAnalyzer;

mod extract;

/// Which extraction routine a grammar uses.
#[derive(Clone, Copy)]
enum Family {
    Rust,
    /// JavaScript and TypeScript share one extractor.
    Ecma,
}

/// A tree-sitter-backed [`CodeAnalyzer`].
#[derive(Default)]
pub struct TreeSitterAnalyzer;

impl TreeSitterAnalyzer {
    /// Creates an analyzer.
    pub fn new() -> Self {
        Self
    }
}

/// Selects a grammar and extractor from a file path's extension.
///
/// Uses the extension (not just [`Language`]) so JSX and TSX get the grammar
/// variant that understands their syntax.
fn grammar_for(path: &Path) -> Option<(tree_sitter::Language, Family)> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    match extension.as_str() {
        "rs" => Some((tree_sitter_rust::LANGUAGE.into(), Family::Rust)),
        "ts" => Some((
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Family::Ecma,
        )),
        "tsx" => Some((tree_sitter_typescript::LANGUAGE_TSX.into(), Family::Ecma)),
        "js" | "jsx" | "mjs" | "cjs" => {
            Some((tree_sitter_javascript::LANGUAGE.into(), Family::Ecma))
        }
        _ => None,
    }
}

/// Parses `file` synchronously into a [`FileAst`].
fn parse_sync(file: &SourceFile, language: Language) -> Result<FileAst, AnalysisError> {
    let Some((grammar, family)) = grammar_for(&file.path) else {
        return Err(AnalysisError::UnsupportedLanguage {
            path: file.path.clone(),
        });
    };

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&grammar)
        .map_err(|error| AnalysisError::ParseFailed {
            path: file.path.clone(),
            reason: error.to_string(),
        })?;

    let tree = parser
        .parse(&file.content, None)
        .ok_or_else(|| AnalysisError::ParseFailed {
            path: file.path.clone(),
            reason: "parser returned no tree".to_string(),
        })?;

    let source = file.content.as_bytes();
    let mut ast = FileAst {
        path: file.path.clone(),
        language,
        ..FileAst::default()
    };
    match family {
        Family::Rust => extract::rust(tree.root_node(), source, &mut ast),
        Family::Ecma => extract::ecma(tree.root_node(), source, &mut ast),
    }
    Ok(ast)
}

#[async_trait]
impl CodeAnalyzer for TreeSitterAnalyzer {
    async fn parse(&self, file: &SourceFile) -> Result<FileAst, AnalysisError> {
        let file = file.clone();
        let language = self.detect_language(&file.path);
        tokio::task::spawn_blocking(move || parse_sync(&file, language))
            .await
            .map_err(|join| AnalysisError::ParseFailed {
                path: Path::new("<unknown>").to_path_buf(),
                reason: format!("parse task failed: {join}"),
            })?
    }

    fn detect_language(&self, path: &Path) -> Language {
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(Language::from_extension)
            .unwrap_or(Language::Unknown)
    }
}
