"""
Verifier backends — 15 backends covering all 2,697 domains.

Each backend implements:
  verify(task: dict, response: str) -> VerifyResult

The backends are:
  1. exact_match      — normalized string/number comparison
  2. code_execution   — run code in sandbox, check output
  3. env_reward       — RL environment step() reward
  4. constraint_check — verify all constraints satisfied
  5. metric_based     — BLEU/ROUGE/SSIM/F1 threshold
  6. rule_engine      — formal rule checking
  7. structural_diff  — AST/DOM/pixel comparison
  8. simulation       — physics/circuit simulator
  9. math_equivalence — symbolic math equivalence
  10. proof_check     — formal proof checker
  11. sql_execution   — execute SQL, compare results
  12. schema_validation — JSON/XML/YAML schema
  13. chemistry_check — molecular verification
  14. graph_check     — graph property verification
  15. audio_metric    — audio quality metrics
"""

from dataclasses import dataclass


@dataclass
class VerifyResult:
    """Result of verifying a model response against a task."""
    score: float          # 0.0 (wrong) to 1.0 (correct)
    reason: str           # Human-readable explanation
    metadata: dict = None # Domain-specific details

    @classmethod
    def correct(cls, reason: str = "correct") -> "VerifyResult":
        return cls(score=1.0, reason=reason)

    @classmethod
    def wrong(cls, reason: str = "incorrect") -> "VerifyResult":
        return cls(score=0.0, reason=reason)

    @classmethod
    def partial(cls, score: float, reason: str = "partial") -> "VerifyResult":
        return cls(score=max(0.0, min(1.0, score)), reason=reason)
