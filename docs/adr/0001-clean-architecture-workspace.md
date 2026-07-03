# ADR-0001: Clean Architecture in a Cargo workspace

Date: 2026-07-03
Status: accepted

## Context

DevPilot is a desktop repository analyzer (Tauri + React frontend, Rust backend) that must stay contributor-friendly and modular as it grows: multiple LLM providers, multiple analysis backends, three operating systems. Early structural decisions are the hardest to change later.

## Decision

1. **Cargo workspace of small crates** instead of a single binary: `devpilot-core` (domain), `devpilot-analysis`, `devpilot-git`, `devpilot-ai`, `devpilot-storage` (adapters), `devpilot-testing` (shared test doubles), `apps/desktop/src-tauri` (composition root).
2. **Dependency direction points inward.** Core defines traits (ports); adapter crates implement them; only the composition root sees all crates.
3. **Explicit dependency injection** via trait objects wired in the Tauri app. No DI framework.
4. **The Tauri layer holds no business logic.** Commands deserialize, call a use case, map errors.
5. **Frontend mirrors the layering**: `features/` slices, `shared/`, and `lib/ipc` as the single boundary to Rust.
6. **Stack**: Tauri 2, React 18 + TypeScript strict, TailwindCSS 4, Vite, Zustand, tokio, tree-sitter, git2, rusqlite, thin reqwest adapters for LLM providers.

## Alternatives considered

- **Single crate**: rejected — slow compiles, no compiler-enforced boundaries.
- **Multiple repositories**: rejected — coordination overhead for a small OSS project.
- **DI framework (e.g. shaku)**: rejected — magic and learning curve vs. one explicit wiring file.
- **LLM SDKs / langchain-style frameworks**: rejected — uncontrolled dependency surface; a thin trait plus reqwest covers our needs.
- **Electron**: rejected — binary size and memory footprint conflict with the "fast, lightweight" goal.

## Consequences

- Every module is replaceable behind a trait; adding an LLM provider touches one adapter file plus wiring.
- Contributors get compiler-enforced architecture: a forbidden dependency does not compile.
- Cost: more manifests to maintain and occasional boilerplate mapping between layers. Accepted as the price of long-term modularity.
