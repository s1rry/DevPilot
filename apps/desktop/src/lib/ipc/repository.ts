import { invoke } from "@tauri-apps/api/core";

/**
 * Typed wrappers around the Repository Manager Tauri commands.
 *
 * This module is the only place in the UI that talks to Rust for repository
 * operations. The types mirror the `devpilot-core` entities exactly as they
 * serialize over IPC (snake_case fields, externally tagged enums).
 */

/** A language DevPilot can detect, as serialized by `Language`. */
export type Language = "Rust" | "TypeScript" | "JavaScript" | "Python" | "Go" | "Unknown";

/** Number of files written in one language. */
export interface LanguageStat {
  language: Language;
  file_count: number;
}

/** Descriptive metadata of an opened project. */
export interface ProjectMetadata {
  name: string;
  local_path: string;
  branch: string;
  head: string;
  commit_count: number;
  file_count: number;
  total_size_bytes: number;
  languages: LanguageStat[];
}

/** Where a project came from, matching the `RepositorySource` enum. */
export type RepositorySource = { LocalPath: string } | { RemoteUrl: string };

/** An entry in the recent-projects list. */
export interface RecentProject {
  id: string;
  name: string;
  source: RepositorySource;
  local_path: string;
  last_opened: number;
}

/** Opens a local folder as a project. */
export function openFolder(path: string): Promise<ProjectMetadata> {
  return invoke<ProjectMetadata>("open_folder", { path });
}

/** Clones a remote repository and opens it. */
export function cloneRepository(url: string): Promise<ProjectMetadata> {
  return invoke<ProjectMetadata>("clone_repository", { url });
}

/** Returns the recent-projects list, most recently opened first. */
export function listRecentProjects(): Promise<RecentProject[]> {
  return invoke<RecentProject[]>("list_recent_projects");
}

/** Removes one project from the recent-projects list. */
export function removeRecentProject(id: string): Promise<void> {
  return invoke<void>("remove_recent_project", { id });
}
