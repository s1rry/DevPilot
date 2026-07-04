import { invoke } from "@tauri-apps/api/core";

import type { LanguageStat } from "@/lib/ipc/repository";

/**
 * Typed wrappers and types for the Repository Scanner command. Types mirror
 * the `devpilot-core` scan entities as they serialize over IPC.
 */

/** Package ecosystem of a dependency. */
export type Ecosystem = "Npm" | "Cargo" | "PyPI" | "Go";

/** A declared dependency. */
export interface Dependency {
  name: string;
  version: string | null;
  ecosystem: Ecosystem;
}

/** Role a framework plays in the project. */
export type FrameworkCategory = "Frontend" | "Backend" | "Fullstack" | "Desktop";

/** A detected framework. */
export interface Framework {
  name: string;
  category: FrameworkCategory;
  source: string;
}

/** Folder structure summary. */
export interface FolderSummary {
  total_files: number;
  total_directories: number;
  top_level_dirs: string[];
  notable: string[];
}

/** One commit, as serialized by `CommitInfo`. */
export interface CommitInfo {
  hash: string;
  author_name: string;
  author_email: string;
  timestamp: number;
  summary: string;
}

/** Contribution stats of one author. */
export interface AuthorStats {
  name: string;
  email: string;
  commit_count: number;
}

/** Git facts about the scanned repository. */
export interface GitSummary {
  branch: string;
  head: string;
  commit_count: number;
  last_commit: CommitInfo | null;
  contributors: AuthorStats[];
}

/** The complete result of scanning a repository. */
export interface ScanReport {
  languages: LanguageStat[];
  frameworks: Framework[];
  dependencies: Dependency[];
  structure: FolderSummary;
  git: GitSummary;
}

/** Scans a local project folder into a full report. */
export function scanFolder(path: string): Promise<ScanReport> {
  return invoke<ScanReport>("scan_folder", { path });
}
