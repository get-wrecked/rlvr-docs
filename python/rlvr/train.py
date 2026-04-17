"""
RLVR Training Loop — GRPO training across 2,697 domains.

Usage:
    python -m rlvr.train --model qwen2-vl-7b --phase 1 --batch-size 64

Architecture:
    Policy (VLM) → generates responses
    → Verifier (Rust server or Python backends) → scores responses
    → GRPO Trainer → updates policy
    → Curriculum Controller → adjusts difficulty/domain mixing
    → TensorBoard → reports metrics
"""

import argparse
import json
import logging
import time
from pathlib import Path
from typing import Optional

from .registry import load_registry, VerifierBackend, DataSourceType
from .curriculum import CurriculumController
from .generators import math_generators, puzzle_generators, science_generators
from .client import RustVerifierClient, RustVerifierClientError

logging.basicConfig(level=logging.INFO, format='%(asctime)s [%(levelname)s] %(message)s')
logger = logging.getLogger(__name__)


class TaskSampler:
    """Samples tasks from generators or datasets based on domain configuration."""

    def __init__(self, registry):
        self.registry = registry
        self._generator_modules = {
            "math": math_generators,
            "puzzle": puzzle_generators,
            "science": science_generators,
        }

    def sample(self, domain: str, difficulty: int = 5) -> dict:
        """Sample a single task for the given domain."""
        config = self.registry.get(domain)
        if not config:
            return self._fallback_task(domain, difficulty)

        # Try procedural generation first
        for module in self._generator_modules.values():
            if hasattr(module, 'GENERATORS') and domain in module.GENERATORS:
                return module.generate(domain, difficulty)

        # Fallback: generic task
        return self._fallback_task(domain, difficulty)

    def _fallback_task(self, domain: str, difficulty: int) -> dict:
        """Generate a basic task when no specific generator exists."""
        return {
            "prompt": f"[{domain}] Solve this problem at difficulty level {difficulty}.",
            "gold": "",
            "metadata": {"domain": domain, "difficulty": difficulty, "type": "text"}
        }


class VerifierClient:
    """Routes verification to the appropriate backend."""

    def __init__(self, registry, rust_server_url: Optional[str] = None):
        self.registry = registry
        self.rust_server_url = rust_server_url
        self._rust_client = RustVerifierClient(rust_server_url) if rust_server_url else None
        self._rust_supported = {
            VerifierBackend.EXACT_MATCH,
            VerifierBackend.CODE_EXECUTION,
            VerifierBackend.MATH_EQUIVALENCE,
            VerifierBackend.SQL_EXECUTION,
            VerifierBackend.SCHEMA_VALIDATION,
            VerifierBackend.CHEMISTRY_CHECK,
            VerifierBackend.GRAPH_CHECK,
        }

        # Import verifier backends
        from .verifiers import exact_match, code_execution
        self._backends = {
            VerifierBackend.EXACT_MATCH: exact_match.verify,
            VerifierBackend.CODE_EXECUTION: code_execution.verify,
        }

    def verify(self, task: dict, response: str, domain: str) -> dict:
        """Verify a response and return score + metadata."""
        config = self.registry.get(domain)
        if config and self._should_use_rust(config.verifier_backend):
            try:
                return self._rust_client.verify(
                    task=task,
                    response=response,
                    domain=domain,
                    verifier=config.verifier_backend.value,
                )
            except RustVerifierClientError as exc:
                logger.warning(
                    "Rust verifier unavailable for %s: %s; using local fallback",
                    domain,
                    exc,
                )

        return self._verify_local(task, response, domain)

    def _verify_local(self, task: dict, response: str, domain: str) -> dict:
        """Verify with the in-process Python fallback path."""
        config = self.registry.get(domain)
        if not config:
            return {"score": 0.0, "reason": f"Unknown domain: {domain}"}

        backend = config.verifier_backend
        verify_fn = self._backends.get(backend)

        if verify_fn:
            result = verify_fn(task, response)
            return {
                "domain": domain,
                "score": result.score,
                "reason": result.reason,
                "difficulty": task.get("metadata", {}).get("difficulty", 5),
            }

        # Fallback: exact match on gold answer
        from .verifiers.exact_match import verify as em_verify
        result = em_verify(task, response)
        return {
            "domain": domain,
            "score": result.score,
            "reason": result.reason,
            "difficulty": task.get("metadata", {}).get("difficulty", 5),
        }

    def verify_batch(
        self,
        tasks: list[dict],
        responses: list[str],
        domains: list[str],
    ) -> list[dict]:
        """Verify a batch of responses."""
        if not (len(tasks) == len(responses) == len(domains)):
            raise ValueError("tasks, responses, and domains must have the same length")

        if not self._rust_client:
            return [
                self._verify_local(task, response, domain)
                for task, response, domain in zip(tasks, responses, domains)
            ]

        results = [None] * len(tasks)
        rust_requests = []
        rust_positions = []

        for i, (task, response, domain) in enumerate(zip(tasks, responses, domains)):
            config = self.registry.get(domain)
            if config and self._should_use_rust(config.verifier_backend):
                rust_positions.append(i)
                rust_requests.append({
                    "task_id": str(i),
                    "domain": domain,
                    "verifier": config.verifier_backend.value,
                    "task": task,
                    "response": response,
                })
            else:
                results[i] = self._verify_local(task, response, domain)

        if rust_requests:
            try:
                rust_results = self._rust_client.verify_batch(rust_requests)
                for position, result in zip(rust_positions, rust_results):
                    results[position] = result
            except RustVerifierClientError as exc:
                logger.warning("Rust batch verification failed: %s; using local fallback", exc)
                for position in rust_positions:
                    results[position] = self._verify_local(
                        tasks[position],
                        responses[position],
                        domains[position],
                    )

        return results

    def _should_use_rust(self, backend: VerifierBackend) -> bool:
        return self._rust_client is not None and backend in self._rust_supported


