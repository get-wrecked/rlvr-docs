"""
Domain Registry — maps all 2,697 domains to verifier backends, data sources, and generators.

Architecture:
  - Each domain has a `verifier_backend` (one of ~15 backends that handle verification)
  - Each domain has a `data_source` (open dataset, procedural generator, or RL environment)
  - Domains sharing the same backend are grouped for efficient verification

Verifier backends (in order of coverage):
  1. exact_match       — normalized string/number comparison (covers ~1,200 domains)
  2. code_execution    — run code in sandbox, check output (covers ~400 domains)
  3. env_reward        — RL environment step() reward (covers ~500 domains)
  4. constraint_check  — verify all constraints satisfied (covers ~100 domains)
  5. metric_based      — BLEU/ROUGE/SSIM/F1 above threshold (covers ~75 domains)
  6. rule_engine       — apply formal rule set (covers ~50 domains)
  7. structural_diff   — AST/DOM/pixel structural comparison (covers ~50 domains)
  8. simulation        — physics/circuit/logic simulator (covers ~45 domains)
  9. math_equivalence  — symbolic math equivalence (covers ~30 domains)
  10. proof_check      — formal proof checker (Lean/Coq/Isabelle) (covers ~20 domains)
  11. sql_execution    — execute SQL, compare result sets (covers ~20 domains)
  12. schema_validation — JSON/XML/YAML schema check (covers ~15 domains)
  13. chemistry_check  — RDKit/xTB molecular verification (covers ~30 domains)
  14. graph_check      — NetworkX graph property verification (covers ~15 domains)
  15. audio_metric     — SDR/WER/MOS audio metrics (covers ~20 domains)
"""

import json
import os
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Optional, Callable


class VerifierBackend(Enum):
    EXACT_MATCH = "exact_match"
    CODE_EXECUTION = "code_execution"
    ENV_REWARD = "env_reward"
    CONSTRAINT_CHECK = "constraint_check"
    METRIC_BASED = "metric_based"
    RULE_ENGINE = "rule_engine"
    STRUCTURAL_DIFF = "structural_diff"
    SIMULATION = "simulation"
    MATH_EQUIVALENCE = "math_equivalence"
    PROOF_CHECK = "proof_check"
    SQL_EXECUTION = "sql_execution"
    SCHEMA_VALIDATION = "schema_validation"
    CHEMISTRY_CHECK = "chemistry_check"
    GRAPH_CHECK = "graph_check"
    AUDIO_METRIC = "audio_metric"


class DataSourceType(Enum):
    OPEN_DATASET = "open"           # Download from URL/HuggingFace
    PROCEDURAL = "procedural"       # Generate at runtime
    RL_ENVIRONMENT = "rl_env"       # Step through environment
    SELF_PLAY = "self_play"         # Generate via self-play
    REMEMBERED = "remembered"       # Needs verification before use


class Stage(Enum):
    PRE = 1    # Rule recognition
    MID = 2    # System mastery
    POST = 3   # Capability climbing


@dataclass
class DomainConfig:
    slug: str
    name: str
    verifier_backend: VerifierBackend
    data_source_type: DataSourceType
    stage: Stage
    difficulty_range: tuple = (1, 10)
    generator_fn: Optional[str] = None      # module.function for procedural
    loader_fn: Optional[str] = None         # module.function for dataset loading
    env_id: Optional[str] = None            # Gymnasium env ID for RL
    dataset_url: Optional[str] = None       # HuggingFace or URL
    verifier_config: dict = field(default_factory=dict)


# ═══════════════════════════════════════════════════════
# VERIFIER BACKEND ROUTING
# Maps verification_type strings to VerifierBackend enum
# ═══════════════════════════════════════════════════════

