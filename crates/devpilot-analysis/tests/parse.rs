//! Integration tests for the tree-sitter analyzer, parsing real source into
//! the `FileAst` model.

use std::path::PathBuf;

use devpilot_analysis::TreeSitterAnalyzer;
use devpilot_core::entities::{ExportKind, Language, SourceFile};
use devpilot_core::errors::AnalysisError;
use devpilot_core::ports::CodeAnalyzer;

fn source(path: &str, content: &str) -> SourceFile {
    SourceFile {
        path: PathBuf::from(path),
        content: content.to_string(),
    }
}

const RUST: &str = r#"use std::sync::Arc;

pub struct Foo {
    x: i32,
}

impl Foo {
    pub fn new() -> Self {
        Foo { x: 0 }
    }
    async fn run(&self) {}
}

pub trait Greet {
    fn hi(&self);
}

pub fn top() {}
"#;

const TS: &str = r#"import { useState } from "react";

export interface Props {
    id: number;
}

export class Widget {
    render() {}
    async load() {}
}

export function make(): number {
    return 1;
}

const handler = async () => {};
"#;

#[tokio::test]
async fn parses_rust_symbols() {
    let analyzer = TreeSitterAnalyzer::new();
    let ast = analyzer
        .parse(&source("src/lib.rs", RUST))
        .await
        .expect("parse rust");

    assert_eq!(ast.language, Language::Rust);
    assert_eq!(ast.imports.len(), 1);
    assert_eq!(ast.imports[0].source, "std::sync::Arc");

    // Functions: new, run, top (impl methods count as functions).
    let fn_names: Vec<&str> = ast.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(fn_names.contains(&"new"));
    assert!(fn_names.contains(&"run"));
    assert!(fn_names.contains(&"top"));
    assert!(
        ast.functions
            .iter()
            .find(|f| f.name == "run")
            .unwrap()
            .is_async
    );

    // Struct Foo with its impl methods attached.
    let foo = ast.classes.iter().find(|c| c.name == "Foo").expect("Foo");
    assert!(foo.methods.contains(&"new".to_string()));
    assert!(foo.methods.contains(&"run".to_string()));

    // Trait Greet as an interface.
    assert!(ast.interfaces.iter().any(|i| i.name == "Greet"));

    // Exports: pub items only.
    let exported: Vec<&str> = ast.exports.iter().map(|e| e.name.as_str()).collect();
    assert!(exported.contains(&"Foo"));
    assert!(exported.contains(&"Greet"));
    assert!(exported.contains(&"top"));
    assert!(exported.contains(&"new"));
    // `run` is private, so not exported.
    assert!(!exported.contains(&"run"));
}

#[tokio::test]
async fn parses_typescript_symbols() {
    let analyzer = TreeSitterAnalyzer::new();
    let ast = analyzer
        .parse(&source("src/App.ts", TS))
        .await
        .expect("parse ts");

    assert_eq!(ast.language, Language::TypeScript);
    assert_eq!(ast.imports[0].source, "react");

    assert!(ast.interfaces.iter().any(|i| i.name == "Props"));

    let widget = ast
        .classes
        .iter()
        .find(|c| c.name == "Widget")
        .expect("Widget");
    assert!(widget.methods.contains(&"render".to_string()));
    assert!(widget.methods.contains(&"load".to_string()));

    let fn_names: Vec<&str> = ast.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(fn_names.contains(&"make"));
    assert!(fn_names.contains(&"handler"));
    assert!(
        ast.functions
            .iter()
            .find(|f| f.name == "handler")
            .unwrap()
            .is_async
    );

    // Exports: Props (interface), Widget (class), make (function).
    assert!(ast
        .exports
        .iter()
        .any(|e| e.name == "Widget" && e.kind == ExportKind::Class));
    assert!(ast
        .exports
        .iter()
        .any(|e| e.name == "make" && e.kind == ExportKind::Function));
    assert!(ast
        .exports
        .iter()
        .any(|e| e.name == "Props" && e.kind == ExportKind::Interface));
}

#[tokio::test]
async fn unsupported_language_is_typed_error() {
    let analyzer = TreeSitterAnalyzer::new();
    let result = analyzer.parse(&source("notes.txt", "hello")).await;
    assert!(matches!(
        result,
        Err(AnalysisError::UnsupportedLanguage { .. })
    ));
}
