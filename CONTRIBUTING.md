# Contributing to DevPilot

Thanks for your interest. The project is in early development, which is the best time to get involved.

## Setup

1. Install [Rust (stable)](https://rustup.rs), [Node.js 20+](https://nodejs.org), [pnpm](https://pnpm.io).
2. Install [Tauri system dependencies](https://tauri.app/start/prerequisites/) for your OS.
3. Run:

```sh
cd apps/desktop
pnpm install
pnpm tauri dev
```

## Where code goes

The one rule: **dependencies point inward, to `devpilot-core`.**

- Domain logic (entities, traits, use cases) → `crates/devpilot-core`
- Implementations of core traits → the matching adapter crate (`devpilot-analysis`, `devpilot-git`, `devpilot-ai`, `devpilot-storage`)
- Tauri commands and dependency wiring → `apps/desktop/src-tauri` (thin layer only, no business logic)
- UI → `apps/desktop/src`, one folder per feature under `features/`; all calls to Rust go through `lib/ipc`

Adapter crates never import each other. Significant decisions are recorded in [docs/adr](docs/adr).

## Before opening a PR

- `cargo fmt --all` and `cargo clippy --workspace -- -D warnings` pass.
- `cargo test --workspace` passes.
- New public items have doc comments.
- Behavior changes come with tests.

## Questions

Open a GitHub issue or discussion.
