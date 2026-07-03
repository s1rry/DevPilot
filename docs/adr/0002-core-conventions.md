# ADR-0002: Core port and entity conventions

Date: 2026-07-03
Status: accepted

## Context

ADR-0001 fixed the crate layout and the inward dependency rule. Before
implementing the first ports and entities in `devpilot-core`, the concrete
conventions must be settled, because every adapter and use case will copy
them.

## Decision

1. **Ports are async traits via the `async-trait` crate.** Dependency
   injection wires ports as `Arc<dyn Trait>`; native `async fn` in traits is
   not `dyn`-compatible yet. When it becomes so, migration is mechanical.
2. **Entities derive `serde::Serialize`/`Deserialize`.** A parallel DTO layer
   would duplicate every type for no behavioral gain at our scale. Derives do
   not pull I/O into the domain.
3. **Identifiers are newtypes** (`RepositoryId`, `CommitHash`), never bare
   strings. The compiler prevents mixing them up.
4. **Errors are one `thiserror` enum per port** (`GitError`,
   `AnalysisError`, `CacheError`), deriving `Clone` and `PartialEq` so tests
   can configure and assert them. No `anyhow` in public signatures.
5. **Progress is a synchronous port** (`ProgressReporter::report`), not a
   channel: neutral to the async runtime and trivial to mock.
6. **Mocks are hand-written** in `devpilot-testing`, not generated with
   `mockall`: they read as ordinary code, and there are only four ports.
7. **Timestamps are Unix seconds (UTC) as `i64`.** No chrono dependency in
   the domain.
8. **Analyzer errors are per-file.** A file that cannot be analyzed becomes
   a `Diagnostic`; a run over a repository never aborts because of one file.

## Alternatives considered

- **Native async fn in traits**: rejected until `dyn` compatibility lands.
- **`mockall`**: rejected — proc-macro DSL raises the contribution barrier.
- **DTO layer instead of serde in core**: rejected — pure duplication.
- **tokio channels for progress**: rejected — couples core to a runtime.
- **One shared error enum**: rejected — destroys per-port match ergonomics.

## Consequences

- Adapters implement small, precisely typed contracts and can be developed
  and tested independently.
- `devpilot-core` gains three light dependencies: `async-trait`,
  `thiserror`, `serde`.
- Hand-written mocks must be extended by hand when ports grow. Accepted:
  port changes are rare and deliberate.
