---
domain: Agent-Based & Discrete Event Simulation
category: computational-modeling
verification_type: execution
dataset_scale: 10K+ (from simulation textbooks + benchmarks)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Agent-Based & Discrete Event Simulation

## Overview
Build simulations (agent-based models, discrete event simulations, system dynamics models) from specifications. Verification: run the simulation, check that it produces expected statistical properties (mean, variance, distribution shape) and matches known analytical results where available.

## Verification Mechanism
```python
import numpy as np
from scipy import stats

def verify(spec: dict, simulation_code: str) -> float:
    # Run the simulation multiple times
    results = []
    for seed in range(100):
        result = execute_simulation(simulation_code, seed=seed, timeout=30)
        if result is None:
            return 0.0
        results.append(result)
    
    score = 0.0
    checks = 0
    
    # Check statistical properties
    if "expected_mean" in spec:
        checks += 1
        sample_mean = np.mean([r["output"] for r in results])
        if abs(sample_mean - spec["expected_mean"]) / spec["expected_mean"] < 0.05:
            score += 1
    
    if "expected_distribution" in spec:
        checks += 1
        values = [r["output"] for r in results]
        if spec["expected_distribution"] == "exponential":
            _, p_value = stats.kstest(values, 'expon', args=(0, np.mean(values)))
            if p_value > 0.05:
                score += 1
        elif spec["expected_distribution"] == "normal":
            _, p_value = stats.normaltest(values)
            if p_value > 0.05:
                score += 1
    
    # Check conservation laws (if applicable)
    if "conservation" in spec:
        checks += 1
        violations = check_conservation(results, spec["conservation"])
        if violations == 0:
            score += 1
    
    # Check qualitative behavior
    if "monotone_increasing" in spec:
        checks += 1
        trajectory = results[0]["trajectory"]
        if all(trajectory[i] <= trajectory[i+1] for i in range(len(trajectory)-1)):
            score += 1
    
    return score / checks if checks > 0 else 0.0
```

## Dataset Sources
- **SimPy examples**: Discrete event simulation examples with known properties.
- **Mesa examples**: Agent-based model framework with benchmark models.
- **NetLogo Models Library**: 400+ agent-based models.
- **BPMN benchmarks**: Business process simulation benchmarks.
- **Queuing theory problems**: M/M/1, M/M/c queues with analytical solutions.
- **Epidemiological models**: SIR, SEIR with known dynamics.
- **Textbook exercises**: Law & Kelton, Banks et al. — simulation textbooks.

## Task Format
- **Input**: "Simulate an M/M/1 queue with arrival rate λ=0.8 and service rate μ=1.0. Estimate the average waiting time over 10,000 customers."
- **Output**: Simulation code + numerical result

## Difficulty Curriculum
- Level 1: Single-server queue (M/M/1)
- Level 3: Multi-server systems (M/M/c), inventory models
- Level 5: Agent-based models (predator-prey, traffic)
- Level 7: Complex systems (supply chains, epidemics with interventions)
- Level 9: Full system dynamics with feedback loops

## Limitations & Risks
- Stochastic simulations require statistical testing, not exact comparison. Use hypothesis testing with appropriate confidence levels.
- Where analytical solutions exist, use them as ground truth. Otherwise, use converged simulation estimates.
- Simulation time limits may prevent convergence for complex models.

## Connections
- [[probability-statistics]] — statistical foundations
- [[mathematical-modeling]] — modeling step
- [[physics-simulation]] — continuous simulation
