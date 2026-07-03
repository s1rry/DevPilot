# Changelog

All notable changes to DevPilot are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: `0.0.x` for fixes and small steps, `0.x` for milestone releases
(the first working repository analysis will be `0.1`).

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
