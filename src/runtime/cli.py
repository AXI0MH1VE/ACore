"""CLI entrypoint exposing BranchManager over stdin/stdout.

This is still a stub: it uses a trivial generator and scorer, but it
illustrates the control flow expected by the Rust supervisor.
"""

import sys

from branch_manager import BranchManager


def default_generator(prompt: str) -> str:
    # In a real system, this would call into a model such as Llama.
    return f"Model answer for: {prompt}"


def default_scorer(text: str) -> float:
    # Trivial scoring based on length.
    return float(len(text))


def main() -> int:
    prompt = sys.stdin.read().strip()
    if not prompt:
        print("", end="")
        return 0

    manager = BranchManager(generator=default_generator, n_branches=4)
    best = manager.generate_best_of_n(prompt, scorer=default_scorer)

    # Stdout is the chosen text; Rust will split into tokens.
    print(best.text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
