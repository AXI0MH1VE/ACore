# Axiom Hive Core · Invariants

This document states the core guarantees Axiom Hive Core aims to provide. These are backed by unit tests, property-based tests, and the structure of the implementation.

## 1. Determinism

For a fixed:

- constitution (`config/constitution.toml`),
- runtime implementation and configuration (`PYTHON_BIN`, `src/runtime/cli.py`), and
- sequence of prompts,

Axiom Hive Core guarantees:

1. The supervisor pipeline is **pure** with respect to its inputs.
2. The ledger forms a **deterministic hash chain**:
   - The same sequence of `(prompt, supervised_output)` pairs produces the same sequence of ledger entries.
   - Each entry `i > 0` satisfies `entries[i].prev_hash == entries[i-1].hash`.

These properties are encoded in tests under `src/lib.rs` and `tests/ledger_props.rs`.

## 2. Auditability

Every call that produces a supervised answer appends a `LedgerEntry` containing:

- `index` – a monotonically increasing integer starting from 0.
- `input_hash` – SHA-256 of the user prompt.
- `output_hash` – SHA-256 of the supervised output.
- `prev_hash` – hash of the previous entry, or `GENESIS` for the first.
- `hash` – SHA-256 over `(index, input_hash, output_hash, prev_hash)`.

This means:

- Any tampering with the ledger can be detected by recomputing hashes.
- A third party can reconstruct which prompt and supervised output correspond to a given hash, provided they have access to the plaintexts.

## 3. Runtime trust boundary

The runtime (Python CLI, HTTP service, or other backend) is treated as **untrusted**. The Rust supervisor enforces:

- All calls to an untrusted runtime go through an implementation of `UntrustedRuntime`.
- Failures are surfaced as structured `RuntimeError` values.
- The supervisor does not write partial or inconsistent ledger entries on runtime failures.

When a runtime fails, callers may choose to:

- propagate the error (preferred in high-assurance deployments), or
- use a local fallback answer while still appending an auditable ledger entry marking the fallback.

## 4. Constitution as code

The constitution module enforces:

- Well-formed configuration via `Constitution::load` and `Constitution::validate`.
- Basic structural constraints, such as:
  - `max_branches >= 1`.
  - `0.0 <= min_consensus_ratio <= 1.0`.

Higher-level invariants (for example, "this policy must always be enabled") should be expressed as additional validation rules in this module, so they remain machine-checkable.

## 5. Extensibility without violating invariants

When extending Axiom Hive Core:

- New runtimes should implement `UntrustedRuntime` and must not bypass the ledger or interceptor.
- New binaries or services should:
  - Load the constitution via the shared module.
  - Use the interceptor for all user-visible text outputs.
  - Append ledger entries for every supervised answer.

As long as these contracts are respected, additional transports (HTTP, gRPC) and runtimes (OpenAI, local models, tools) can be integrated without weakening the core guarantees.