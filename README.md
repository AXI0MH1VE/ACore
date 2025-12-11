# Axiom Hive Core

<p align="center">
  <img src="assets/axiom-hive-wordmark.svg" alt="Axiom Hive Core wordmark" width="480" />
</p>

Deterministic supervisor for untrusted runtimes.

Axiom Hive Core is a reference implementation of the Axiom Hive architecture: a **cryptographically auditable, policy‑driven supervisor** that wraps any LLM or sampling runtime. It turns a probabilistic generator into a constrained, verifiable system.

## High‑level architecture

- **Supervisor · Rust**  
  `src/supervisor/` — the on‑chain “digital straitjacket” that enforces policy at the token and interaction level.
- **Runtime · Python**  
  `src/runtime/` — branch‑based sampling (`BranchManager`) exposed via a small CLI for the Rust supervisor.
- **Constitution · TOML**  
  `config/constitution.toml` — immutable policy‑as‑code loaded at startup (safety, PII, branching limits, consensus).
- **Ledger · Rust**  
  `src/supervisor/ledger.rs` — cryptographic, append‑only audit trail linking every supervised interaction via hashes.

The full conceptual design is described in `docs/whitepaper_v1.md`. This repository gives you a concrete, minimal core you can run, extend, and audit.

## What you can do with v1

- Call the system from a **CLI** and receive a supervised answer:
  - Constitution is loaded.
  - Python runtime proposes an answer via `BranchManager`.
  - Rust interceptor applies constitution‑driven filters.
  - Ledger records a hash‑chained audit entry.
- Run a **supervisor demo** binary that exercises the full stack end‑to‑end.

## Getting started

### Prerequisites

- Rust toolchain (stable).
- Python 3.x on your PATH.

Optional:

- Set `PYTHON_BIN` to point at a specific Python executable if `python` is not the right command on your system.

```bash
# PowerShell
$env:PYTHON_BIN = "python"
```

### Clone and build

```bash
git clone https://github.com/AXI0MH1VE/ACore.git
cd ACore
cargo test
```

## Running the system

### 1. CLI entrypoint (recommended)

`axiom_hive_cli` is the primary interface for sending prompts through the supervisor.

Prompt as argument:

```bash
cargo run --bin axiom_hive_cli -- "test prompt from CLI"
```

Interactive prompt (stdin):

```bash
cargo run --bin axiom_hive_cli
# type your prompt, then press Enter
```

Pipeline for each request:

1. Load `config/constitution.toml`.
2. Send the prompt to the Python runtime (`src/runtime/cli.py`).
3. Receive the best‑of‑N answer from `BranchManager`.
4. Tokenize and apply `Interceptor` using policy‑derived disallowed tokens.
5. Append a `LedgerEntry` for the interaction.
6. Print the **supervised answer** and the corresponding ledger metadata.

### 2. Supervisor demo binary

`hive_supervisor` is a compact, scripted demo that exercises the same pipeline with a fixed prompt.

```bash
cargo run --bin hive_supervisor
```

This is useful as an integration smoke test and as a reference for integrating the supervisor into your own binaries or services.

## Project layout

- `src/lib.rs` – public modules for the supervisor and runtime bridge.
- `src/supervisor/interceptor.rs` – token‑level interceptor enforcing constitution‑derived constraints.
- `src/supervisor/ledger.rs` – append‑only, hash‑chained audit ledger.
- `src/runtime/branch_manager.py` – best‑of‑N branch selection runtime.
- `src/runtime/cli.py` – stdin/stdout CLI exposing the runtime to Rust.
- `src/runtime_bridge.rs` – Rust bridge that shells out to the Python CLI.
- `src/bin/axiom_hive_cli.rs` – main user‑facing CLI.
- `src/bin/hive_supervisor.rs` – demo supervisor binary.
- `config/constitution.toml` – example constitution.
- `docs/whitepaper_v1.md` – structured whitepaper describing the broader Axiom Hive architecture.
- `docs/architecture.md` – implementation‑level view of this repository.
- `docs/USAGE.md` – focused usage guide with concrete commands and flows.
- `docs/INVARIANTS.md` – determinism and auditability guarantees.
- `docs/OPS.md` – operational guidance for running Axiom Hive Core.

## Design principles

Axiom Hive Core is opinionated around three invariants:

1. **Determinism over stochastic comfort**  
   When in doubt, halt and prove rather than guess and hope.
2. **Cryptographic receipts, not narratives**  
   Every interaction should be auditable via hashes and immutable logs.
3. **Policy as code, not documentation**  
   The constitution is a machine‑readable contract, not a slide deck.

For the full philosophical and regulatory framing, see `docs/whitepaper_v1.md`.

## License

This project is licensed under the MIT License. See `LICENSE` for details.

## Branding assets

The `assets/` directory contains simple SVG assets for reuse:

- `assets/axiom-hive-wordmark.svg` – dark‑background wordmark used in this README.
- `assets/axiom-hive-wordmark-light.svg` – light‑background variant for embedding on light sites.
- `assets/axiom-hive-badge.svg` – compact badge suitable for external READMEs or documentation.

Example markdown for the badge:

```markdown
[![Axiom Hive Core](assets/axiom-hive-badge.svg)](https://github.com/AXI0MH1VE/ACore)
```
