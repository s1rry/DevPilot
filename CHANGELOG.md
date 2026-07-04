# Changelog

All notable changes to DevPilot are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: `0.0.x` for fixes and small steps, `0.x` for milestone releases
(the first working repository analysis will be `0.1`).

## [0.0.6] - 2026-07-04

### Added

- Repository Manager UI, completing the feature (no AI):
  - Open a local folder via the native file picker
    (`tauri-plugin-dialog`), or clone a GitHub repository by URL with a
    busy state.
  - Recent projects list: reopen on click, remove, relative timestamps.
  - Project metadata panel: branch, commits, files, size and a language
    breakdown.
  - New `RepositoryView` feature slice with its own Zustand store; all
    backend access flows through the typed `lib/ipc` layer.

## [0.1.0] - 2026-07-04

First milestone: DevPilot can analyze a repository end to end.

### Added

- Repository Scanner (no AI): scan a project folder and get a full report.
  - New `devpilot-scan` crate: pure manifest detectors (npm, Cargo, PyPI,
    Go) split from a filesystem adapter (`FsProjectScanner`).
  - Detects languages (from the file tree), frameworks and dependencies
    (from manifests), folder structure, and git information (branch,
    commit count, last commit, top contributors).
  - New `ProjectScanner` port and `ScanRepository` use case in the core.
  - `scan_folder` Tauri command wired through DI; typed `lib/ipc/scan.ts`.
  - Analysis view: overview cards, language bars, framework chips,
    dependency list, structure and git panels.

## [0.0.5] - 2026-07-04

### Added

- Repository Manager wiring (internal, no user-facing UI yet):
  - Tauri composition root (`di.rs`) building the concrete adapters
    (`Git2Reader`, `JsonRecentProjectsStore`) into `AppState` under the app
    data directory. First real dependency injection.
  - Thin IPC commands: `open_folder`, `clone_repository`,
    `list_recent_projects`, `remove_recent_project`.
  - Typed frontend IPC layer (`lib/ipc/repository.ts`) mirroring the core
    entities; the only place the UI talks to Rust for repositories.

## [0.0.4] - 2026-07-03

### Added

- Repository Manager backend (no UI, no AI):
  - `devpilot-git`: libgit2-backed `GitReader` — open a local repository,
    clone a remote one, read the file tree, history, per-file churn, file
    contents, current branch and commit count. Blocking git work runs off
    the async runtime.
  - `devpilot-core`: `ProjectMetadata` and `RecentProject` entities, a
    `RecentProjectsStore` port, and use cases `OpenProject`,
    `ListRecentProjects`, `RemoveRecentProject`. File tree gains size and
    per-language counting.
  - `devpilot-storage`: `JsonRecentProjectsStore` — a capped, atomically
    written JSON list in the app data directory.
  - `devpilot-testing`: `MockRecentProjectsStore` plus use-case and adapter
    tests (git on real temp repositories, storage on real files).

## [0.0.3] - 2026-07-03

### Added

- Application shell UI (no business logic): top bar, collapsible left
  sidebar with the five feature views, content area and status bar.
- Resizable sidebar with a keyboard-accessible drag handle (no external
  panel library).
- Dark and light themes via runtime CSS variables, toggle in the top bar,
  persisted to localStorage; dark is the default.
- Responsive layout: the sidebar auto-collapses to an icon rail below 900px.
- Feature-slice scaffolding: `RepositoryView`, `AnalysisView`,
  `AiChatView`, `InsightsView`, `SettingsView`, each an honest placeholder.

## [0.0.2] - 2026-07-03

### Added

- `devpilot-core`: domain entities (repository, file tree, languages,
  metrics, history, analysis results and progress), four ports
  (`GitReader`, `CodeAnalyzer`, `AnalysisCache`, `ProgressReporter`) and
  typed errors per port (ADR-0002).
- `devpilot-testing`: hand-written configurable mocks for all four ports,
  consistent repository fixtures, and behavior tests for the mocks.

## [0.0.1] - 2026-07-03

### Added

- Project skeleton: Cargo workspace with six crates (`devpilot-core`,
  `devpilot-analysis`, `devpilot-git`, `devpilot-ai`, `devpilot-storage`,
  `devpilot-testing`) following Clean Architecture (ADR-0001).
- Tauri 2 desktop shell with React 18, TypeScript (strict), TailwindCSS 4
  and Vite; verified builds on macOS.
- App icons, MIT license, contributor guide, architecture docs and ADR log.
- CI: format, lint and tests for Rust on Linux/macOS/Windows; frontend
  type-check and build.
