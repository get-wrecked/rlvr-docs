---
domain: Bayesian Network Reasoning
category: probabilistic-reasoning
verification_type: execution
dataset_scale: 10K+ (from BN benchmarks + procedural)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Bayesian Network Reasoning

## Overview
Given a Bayesian network (DAG + conditional probability tables), answer probabilistic queries: compute marginal probabilities, conditional probabilities, most probable explanation (MPE), find d-separation sets. Verification: exact inference algorithms compute the correct answer.

## Verification Mechanism
```python
from pgmpy.models import BayesianNetwork
from pgmpy.inference import VariableElimination

def verify(task_type: str, bn_spec: dict, query: dict, answer: float) -> float:
    model = build_bayesian_network(bn_spec)
    inference = VariableElimination(model)
    
    if task_type == "marginal":
        result = inference.query([query["variable"]], evidence=query.get("evidence", {}))
        correct_prob = result.values[query["value_index"]]
        return 1.0 if abs(answer - correct_prob) < 0.001 else 0.0
    
    elif task_type == "conditional":
        result = inference.query([query["variable"]], evidence=query["evidence"])
        correct = result.values[query["value_index"]]
        return 1.0 if abs(answer - correct) < 0.001 else 0.0
    
    elif task_type == "mpe":
        result = inference.map_query(query.get("evidence", {}))
        return 1.0 if answer == result else 0.0
    
    elif task_type == "d_separation":
        correct = d_separated(model, query["X"], query["Y"], query.get("Z", set()))
        return 1.0 if answer == correct else 0.0
```

## Dataset Sources
- **BnLearn repository**: Standard Bayesian networks (Asia, Alarm, Insurance, etc.).
- **bnlearn R package datasets**: Curated collection.
- **UAI competition benchmarks**: Probabilistic inference competition.
- **Medical diagnosis BNs**: Published clinical Bayesian networks.
- **Procedural generation**: Generate random DAGs + CPTs. Unlimited.
- **Textbook exercises**: Koller & Friedman, Pearl, Murphy.

## Task Format
- **Input**: Bayesian network diagram + CPTs + "What is P(Burglary | JohnCalls=true, MaryCalls=true)?"
- **Output**: 0.284 (numerical probability)

## Difficulty Curriculum
- Level 1: Simple 3-node networks, direct probability lookup
- Level 3: 5-10 nodes, requires chain rule application
- Level 5: 20+ nodes, requires message passing or variable elimination
- Level 7: Structure learning from data
- Level 9: Dynamic Bayesian networks, continuous variables

## Limitations & Risks
- Exact inference is #P-hard in general, but efficient for small-to-medium networks.
- For large networks, approximate inference (MCMC) introduces error. Focus on networks where exact inference is tractable.
- Numerical precision: compute at sufficient precision and compare with appropriate tolerance.

## Connections
- [[probability-statistics]] — probability theory
- [[causal-reasoning]] — causal inference with BNs
- [[graph-theory]] — DAG structure
- [[medical-diagnosis]] — clinical BNs
