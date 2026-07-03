# DevPilot

AI-powered GitHub repository analyzer. Desktop app for Windows, macOS, and Linux.

> **Status: early development.** The project skeleton is in place; features are being built in the open. Nothing to download yet.

## What it will do

- Open a GitHub repository (URL or local path) and analyze it with tree-sitter: structure, metrics, hotspots.
- Chat with an AI about the codebase. Supported providers: Ollama (local, default), Claude, OpenAI, Gemini.
- Generate insight reports: code quality, risks, complexity trends.

## Development

Prerequisites: [Rust (stable)](https://rustup.rs), [Node.js 20+](https://nodejs.org), [pnpm](https://pnpm.io), and the [Tauri system dependencies](https://tauri.app/start/prerequisites/) for your OS.

```sh
cd apps/desktop
pnpm install
pnpm tauri dev
```

## Architecture

Cargo workspace with Clean Architecture: all dependencies point inward to `devpilot-core`.

| Path | Purpose |
|---|---|
| `crates/devpilot-core` | Domain: entities, ports (traits), use cases. No heavy deps. |
| `crates/devpilot-analysis` | tree-sitter parsing, AST metrics. |
| `crates/devpilot-git` | Repository reading, history, churn. |
| `crates/devpilot-ai` | LLM provider adapters (Ollama, Claude, OpenAI, Gemini). |
| `crates/devpilot-storage` | Local SQLite cache and settings. |
| `crates/devpilot-testing` | Shared mocks and fixtures. |
| `apps/desktop` | Tauri shell (composition root) + React/TypeScript/Tailwind UI. |

Details: [docs/architecture.md](docs/architecture.md). Decisions: [docs/adr](docs/adr).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
