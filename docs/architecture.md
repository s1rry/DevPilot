# DevPilot Architecture

## Principle

Clean Architecture. Dependencies point inward, to `devpilot-core`. Core knows nothing about Tauri, tree-sitter, git2, or any LLM provider.

## Module diagram

```
                ┌─────────────────────────────┐
                │     UI (React + TS)         │
                │  features / shared / lib    │
                └──────────────┬──────────────┘
                               │ typed IPC (lib/ipc only)
                ┌──────────────▼──────────────┐
                │   apps/desktop/src-tauri    │
                │  commands + DI wiring       │
                └──────────────┬──────────────┘
                               │ calls use cases
                ┌──────────────▼──────────────┐
                │       devpilot-core         │
                │  entities, use cases, ports │◄──── everything depends on it
                └──────────────▲──────────────┘
                               │ implement ports (traits)
      ┌──────────────┬────────┴──────┬──────────────┐
      │              │               │              │
┌─────┴──────┐ ┌─────┴──────┐ ┌──────┴─────┐ ┌──────┴─────┐
│ devpilot-  │ │ devpilot-  │ │ devpilot-  │ │ devpilot-  │
│ analysis   │ │    git     │ │     ai     │ │  storage   │
│ tree-sitter│ │    git2    │ │ 4 adapters │ │   SQLite   │
└────────────┘ └────────────┘ └────────────┘ └────────────┘
```

## Rules enforced by this layout

1. `devpilot-core` has no heavy dependencies.
2. Adapter crates depend only on `devpilot-core`, never on each other.
3. Business logic lives in core use cases; Tauri commands are thin wrappers.
4. Dependency injection is explicit: trait objects wired in the composition root (`src-tauri/src/di.rs`, added when the first port lands).
5. UI components never call `invoke()` directly; only `lib/ipc` talks to Rust.

## Main data flow (repository analysis)

1. User picks a repository (GitHub URL or local path).
2. UI calls a Tauri command through `lib/ipc`.
3. The command invokes the `AnalyzeRepository` use case in core.
4. The use case works through ports: `GitReader` (tree, history, churn), `CodeAnalyzer` (tree-sitter metrics), `Cache` (results keyed by commit hash).
5. Progress streams to the UI as Tauri events; results render incrementally.
6. Questions about the code go through `AskAboutCode`, which builds context from analysis results and streams the answer from the selected `LlmProvider`.

## Technology choices

| Layer | Choice | ADR |
|---|---|---|
| Shell | Tauri 2 | ADR-0001 |
| UI | React 18 + TypeScript strict + TailwindCSS 4 + Vite | ADR-0001 |
| State | Zustand | ADR-0001 |
| Core | Rust stable, tokio | ADR-0001 |
| Parsing | tree-sitter | ADR-0001 |
| Git | git2 (libgit2) | ADR-0001 |
| AI | thin reqwest adapters, no SDKs/frameworks | ADR-0001 |
| Cache | SQLite (rusqlite) | ADR-0001 |
