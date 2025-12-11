# Axiom Hive Core · Architecture

This document describes the concrete implementation in this repository, as distinct from the broader conceptual design in the whitepaper.

## 1. Modules

### 1.1 Supervisor (Rust)

- `src/supervisor/interceptor.rs`  
  Stateless component that receives a token stream and a set of disallowed tokens derived from the constitution. It returns a filtered list of tokens. In a production deployment this would sit on the logit/decoding boundary of the model.

- `src/supervisor/ledger.rs`  
  In‑memory append‑only ledger. Each `LedgerEntry` stores:
  - `index` – monotonically increasing integer.
  - `input_hash` – SHA‑256 of the user prompt.
  - `output_hash` – SHA‑256 of the supervised output.
  - `prev_hash` – hash of the previous entry, or `GENESIS`.
  - `hash` – SHA‑256 over `(index, input_hash, output_hash, prev_hash)`.

This creates a simple, auditable hash chain for all supervised interactions.

### 1.2 Runtime bridge (Rust)

- `src/runtime_bridge.rs`  
  Provides `ExternalPythonRuntime`, a minimal bridge that:
  - Resolves the Python binary from `PYTHON_BIN` or defaults to `python`.
  - Spawns `src/runtime/cli.py` as a subprocess.
  - Writes the prompt to stdin.
  - Reads the chosen answer from stdout as UTF‑8.
  - Surfaces errors as Rust `Result<String, String>` values.

This keeps the supervisor runtime‑agnostic: any system that can speak stdin/stdout can be wrapped.

### 1.3 Runtime (Python)

- `src/runtime/branch_manager.py`  
  Implements `BranchManager`, which orchestrates best‑of‑N sampling using a pluggable `generator(prompt) -> str` and `scorer(text) -> float`.

- `src/runtime/cli.py`  
  Thin CLI that:
  - Reads the full prompt from stdin.
  - Constructs a `BranchManager` with a default generator and scorer.
  - Selects the best branch and writes the chosen text to stdout.

In a production system, the generator would call an actual model (e.g., GGUF, PyTorch, or an external inference API).

## 2. Binaries

### 2.1 `axiom_hive_cli`

Location: `src/bin/axiom_hive_cli.rs`.

Responsibilities:

1. Accept a prompt from CLI arguments or stdin.
2. Load `config/constitution.toml` and derive policy settings.
3. Build a minimal list of disallowed tokens from the policy (e.g., PII markers, harmful verbs).
4. Call `ExternalPythonRuntime` with the user prompt.
5. Tokenize the returned text and filter it through the interceptor.
6. Append a `LedgerEntry` capturing the prompt and supervised output.
7. Print the supervised answer and ledger metadata for audit.

### 2.2 `hive_supervisor`

Location: `src/bin/hive_supervisor.rs`.

This binary is a scripted demo that exercises the same plumbing using a fixed prompt. It is useful for:

- Verifying integration without interactive input.
- Serving as a template for embedding the supervisor into other services.

## 3. Configuration

### 3.1 Constitution

`config/constitution.toml` is intentionally small but demonstrates the pattern:

- `[meta]` – human‑readable name and version.
- `[policy]` – high‑level safety knobs (e.g., `allow_pii_leakage`).
- `[supervisor]` – structural limits (e.g., `max_branches`, `min_consensus_ratio`).

In this core implementation, policy is mapped directly to a blacklist of tokens. More advanced deployments could map policy into:

- logit bias tables,
- structured guardrails,
- runtime invariants enforced by formal methods.

## 4. Extending the core

The repository is designed to be extended without rewriting the core invariants.

Examples:

- Swap out the default Python generator for your own model implementation.
- Replace the in‑memory ledger with a persistent store or append‑only log.
- Expose the supervisor via HTTP or gRPC while reusing the existing modules.

Keep the following invariant in mind when extending:

> The runtime may guess; the supervisor may not.

All external integrations should preserve the ledger semantics and constitution‑driven interception so that every answer remains auditable and policy‑constrained.