_VTYPE_TO_BACKEND = {
    "exact_match": VerifierBackend.EXACT_MATCH,
    "exact": VerifierBackend.EXACT_MATCH,
    "execution": VerifierBackend.CODE_EXECUTION,
    "outcome": VerifierBackend.ENV_REWARD,
    "constraint": VerifierBackend.CONSTRAINT_CHECK,
    "approximate": VerifierBackend.METRIC_BASED,
    "rule": VerifierBackend.RULE_ENGINE,
    "diff": VerifierBackend.STRUCTURAL_DIFF,
    "simulation": VerifierBackend.SIMULATION,
}

# Override specific domains to more precise backends
_DOMAIN_BACKEND_OVERRIDES = {
    # Math equivalence (symbolic, not just string match)
    "math-symbolic": VerifierBackend.MATH_EQUIVALENCE,
    "math-equivalence": VerifierBackend.MATH_EQUIVALENCE,
    "math-formal-proofs": VerifierBackend.PROOF_CHECK,
    "lean-mathlib-contribution": VerifierBackend.PROOF_CHECK,
    "autoformalization": VerifierBackend.PROOF_CHECK,
    "imo-problem-solving": VerifierBackend.MATH_EQUIVALENCE,
    # SQL
    "sql-generation": VerifierBackend.SQL_EXECUTION,
    "text-to-sql-complex": VerifierBackend.SQL_EXECUTION,
    "bird-sql": VerifierBackend.SQL_EXECUTION,
    "spider-dev": VerifierBackend.SQL_EXECUTION,
    "sql-window-functions": VerifierBackend.SQL_EXECUTION,
    "sql-recursive-cte": VerifierBackend.SQL_EXECUTION,
    # Schema validation
    "json-schema": VerifierBackend.SCHEMA_VALIDATION,
    "yaml-schema-validation": VerifierBackend.SCHEMA_VALIDATION,
    "xml-dtd-validation": VerifierBackend.SCHEMA_VALIDATION,
    "kubernetes-manifest-generation": VerifierBackend.SCHEMA_VALIDATION,
    # Chemistry
    "chemistry-computation": VerifierBackend.CHEMISTRY_CHECK,
    "molecular-generation": VerifierBackend.CHEMISTRY_CHECK,
    "drug-property-prediction": VerifierBackend.CHEMISTRY_CHECK,
    # Graph
    "graph-theory": VerifierBackend.GRAPH_CHECK,
    "graph-coloring": VerifierBackend.GRAPH_CHECK,
    "graph-algorithm-execution": VerifierBackend.GRAPH_CHECK,
    # Audio
    "audio-speech-recognition": VerifierBackend.AUDIO_METRIC,
    "music-source-separation": VerifierBackend.AUDIO_METRIC,
}

# Domains that use RL environments (outcome-based via env.step())
_RL_ENV_DOMAINS = set()
_RL_ENV_PREFIXES = [
    "atari-", "dmc-", "metaworld-", "d4rl-", "procgen-",
    "minihack-", "babyai-", "rlbench-", "safetygym-",
    "gym-", "pz-", "openspiel-", "alfred-", "behavior-",
]


def _classify_data_source(slug: str, status: str) -> DataSourceType:
    """Determine data source type from domain slug and status."""
    if status == "remembered":
        return DataSourceType.REMEMBERED

    # RL environments
    for prefix in _RL_ENV_PREFIXES:
        if slug.startswith(prefix):
            return DataSourceType.RL_ENVIRONMENT

    # Self-play games
    if any(slug.startswith(p) for p in ["chess", "go-", "hex-", "connect-four"]):
        return DataSourceType.SELF_PLAY

    # Known procedural domains
    procedural_keywords = [
        "procedural", "unlimited", "unit-conversion", "date-time",
        "chemical-equation", "regex-synthesis", "json-schema",
        "graph-properties", "roman-numeral", "base-conversion",
        "morse-code", "isbn", "checksum", "color-space",
        "fibonacci", "catalan", "bernoulli", "stirling",
        "euler-circuit", "hamiltonian", "magic-square", "latin-square",
        "algo-", "pattern-", "system-of-equations", "eigenvalue",
        "derivative", "integral", "limit-computation", "series-convergence",
        "projectile-motion", "circular-motion", "ohms-law",
    ]
    for kw in procedural_keywords:
        if kw in slug:
            return DataSourceType.PROCEDURAL

    return DataSourceType.OPEN_DATASET


