//! Symbol extraction from tree-sitter syntax trees into [`FileAst`] pieces.
//!
//! Two extractors: [`rust`] for Rust, [`ecma`] for JavaScript and
//! TypeScript. Both walk the tree and match on node kinds — no semantic
//! analysis.

use std::collections::HashMap;

use devpilot_core::entities::{
    ClassDef, ExportDecl, ExportKind, FileAst, FunctionDef, ImportDecl, InterfaceDef,
};
use tree_sitter::Node;

/// Text of a node, or an empty string if it is not valid UTF-8.
fn text<'a>(node: Node, src: &'a [u8]) -> &'a str {
    node.utf8_text(src).unwrap_or("")
}

/// Name of a node from its `name` field.
fn name_of(node: Node, src: &[u8]) -> Option<String> {
    node.child_by_field_name("name")
        .map(|child| text(child, src).to_string())
}

/// 1-based start line of a node.
fn start_line(node: Node) -> usize {
    node.start_position().row + 1
}

/// 1-based end line of a node.
fn end_line(node: Node) -> usize {
    node.end_position().row + 1
}

/// Whether a node has a direct named child of the given kind.
fn has_child_kind(node: Node, kind: &str) -> bool {
    named_children(node)
        .iter()
        .any(|child| child.kind() == kind)
}

// --- Rust --------------------------------------------------------------

/// Extracts Rust symbols into `ast`.
pub fn rust(root: Node, src: &[u8], ast: &mut FileAst) {
    let impls = rust_impl_methods(root, src);
    rust_visit(root, src, ast, &impls);
}

/// Collects, per implemented type, the method names declared in its `impl`
/// blocks, so struct/enum entries can list their methods.
fn rust_impl_methods(root: Node, src: &[u8]) -> HashMap<String, Vec<String>> {
    let mut methods: HashMap<String, Vec<String>> = HashMap::new();

    let mut stack = named_children(root);
    while let Some(node) = stack.pop() {
        if node.kind() == "impl_item" {
            if let Some(type_node) = node.child_by_field_name("type") {
                // Strip generics: `Foo<T>` -> `Foo`.
                let type_name = text(type_node, src)
                    .split('<')
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                for name in impl_function_names(node, src) {
                    methods.entry(type_name.clone()).or_default().push(name);
                }
            }
        }
        stack.extend(named_children(node));
    }
    methods
}

/// Collects the named children of a node into an owned vector, freeing the
/// borrow of the temporary cursor.
fn named_children(node: Node) -> Vec<Node> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).collect()
}

/// Names of the `function_item`s declared anywhere within an `impl` block.
fn impl_function_names(impl_node: Node, src: &[u8]) -> Vec<String> {
    let mut names = Vec::new();
    let mut stack = named_children(impl_node);
    while let Some(node) = stack.pop() {
        if node.kind() == "function_item" {
            if let Some(name) = name_of(node, src) {
                names.push(name);
            }
        } else {
            stack.extend(named_children(node));
        }
    }
    names
}

/// Whether a Rust item node is `async`.
fn rust_is_async(node: Node, src: &[u8]) -> bool {
    named_children(node)
        .iter()
        .any(|child| child.kind() == "function_modifiers" && text(*child, src).contains("async"))
}

/// Records an export for a `pub` Rust item.
fn rust_export(node: Node, src: &[u8], kind: ExportKind, ast: &mut FileAst) {
    if has_child_kind(node, "visibility_modifier") {
        if let Some(name) = name_of(node, src) {
            ast.exports.push(ExportDecl {
                name,
                kind,
                line: start_line(node),
            });
        }
    }
}

/// Recursively visits Rust nodes, filling `ast`.
fn rust_visit(node: Node, src: &[u8], ast: &mut FileAst, impls: &HashMap<String, Vec<String>>) {
    match node.kind() {
        "function_item" => {
            if let Some(name) = name_of(node, src) {
                ast.functions.push(FunctionDef {
                    name,
                    start_line: start_line(node),
                    end_line: end_line(node),
                    is_async: rust_is_async(node, src),
                });
            }
            rust_export(node, src, ExportKind::Function, ast);
        }
        "struct_item" | "enum_item" | "union_item" => {
            if let Some(name) = name_of(node, src) {
                let methods = impls.get(&name).cloned().unwrap_or_default();
                ast.classes.push(ClassDef {
                    name,
                    start_line: start_line(node),
                    end_line: end_line(node),
                    methods,
                });
            }
            rust_export(node, src, ExportKind::Class, ast);
        }
        "trait_item" => {
            if let Some(name) = name_of(node, src) {
                ast.interfaces.push(InterfaceDef {
                    name,
                    start_line: start_line(node),
                });
            }
            rust_export(node, src, ExportKind::Interface, ast);
        }
        "use_declaration" => {
            let raw = text(node, src);
            let source = raw
                .split_once("use ")
                .map(|(_, rest)| rest)
                .unwrap_or(raw)
                .trim()
                .trim_end_matches(';')
                .trim()
                .to_string();
            ast.imports.push(ImportDecl {
                source,
                line: start_line(node),
            });
        }
        "const_item" | "static_item" => {
            rust_export(node, src, ExportKind::Value, ast);
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        rust_visit(child, src, ast, impls);
    }
}

// --- JavaScript / TypeScript ------------------------------------------

/// Extracts JavaScript/TypeScript symbols into `ast`.
pub fn ecma(root: Node, src: &[u8], ast: &mut FileAst) {
    ecma_visit(root, src, ast);
}

/// Whether an EcmaScript node carries the `async` keyword.
fn ecma_is_async(node: Node) -> bool {
    (0..node.child_count()).any(|index| {
        node.child(index)
            .map(|child| child.kind() == "async")
            .unwrap_or(false)
    })
}

/// Collects the method names of a class body.
fn class_methods(class_node: Node, src: &[u8]) -> Vec<String> {
    let Some(body) = class_node.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|child| child.kind() == "method_definition")
        .filter_map(|method| name_of(method, src))
        .collect()
}

