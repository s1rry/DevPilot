import { invoke } from "@tauri-apps/api/core";

/**
 * Typed wrappers and types for Code Intelligence, mirroring the core
 * `CodeIntelligenceReport` and `SearchHit`.
 */

/** A dependency cycle. */
export interface Cycle {
  nodes: string[];
}

/** A function with no detected callers (heuristic). */
export interface DeadSymbol {
  name: string;
  file: string;
  line: number;
}

/** One location of a duplicated block. */
export interface DuplicationLocation {
  file: string;
  start_line: number;
  end_line: number;
}

/** A group of duplicated code blocks. */
export interface DuplicationGroup {
  line_count: number;
  occurrences: DuplicationLocation[];
}

/** The deterministic code-intelligence report. */
export interface CodeIntelligenceReport {
  cyclic_dependencies: Cycle[];
  dead_code: DeadSymbol[];
  duplication: DuplicationGroup[];
}

/** A code-search result. */
export interface SearchHit {
  path: string;
  symbol: string | null;
  line: number;
  score: number;
}

/** Runs the cyclic-dependency, dead-code and duplication detectors. */
export function analyzeCodeIntelligence(path: string): Promise<CodeIntelligenceReport> {
  return invoke<CodeIntelligenceReport>("analyze_code_intelligence", { path });
}

/** Searches the project's symbols and paths for a query. */
export function searchCode(path: string, query: string): Promise<SearchHit[]> {
  return invoke<SearchHit[]>("search_code", { path, query });
}