def _classify_stage(slug: str, category: str) -> Stage:
    """Assign training stage based on domain characteristics."""
    # Stage 1: pure structure, no domain knowledge
    stage1_prefixes = [
        "cellular-automata", "synthetic-grammar", "abstract-pattern",
        "boolean-function", "latent-concept", "formal-language",
        "logic-propositional", "logic-first-order", "smt-solving",
        "algo-", "binary-decision", "truth-table", "lambda-calculus",
        "automata-construction", "regex-", "mnist", "cifar",
    ]
    for p in stage1_prefixes:
        if slug.startswith(p):
            return Stage.PRE

    # Stage 3: specialized/expert domains
    stage3_prefixes = [
        "medical-", "legal-", "clinical-", "drug-", "protein-",
        "molecular-", "catalyst-", "crystal-", "antenna-",
        "cybersecurity-", "reverse-engineering", "binary-exploitation",
        "frontier-math", "imo-", "putnam-", "superhuman",
    ]
    for p in stage3_prefixes:
        if slug.startswith(p):
            return Stage.POST

    # Everything else: Stage 2
    return Stage.MID


def load_registry(domains_json: str = "domains.json") -> dict[str, DomainConfig]:
    """Load all domains from domains.json and create DomainConfig for each."""
    path = Path(domains_json)
    if not path.exists():
        path = Path(__file__).parent.parent.parent / "domains.json"

    with open(path) as f:
        raw_domains = json.load(f)

    registry = {}
    for d in raw_domains:
        slug = d["slug"]
        vtype_str = d.get("verification_type", "exact_match")
        status = d.get("status", "remembered")
        category = d.get("category", "")

        # Determine verifier backend
        if slug in _DOMAIN_BACKEND_OVERRIDES:
            backend = _DOMAIN_BACKEND_OVERRIDES[slug]
        else:
            # Parse the first verification type if multiple
            primary_vtype = vtype_str.split("|")[0].split(",")[0].strip().lower()
            backend = _VTYPE_TO_BACKEND.get(primary_vtype, VerifierBackend.EXACT_MATCH)

        # Determine data source
        data_source = _classify_data_source(slug, status)

        # Determine stage
        stage = _classify_stage(slug, category)

        config = DomainConfig(
            slug=slug,
            name=d.get("domain", slug),
            verifier_backend=backend,
            data_source_type=data_source,
            stage=stage,
        )
        registry[slug] = config

    return registry


def get_backend_stats(registry: dict[str, DomainConfig]) -> dict:
    """Get statistics about verifier backend distribution."""
    stats = {
        "by_backend": {},
        "by_data_source": {},
        "by_stage": {},
        "total": len(registry),
    }
    for config in registry.values():
        b = config.verifier_backend.value
        stats["by_backend"][b] = stats["by_backend"].get(b, 0) + 1
        ds = config.data_source_type.value
        stats["by_data_source"][ds] = stats["by_data_source"].get(ds, 0) + 1
        s = config.stage.name
        stats["by_stage"][s] = stats["by_stage"].get(s, 0) + 1
    return stats


if __name__ == "__main__":
    registry = load_registry()
    stats = get_backend_stats(registry)
    print(f"Loaded {stats['total']} domains\n")
    print("By verifier backend:")
    for k, v in sorted(stats["by_backend"].items(), key=lambda x: -x[1]):
        print(f"  {k}: {v}")
    print("\nBy data source:")
    for k, v in sorted(stats["by_data_source"].items(), key=lambda x: -x[1]):
        print(f"  {k}: {v}")
    print("\nBy training stage:")
    for k, v in sorted(stats["by_stage"].items(), key=lambda x: -x[1]):
        print(f"  {k}: {v}")
