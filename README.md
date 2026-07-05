<p align="center">
  <img src="docs/assets/banner.svg" alt="DevPilot — AI-powered repository analyzer" width="100%" />
</p>

<p align="center">
  <a href="https://github.com/s1rry/devpilot/actions/workflows/ci.yml"><img src="https://github.com/s1rry/devpilot/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT" /></a>
  <img src="https://img.shields.io/badge/Tauri-2-24C8DB.svg" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Rust-stable-orange.svg" alt="Rust" />
</p>

**DevPilot** is a desktop app that helps you understand any codebase — open a repository and explore its structure, dependencies and quality, then ask an AI questions about the code. It runs on Windows, macOS and Linux, and works fully offline with a local model.

> **Status:** actively developed in the open. Build it from source today (see [Getting started](#getting-started)); packaged releases are on the way.

## Features

- **Repository Manager** — open a local folder or clone a GitHub repo; recent projects; branch, commit and language metadata.
- **Repository Scanner** — detects languages, frameworks and dependencies (npm, Cargo, PyPI, Go), folder structure and git contributors.
- **AST analyzer** — tree-sitter parsing of Rust and TypeScript/JavaScript into a structural model (functions, classes, interfaces, imports, exports).
- **Architecture graphs** — interactive dependency, module, folder and call graphs with pan, zoom and drag.
- **AI Chat** — repository-aware, streaming chat about your code with **Ollama** (local, no key), **Claude**, **OpenAI** or **Gemini**. Markdown and highlighted code blocks.
- **Code Intelligence** — find cyclic dependencies, dead code and duplication, and search "where is authentication?" across symbols and paths.

All analysis is deterministic and local; the AI is optional and provider-agnostic. API keys stay on your machine.

## Getting started

Prerequisites: [Rust (stable)](https://rustup.rs), [Node.js 20+](https://nodejs.org), [pnpm](https://pnpm.io), and the [Tauri system dependencies](https://tauri.app/start/prerequisites/) for your OS.

```sh
git clone https://github.com/s1rry/devpilot.git
cd devpilot/apps/desktop
pnpm install
pnpm tauri dev
```

To use AI Chat, open **Settings** and pick a provider. **Ollama** works locally with no API key — install [Ollama](https://ollama.com) and pull a model (e.g. `ollama pull llama3`). For Claude, OpenAI or Gemini, paste your API key.

## Architecture

A Cargo workspace following Clean Architecture: every dependency points inward to `devpilot-core`, which defines the domain and the ports (traits) that adapter crates implement. See [ADR-0001](docs/adr/0001-clean-architecture-workspace.md).

| Path | Purpose |
|---|---|
| `crates/devpilot-core` | Domain: entities, ports, use cases, pure detectors (graphs, code intelligence). |
| `crates/devpilot-git` | Repository reading via libgit2: open, clone, tree, history, churn. |
| `crates/devpilot-analysis` | tree-sitter parsing into the AST model. |
| `crates/devpilot-scan` | Manifest detection: languages, frameworks, dependencies. |
| `crates/devpilot-ai` | LLM provider adapters (Ollama, Claude, OpenAI, Gemini). |
| `crates/devpilot-storage` | Local JSON persistence (recent projects, settings). |
| `crates/devpilot-testing` | Shared mocks and fixtures. |
| `apps/desktop` | Tauri shell (composition root) + React/TypeScript/Tailwind UI. |

More: [docs/architecture.md](docs/architecture.md) · Decisions: [docs/adr](docs/adr) · Changes: [CHANGELOG.md](CHANGELOG.md).

## Contributing

Contributions are welcome — the project is young, which is the best time to shape it. Start with [CONTRIBUTING.md](CONTRIBUTING.md): it maps the codebase and shows how to add a provider, a language or a feature slice.

## License

[MIT](LICENSE)