/// Records arrow/function expressions bound in a `const`/`let`/`var`.
fn ecma_bound_functions(node: Node, src: &[u8], ast: &mut FileAst) {
    let mut cursor = node.walk();
    for declarator in node.named_children(&mut cursor) {
        if declarator.kind() != "variable_declarator" {
            continue;
        }
        let Some(value) = declarator.child_by_field_name("value") else {
            continue;
        };
        if matches!(
            value.kind(),
            "arrow_function" | "function" | "function_expression"
        ) {
            if let Some(name) = name_of(declarator, src) {
                ast.functions.push(FunctionDef {
                    name,
                    start_line: start_line(declarator),
                    end_line: end_line(value),
                    is_async: ecma_is_async(value),
                });
            }
        }
    }
}

/// Records the export(s) declared by an `export` statement.
fn ecma_export(node: Node, src: &[u8], ast: &mut FileAst) {
    let line = start_line(node);

    if let Some(decl) = node.child_by_field_name("declaration") {
        match decl.kind() {
            "function_declaration" | "generator_function_declaration" => {
                if let Some(name) = name_of(decl, src) {
                    ast.exports.push(ExportDecl {
                        name,
                        kind: ExportKind::Function,
                        line,
                    });
                }
            }
            "class_declaration" => {
                if let Some(name) = name_of(decl, src) {
                    ast.exports.push(ExportDecl {
                        name,
                        kind: ExportKind::Class,
                        line,
                    });
                }
            }
            "interface_declaration" => {
                if let Some(name) = name_of(decl, src) {
                    ast.exports.push(ExportDecl {
                        name,
                        kind: ExportKind::Interface,
                        line,
                    });
                }
            }
            "lexical_declaration" | "variable_declaration" => {
                let mut cursor = decl.walk();
                for declarator in decl.named_children(&mut cursor) {
                    if declarator.kind() == "variable_declarator" {
                        if let Some(name) = name_of(declarator, src) {
                            ast.exports.push(ExportDecl {
                                name,
                                kind: ExportKind::Value,
                                line,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
        return;
    }

    // `export { a, b }`
    let clause = named_children(node)
        .into_iter()
        .find(|c| c.kind() == "export_clause");
    if let Some(clause) = clause {
        for specifier in named_children(clause) {
            if specifier.kind() == "export_specifier" {
                if let Some(name) = name_of(specifier, src) {
                    ast.exports.push(ExportDecl {
                        name,
                        kind: ExportKind::Other,
                        line,
                    });
                }
            }
        }
        return;
    }

    // `export default ...`
    if (0..node.child_count()).any(|i| {
        node.child(i)
            .map(|c| c.kind() == "default")
            .unwrap_or(false)
    }) {
        ast.exports.push(ExportDecl {
            name: "default".to_string(),
            kind: ExportKind::Other,
            line,
        });
    }
}

/// Recursively visits EcmaScript nodes, filling `ast`.
fn ecma_visit(node: Node, src: &[u8], ast: &mut FileAst) {
    match node.kind() {
        "function_declaration" | "generator_function_declaration" => {
            if let Some(name) = name_of(node, src) {
                ast.functions.push(FunctionDef {
                    name,
                    start_line: start_line(node),
                    end_line: end_line(node),
                    is_async: ecma_is_async(node),
                });
            }
        }
        "class_declaration" => {
            if let Some(name) = name_of(node, src) {
                ast.classes.push(ClassDef {
                    name,
                    start_line: start_line(node),
                    end_line: end_line(node),
                    methods: class_methods(node, src),
                });
            }
            // Do not descend into the class body; methods are captured above.
            return;
        }
        "interface_declaration" => {
            if let Some(name) = name_of(node, src) {
                ast.interfaces.push(InterfaceDef {
                    name,
                    start_line: start_line(node),
                });
            }
        }
        "import_statement" => {
            if let Some(source) = node.child_by_field_name("source") {
                let trimmed = text(source, src).trim_matches(['"', '\'']).to_string();
                ast.imports.push(ImportDecl {
                    source: trimmed,
                    line: start_line(node),
                });
            }
        }
        "lexical_declaration" | "variable_declaration" => {
            ecma_bound_functions(node, src, ast);
        }
        "export_statement" => {
            ecma_export(node, src, ast);
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        ecma_visit(child, src, ast);
    }
}