def train(
    model_name: str = "Qwen/Qwen2-VL-7B-Instruct",
    phase: int = 1,
    batch_size: int = 64,
    num_generations: int = 4,
    learning_rate: float = 1e-6,
    max_steps: int = 100000,
    log_every: int = 100,
    eval_every: int = 1000,
    output_dir: str = "outputs",
):
    """
    Main training loop.

    This is the reference implementation. For actual training, use:
    - TRL's GRPOTrainer for the RL optimization
    - vLLM or SGLang for fast inference
    - The Rust verifier server for production verification
    """
    logger.info(f"Initializing RLVR training")
    logger.info(f"  Model: {model_name}")
    logger.info(f"  Phase: {phase}")
    logger.info(f"  Batch size: {batch_size}")
    logger.info(f"  Num generations: {num_generations}")

    # Load registry
    registry = load_registry()
    logger.info(f"  Domains: {len(registry)}")

    # Initialize components
    curriculum = CurriculumController(registry)
    curriculum.set_phase(phase)
    sampler = TaskSampler(registry)
    verifier = VerifierClient(registry)

    logger.info(f"  Curriculum stage weights: {curriculum.stage_weights}")

    # Training loop
    step = 0
    total_reward = 0.0
    domain_rewards = {}

    logger.info("Starting training loop...")

    while step < max_steps:
        # 1. Sample batch of (domain, difficulty) pairs
        batch_config = curriculum.sample_batch(batch_size)

        # 2. Generate tasks for each domain
        tasks = []
        domains = []
        for bc in batch_config:
            task = sampler.sample(bc["domain"], bc["difficulty"])
            tasks.append(task)
            domains.append(bc["domain"])

        # 3. Generate responses from policy (placeholder — use TRL/vLLM in practice)
        # In real training:
        #   responses = policy.generate(prompts, num_return_sequences=num_generations)
        # Here we just use empty responses for the framework demo
        prompts = [t["prompt"] for t in tasks]

        # 4. Verify responses
        # In real training, each prompt gets num_generations responses
        # and GRPO uses the group of scores for policy gradient
        # For demo, we simulate:
        responses = ["" for _ in prompts]  # placeholder
        results = verifier.verify_batch(tasks, responses, domains)

        # 5. Update curriculum
        curriculum.update(results)

        # 6. Compute reward for GRPO
        scores = [r["score"] for r in results]
        mean_reward = sum(scores) / len(scores) if scores else 0.0
        total_reward += mean_reward

        # Track per-domain rewards
        for r in results:
            d = r["domain"]
            if d not in domain_rewards:
                domain_rewards[d] = []
            domain_rewards[d].append(r["score"])

        step += 1

        # 7. Log metrics
        if step % log_every == 0:
            report = curriculum.get_report()
            logger.info(
                f"Step {step}/{max_steps} | "
                f"Mean reward: {mean_reward:.3f} | "
                f"Active domains: {report['active_domains']}/{report['total_domains']} | "
                f"Avg difficulty: {report['avg_difficulty']:.1f} | "
                f"Avg success: {report['avg_success_rate']:.3f}"
            )

    logger.info("Training complete!")
    return curriculum.get_report()


def main():
    parser = argparse.ArgumentParser(description="RLVR Training Harness")
    parser.add_argument("--model", default="Qwen/Qwen2-VL-7B-Instruct", help="Base model")
    parser.add_argument("--phase", type=int, default=1, choices=[1, 2, 3, 4], help="Training phase")
    parser.add_argument("--batch-size", type=int, default=64)
    parser.add_argument("--num-generations", type=int, default=4)
    parser.add_argument("--lr", type=float, default=1e-6)
    parser.add_argument("--max-steps", type=int, default=100000)
    parser.add_argument("--output-dir", default="outputs")
    args = parser.parse_args()

    train(
        model_name=args.model,
        phase=args.phase,
        batch_size=args.batch_size,
        num_generations=args.num_generations,
        learning_rate=args.lr,
        max_steps=args.max_steps,
        output_dir=args.output_dir,
    )


if __name__ == "__main__":
    main()
