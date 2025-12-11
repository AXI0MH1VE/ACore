# Axiom Hive Core · Usage

This guide focuses on concrete commands and execution flows. For architecture and philosophy, see `README.md`, `docs/architecture.md`, and `docs/whitepaper_v1.md`.

## 1. Environment

### 1.1 Rust

Install a recent stable Rust toolchain.

```bash
rustup default stable
```

### 1.2 Python runtime

Install Python 3.x and ensure it is available as `python` on your PATH.

If your system uses a different command (for example `python3`), set:

```bash
# PowerShell
$env:PYTHON_BIN = "python3"
```

The Rust supervisor uses this variable to locate Python.

## 2. Build and test

From the repository root:

```bash
cargo test
```

This compiles the supervisor, runtime bridge, and binaries, and runs the Rust unit tests.

## 3. Constitution

The constitution is stored at `config/constitution.toml`.

Example fields:

- `[meta]` – name and version.
- `[policy]` – high‑level safety flags.
- `[supervisor]` – structural limits (branching, consensus thresholds).

You can safely copy this file and iterate, keeping the same shape. The supervisor will fail fast if the file is missing or malformed.

## 4. CLI workflow

### 4.1 Single prompt via arguments

```bash
cargo run --bin axiom_hive_cli -- "test prompt from CLI"
```

What happens:

1. The constitution is loaded.
2. A disallowed token list is derived from policy.
3. The prompt is sent to the Python runtime (`src/runtime/cli.py`).
4. `BranchManager` selects the best branch.
5. Tokens are filtered by the interceptor.
6. A ledger entry is appended.
7. The supervised answer and ledger metadata are printed.

### 4.2 Interactive mode

```bash
cargo run --bin axiom_hive_cli
```

If no arguments are provided, the CLI reads a single prompt from stdin. This is convenient for quick experiments.

## 5. Supervisor demo

The demo binary is useful for verifying that the stack is wired correctly.

```bash
cargo run --bin hive_supervisor
```

It uses a fixed prompt and prints:

- Loaded constitution parameters.
- The runtime answer from Python.
- Original and filtered tokens.
- The resulting ledger entry.

## 6. Customizing the runtime

The Python runtime is intentionally simple. You can extend it without touching the Rust supervisor.

### 6.1 Swap the generator

Edit `src/runtime/cli.py` and replace `default_generator` with your own implementation that calls:

- a local GGUF model,
- a PyTorch model,
- or a remote inference API.

As long as the CLI reads a prompt from stdin and writes a single answer to stdout, the Rust bridge does not need to change.

### 6.2 Adjust branching

`BranchManager` in `src/runtime/branch_manager.py` accepts `n_branches`. You can expose this as a configuration value or tune it directly in the CLI.

## 7. Operational notes

- The ledger in this repository is in‑memory only. For production use, back it with a durable store or append‑only log.
- The interceptor currently uses a simple token blacklist. You can replace this with structured rules, logit biasing, or external policy engines.
- Always preserve the invariant that the supervisor remains deterministic and auditable, even if the underlying runtime is probabilistic.