use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::language::Language;

/// A function or method definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionDef {
    /// Function name.
    pub name: String,
    /// 1-based line where the function starts.
    pub start_line: usize,
    /// 1-based line where the function ends.
    pub end_line: usize,
    /// Whether the function is declared `async`.
    pub is_async: bool,
}

/// A class or, by analogy, a Rust struct/enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassDef {
    /// Class name.
    pub name: String,
    /// 1-based line where the class starts.
    pub start_line: usize,
    /// 1-based line where the class ends.
    pub end_line: usize,
    /// Method names declared in the class body.
    pub methods: Vec<String>,
}

/// An interface or, by analogy, a Rust trait.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfaceDef {
    /// Interface name.
    pub name: String,
    /// 1-based line where the interface starts.
    pub start_line: usize,
}

/// An import declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDecl {
    /// Imported module or path (e.g. `react`, `std::sync`).
    pub source: String,
    /// 1-based line of the declaration.
    pub line: usize,
}

/// What kind of symbol an export refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportKind {
    /// An exported function.
    Function,
    /// An exported class / struct / enum.
    Class,
    /// An exported interface / trait.
    Interface,
    /// An exported constant or variable.
    Value,
    /// A default export, or anything not otherwise classified.
    Other,
}

/// An export declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportDecl {
    /// Exported name, when one is available.
    pub name: String,
    /// Kind of the exported symbol.
    pub kind: ExportKind,
    /// 1-based line of the declaration.
    pub line: usize,
}

/// The structural model of one parsed source file — the "internal JSON model"
/// produced by the AST analyzer. Everything is derived from the syntax tree;
/// there is no semantic resolution and no AI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FileAst {
    /// Path relative to the repository root.
    pub path: PathBuf,
    /// Language the file was parsed as.
    pub language: Language,
    /// Top-level and nested function definitions.
    pub functions: Vec<FunctionDef>,
    /// Class / struct / enum definitions.
    pub classes: Vec<ClassDef>,
    /// Interface / trait definitions.
    pub interfaces: Vec<InterfaceDef>,
    /// Import declarations.
    pub imports: Vec<ImportDecl>,
    /// Export declarations.
    pub exports: Vec<ExportDecl>,
}
