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

## [0.1.4] - 2026-07-04

### Added

- AI Chat backend (sub-step; no UI yet):
  - `ChatWithRepository` use case: gathers repository context (summary plus
    files relevant to the question), fits it and the conversation into an
    approximate token budget, and streams the selected provider's reply.
  - Pure context builder in `devpilot-core::chat` (relevance selection,
    system prompt, budget-aware trimming) with tests.
  - AI settings: `AiSettings`/`ProviderKind` entities, `SettingsStore` port,
    `JsonSettingsStore` (API keys in a JSON file), `MockSettingsStore`.
  - `build_provider` factory in `devpilot-ai`.
  - Tauri commands wired via DI: `chat` (streams tokens through an IPC
    `Channel`), `get_ai_settings`, `set_ai_settings`.

## [0.1.3] - 2026-07-04

### Added

- LLM provider abstraction (no business logic): one `LlmProvider` port, four
  adapters.
  - `LlmProvider` trait in core with provider-neutral `ChatRequest`/
    `ChatMessage`/`ModelInfo`, token streaming (`TokenStream`) and a typed
    `LlmError` (auth, rate limit, context length, network).
  - `devpilot-ai` adapters over raw `reqwest` (rustls TLS), no SDKs:
    `OllamaProvider` (local, no key — the reference), `ClaudeProvider`,
    `OpenAiProvider`, `GeminiProvider`. API keys are redacted from `Debug`.
  - `MockLlmProvider` in `devpilot-testing`; wiremock adapter tests covering
    SSE and NDJSON streaming, model listing and error mapping.



### Added

- Architecture Engine (no AI): builds four graphs from AST data and exports
  them as JSON.
  - `ArchitectureModel` in core with a generic `Graph` (typed nodes/edges):
    **folder** (directory containment), **dependency** (file imports),
    **module** (directory-level dependencies) and **call** (function calls).
  - Pure, deterministic builders in `devpilot-core::architecture`. Import and
    call resolution are name-based heuristics, documented as such.
  - `AnalyzeArchitecture` use case: parses every supported file in a
    repository and builds the model; unreadable/unparsable files are skipped.
  - AST analyzer now captures per-function call sites (`FunctionDef.calls`)
    for Rust and TypeScript/JavaScript.
  - `analyze_architecture` and `export_architecture` (writes JSON to disk)
    Tauri commands via DI; typed `lib/ipc/architecture.ts`.

## [0.1.1] - 2026-07-04

### Added

- AST analyzer (no AI, internal model): parses a source file into a
  structural `FileAst` — functions, classes, interfaces, imports, exports.
  - `devpilot-analysis` now implements the `CodeAnalyzer` port on tree-sitter
    for Rust and TypeScript/JavaScript (incl. TSX/JSX grammars). Pure
    syntactic extraction; a parse failure is a per-file error, not a panic.
  - The `CodeAnalyzer` port is refocused from metrics onto AST parsing
    (`parse -> FileAst`); metrics types remain for a later step.
  - `parse_file` Tauri command wired through DI; typed `lib/ipc/ast.ts`.

### Changed

- `Language` now derives `Default` (`Unknown`).

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
