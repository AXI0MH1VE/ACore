# Axiom Hive Core

A reference implementation of the "Axiom Hive" architecture described in the technical audit JSON.

- **Supervisor (Rust)**: `src/supervisor/` — the O.L.O. "digital straitjacket" that wraps any LLM runtime.
- **Runtime (Python)**: `src/runtime/` — interfaces with PyTorch / GGUF models and implements branch-based search.
- **Constitution (TOML)**: `config/constitution.toml` — immutable policy-as-code, loaded by the supervisor at startup.
- **Ledger (Rust)**: `src/supervisor/ledger.rs` — cryptographic, append-only audit trail of all supervised interactions.

This repo is a scaffold only: it gives you the right boundaries and file layout to build the full system.