---
domain: Causal Reasoning & Inference
category: reasoning
verification_type: exact_match
dataset_scale: 50K+ (from causal inference benchmarks)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Causal Reasoning & Inference

## Overview
Answer causal questions: identify causes and effects, determine if a variable is a confounder, compute causal effects from data using do-calculus, identify valid adjustment sets, determine identifiability. Verification: match against known causal graph ground truth or compute using causal inference algorithms.

## Verification Mechanism
```python
from causallearn.search.ScoreSearch import GES
from dowhy import CausalModel

def verify(task_type: str, problem: dict, answer: Any) -> float:
    if task_type == "identify_adjustment_set":
        # Check if proposed adjustment set is valid (back-door criterion)
        graph = problem["causal_graph"]
        treatment = problem["treatment"]
        outcome = problem["outcome"]
        proposed_set = set(answer)
        
        valid_sets = compute_all_valid_adjustment_sets(graph, treatment, outcome)
        return 1.0 if proposed_set in valid_sets else 0.0
    
    elif task_type == "causal_effect":
        # Check if computed causal effect matches ground truth
        true_effect = problem["true_causal_effect"]
        return 1.0 if abs(answer - true_effect) < 0.05 else 0.0
    
    elif task_type == "is_confounder":
        graph = problem["causal_graph"]
        variable = problem["variable"]
        # Determine if variable is a confounder
        is_conf = is_confounder(graph, variable, problem["treatment"], problem["outcome"])
        return 1.0 if answer == is_conf else 0.0
    
    elif task_type == "d_separation":
        # Check d-separation statement
        graph = problem["causal_graph"]
        result = d_separated(graph, problem["X"], problem["Y"], problem.get("Z", set()))
        return 1.0 if answer == result else 0.0
```

## Dataset Sources
- **CausalBench**: Benchmark for causal reasoning.
- **Tüebingen pairs**: Cause-effect pairs from real data.
- **BnLearn repository**: Bayesian network datasets with known structures.
- **CLADDER**: Causal reasoning benchmark (formal causal questions).
- **CORR2CAUSE**: Dataset testing correlation vs. causation reasoning.
- **Procedural generation**: Generate random DAGs, compute d-separation, adjustment sets.
- **Judea Pearl's textbook exercises**: Causal inference problems with solutions.

## Task Format
- **Input**: "Given the causal graph X → Z → Y, X → Y, is Z a confounder, mediator, or collider for the effect of X on Y?"
- **Output**: "Z is a mediator"

## Difficulty Curriculum
- Level 1: Simple 3-variable causal chains, basic identification
- Level 3: d-separation in larger graphs
- Level 5: Adjustment set computation, front-door criterion
- Level 7: Instrumental variables, bounds on unidentifiable effects
- Level 9: Transportability, selection bias, nested counterfactuals

## Limitations & Risks
- Causal reasoning from observational data has fundamental limitations (assumptions needed). Focus on problems with known causal structure.
- Graph-based causal reasoning is fully verifiable. Data-based causal discovery is less so.
- Do-calculus provides a complete algorithm for identifiability — implement as verifier.

## Connections
- [[probability-statistics]] — statistical foundations
- [[graph-theory]] — causal graphs are DAGs
- [[scientific-experiment-design]] — experiments for causal identification
