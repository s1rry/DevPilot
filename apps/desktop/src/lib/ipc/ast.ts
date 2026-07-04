import { invoke } from "@tauri-apps/api/core";

import type { Language } from "@/lib/ipc/repository";

/**
 * Typed wrapper and types for the AST parse command. Types mirror the
 * `devpilot-core` `FileAst` model as it serializes over IPC.
 */

/** A function or method definition. */
export interface FunctionDef {
  name: string;
  start_line: number;
  end_line: number;
  is_async: boolean;
}

/** A class, struct or enum definition. */
export interface ClassDef {
  name: string;
  start_line: number;
  end_line: number;
  methods: string[];
}

/** An interface or trait definition. */
export interface InterfaceDef {
  name: string;
  start_line: number;
}

/** An import declaration. */
export interface ImportDecl {
  source: string;
  line: number;
}

/** Kind of an exported symbol. */
export type ExportKind = "Function" | "Class" | "Interface" | "Value" | "Other";

/** An export declaration. */
export interface ExportDecl {
  name: string;
  kind: ExportKind;
  line: number;
}

/** The structural model of one parsed source file. */
export interface FileAst {
  path: string;
  language: Language;
  functions: FunctionDef[];
  classes: ClassDef[];
  interfaces: InterfaceDef[];
  imports: ImportDecl[];
  exports: ExportDecl[];
}

/** Parses one source file into its AST model. */
export function parseFile(path: string): Promise<FileAst> {
  return invoke<FileAst>("parse_file", { path });
}
