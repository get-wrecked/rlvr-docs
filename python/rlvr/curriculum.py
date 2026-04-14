"""
Curriculum Controller — manages difficulty and domain mixing across 2,697 domains.

The controller decides:
  1. Which domains to sample from (weighted by capability gap)
  2. What difficulty level to use (targeting 40-60% success rate)
  3. How to balance across training stages (Pre/Mid/Post)

It maintains per-domain statistics and adapts in real-time.
"""

import random
import math
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Optional

from .registry import DomainConfig, Stage, load_registry


@dataclass
class DomainStats:
    """Running statistics for a single domain."""
    total_attempts: int = 0
    total_correct: int = 0
    recent_scores: list = field(default_factory=list)  # last N scores
    target_difficulty: int = 1
    current_weight: float = 1.0

    WINDOW = 100  # rolling window for recent stats

    def add(self, score: float, difficulty: int):
        self.total_attempts += 1
        self.total_correct += int(score >= 0.5)
        self.recent_scores.append(score)
        if len(self.recent_scores) > self.WINDOW:
            self.recent_scores.pop(0)

    @property
    def success_rate(self) -> float:
        if not self.recent_scores:
            return 0.5  # prior
        return sum(1 for s in self.recent_scores if s >= 0.5) / len(self.recent_scores)

    @property
    def mean_score(self) -> float:
        if not self.recent_scores:
            return 0.5
        return sum(self.recent_scores) / len(self.recent_scores)


class CurriculumController:
    """
    Manages the training curriculum across all domains.

    Sampling strategy:
      1. Sample stage according to stage_weights
      2. Within stage, sample domain weighted by capability gap
         (domains with success rate furthest from 50% get less weight)
      3. Within domain, set difficulty to target ~40-60% success rate
    """

    def __init__(
        self,
        registry: Optional[dict[str, DomainConfig]] = None,
        stage_weights: Optional[dict[Stage, float]] = None,
        target_success_rate: float = 0.5,
        min_domain_weight: float = 0.01,
    ):
        if registry is None:
            registry = load_registry()

        self.registry = registry
        self.target_success_rate = target_success_rate
        self.min_domain_weight = min_domain_weight

        # Group domains by stage
        self.stage_domains: dict[Stage, list[str]] = defaultdict(list)
        for slug, config in registry.items():
            self.stage_domains[config.stage].append(slug)

        # Stage weights (default: balanced)
        self.stage_weights = stage_weights or {
            Stage.PRE: 0.15,
            Stage.MID: 0.65,
            Stage.POST: 0.20,
        }

        # Per-domain statistics
        self.stats: dict[str, DomainStats] = defaultdict(DomainStats)

    def sample_batch(self, batch_size: int) -> list[dict]:
        """
        Sample a batch of (domain, difficulty) pairs for training.

        Returns list of:
            {"domain": slug, "difficulty": int, "stage": Stage}
        """
        batch = []
        for _ in range(batch_size):
            # 1. Sample stage
            stage = self._sample_stage()

            # 2. Sample domain within stage
            domain_slug = self._sample_domain(stage)

            # 3. Get target difficulty
            difficulty = self._get_difficulty(domain_slug)

            batch.append({
                "domain": domain_slug,
                "difficulty": difficulty,
                "stage": stage,
            })

        return batch

    def update(self, results: list[dict]):
        """
        Update stats after a batch of verifications.

        results: [{"domain": slug, "score": float, "difficulty": int}, ...]
        """
        for r in results:
            slug = r["domain"]
            score = r["score"]
            difficulty = r["difficulty"]

            stats = self.stats[slug]
            stats.add(score, difficulty)

            # Adjust difficulty to maintain target success rate
            if len(stats.recent_scores) >= 10:
                sr = stats.success_rate
                if sr > 0.7:
                    stats.target_difficulty = min(10, stats.target_difficulty + 1)
                elif sr < 0.3:
                    stats.target_difficulty = max(1, stats.target_difficulty - 1)

            # Adjust domain weight — prefer domains near the learning frontier
            gap = abs(stats.success_rate - self.target_success_rate)
            # Bell curve: maximum weight at 50% success rate
            stats.current_weight = max(
                self.min_domain_weight,
                math.exp(-4 * gap ** 2)  # Gaussian around 0.5
            )

    def _sample_stage(self) -> Stage:
        stages = list(self.stage_weights.keys())
        weights = list(self.stage_weights.values())
        return random.choices(stages, weights=weights, k=1)[0]

    def _sample_domain(self, stage: Stage) -> str:
        domains = self.stage_domains[stage]
        if not domains:
            # Fallback to any domain
            domains = list(self.registry.keys())

        weights = [self.stats[d].current_weight for d in domains]
        total = sum(weights)
        if total == 0:
            return random.choice(domains)

        return random.choices(domains, weights=weights, k=1)[0]

    def _get_difficulty(self, domain_slug: str) -> int:
        return self.stats[domain_slug].target_difficulty

    def set_phase(self, phase: int):
        """Adjust stage weights for different training phases."""
        if phase == 1:
            self.stage_weights = {Stage.PRE: 0.80, Stage.MID: 0.15, Stage.POST: 0.05}
        elif phase == 2:
            self.stage_weights = {Stage.PRE: 0.10, Stage.MID: 0.80, Stage.POST: 0.10}
        elif phase == 3:
            self.stage_weights = {Stage.PRE: 0.05, Stage.MID: 0.30, Stage.POST: 0.65}
        elif phase == 4:
            self.stage_weights = {Stage.PRE: 0.05, Stage.MID: 0.30, Stage.POST: 0.65}

    def get_report(self) -> dict:
        """Get a summary report of curriculum state."""
        active_domains = sum(1 for s in self.stats.values() if s.total_attempts > 0)
        avg_difficulty = (
            sum(s.target_difficulty for s in self.stats.values() if s.total_attempts > 0) /
            max(active_domains, 1)
        )
        avg_success = (
            sum(s.success_rate for s in self.stats.values() if s.total_attempts > 0) /
            max(active_domains, 1)
        )

        return {
            "total_domains": len(self.registry),
            "active_domains": active_domains,
            "avg_difficulty": round(avg_difficulty, 2),
            "avg_success_rate": round(avg_success, 3),
            "stage_weights": {s.name: w for s, w in self.stage_weights.items()},
            "total_attempts": sum(s.total_attempts for s in self.stats.values()),
        }
