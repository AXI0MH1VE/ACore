"""Branch manager: orchestrates best-of-N / tree-of-thoughts style sampling.

This is a *stub* that illustrates the control flow between the Rust supervisor
and a Python runtime. In practice, this would call into PyTorch / GGUF-backed
models such as Llama or Mistral.
"""

from dataclasses import dataclass
from typing import Callable, List


@dataclass
class BranchResult:
    text: str
    score: float


class BranchManager:
    def __init__(self, generator: Callable[[str], str], n_branches: int = 4):
        self.generator = generator
        self.n_branches = n_branches

    def generate_best_of_n(self, prompt: str, scorer: Callable[[str], float]) -> BranchResult:
        branches: List[BranchResult] = []
        for _ in range(self.n_branches):
            text = self.generator(prompt)
            score = scorer(text)
            branches.append(BranchResult(text=text, score=score))
        return max(branches, key=lambda b: b.score)