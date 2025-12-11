# Axiom Hive Core · Operations

This document sketches how to run Axiom Hive Core in more realistic environments.

## 1. Process model

The reference implementation is a CLI-first tool. In production you would typically:

- Wrap `axiom_hive_cli` or `hive_supervisor` in a long-running service (systemd, container, or custom daemon), or
- Embed the supervisor modules in your own Rust service binary and expose an HTTP or gRPC interface.

The key constraints are:

- All user-facing text must pass through the interceptor.
- All supervised answers must be logged in the ledger.
- All untrusted runtimes must sit behind `UntrustedRuntime`.

## 2. Configuration management

The constitution is plain TOML at `config/constitution.toml`.

Recommended patterns:

- Use explicit versioning in `[meta]` and keep past versions under `config/constitutions/`.
- Store constitutions in the same version control system as code.
- Deploy updates via normal release pipelines, not by editing production files in place.

On startup:

- The supervisor validates `max_branches` and `min_consensus_ratio`.
- Any validation error should be treated as a failed deployment and surfaced by your orchestration layer.

## 3. Ledger handling

The built-in ledger is in-memory and intended as a reference.

For production:

- Replace the in-memory vector with:
  - an append-only file,
  - a write-ahead log,
  - or an external database table that enforces immutability at the application level.
- Rotate or archive logs on a schedule, depending on regulatory and storage requirements.
- Consider shipping hashes to a separate system of record (for example, a blockchain or distributed log) for independent attestation.

## 4. Runtime isolation

The default runtime bridge spawns a Python process and communicates via stdin/stdout.

For production:

- Run the runtime in a restricted environment (container, VM, or sandbox).
- Apply resource limits (CPU, memory, wall-clock time) via your process supervisor.
- Use network policies to prevent the runtime from reaching unapproved endpoints unless explicitly required.

When adding new runtimes (HTTP, OpenAI, local models):

- Implement `UntrustedRuntime` and keep all side effects inside that boundary.
- Ensure that any network or file access is treated as untrusted and audited.

## 5. Observability

At minimum you should monitor:

- Successful and failed calls to the runtime.
- Counts of tokens removed by the interceptor.
- Ledger growth (number of entries, storage size if persisted).

You can integrate with existing logging/metrics systems by:

- Emitting structured logs (JSON) from your binaries.
- Exposing counters via HTTP if you embed the supervisor in a service.

## 6. Failure handling

The binaries use exit codes to signal different failure classes:

- `0` – success.
- `1` – user input error (for example, empty prompt).
- `2` – constitution/configuration error.
- `3` and above – runtime or unexpected errors (you can refine this in your own binaries).

In higher-level services:

- Map configuration errors to startup failures.
- Map runtime errors to clear API responses while preserving ledger semantics where appropriate.

## 7. Extending safely

When extending Axiom Hive Core:

- Keep new logic behind explicit interfaces.
- Add tests for any new invariants.
- Update `docs/INVARIANTS.md` when you introduce new guarantees that external integrators can rely on.

The core principle is that new capabilities must not weaken determinism or auditability. If a feature conflicts with those properties, it should live outside the core or be redesigned.