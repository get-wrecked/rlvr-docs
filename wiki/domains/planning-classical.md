---
domain: Classical AI Planning (STRIPS/PDDL)
category: reasoning
verification_type: simulation
dataset_scale: 10K+ benchmarks, unlimited procedural
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Classical AI Planning (STRIPS/PDDL)

## Overview
Given an initial state, goal state, and available actions (in PDDL format), find a valid plan (action sequence) that reaches the goal. This is a foundational AI task — planning under deterministic, fully observable conditions. PDDL (Planning Domain Definition Language) provides a standardized format, and plan validators exist.

## Verification Mechanism
```python
def verify(domain_pddl: str, problem_pddl: str, plan: list[str]) -> float:
    # Use VAL (the standard PDDL plan validator)
    result = run_val_validator(domain_pddl, problem_pddl, plan)
    if not result.valid:
        return 0.0
    
    # Optional: reward shorter plans
    optimal_length = get_optimal_plan_length(domain_pddl, problem_pddl)  # may be expensive
    if optimal_length:
        return min(1.0, optimal_length / len(plan))  # reward efficiency
    return 1.0
```

VAL is the standard plan validator used in the International Planning Competition (IPC). It rigorously checks preconditions and effects.

## Dataset Sources
- **International Planning Competition (IPC)**: Decades of benchmark domains (Blocksworld, Logistics, Rovers, Satellite, etc.). Hundreds of instances per domain.
- **planning.domains**: Repository of PDDL domains and problems.
- **FF benchmark suite**: Classical planning benchmarks.
- **Procedural generation**: Generate random PDDL instances by sampling initial/goal states. Control difficulty by plan length.

## Task Format
- **Input**: PDDL domain + problem file (or natural language description)
- **Output**: Sequence of grounded actions

## Difficulty Curriculum
- Level 1: Blocksworld with 3 blocks
- Level 3: Logistics with 5 packages
- Level 5: Multi-agent coordination, resource constraints
- Level 7: Large-scale planning (50+ objects)
- Level 9: Temporal planning, numeric fluents
- Level 10: Probabilistic/partial observability (extends beyond classical)

## Limitations & Risks
- Plan validation is fast, but finding optimal plans is PSPACE-hard. Use satisficing (valid but not necessarily optimal) as base, optimality as bonus.
- PDDL is a specific formalism — transfer to natural language planning is non-trivial but valuable.
- Some planning domains have exponentially many valid plans — reward optimization incentivizes efficiency.

## Connections
- [[robotics-planning]] — physical instantiation of planning
- [[puzzle-games]] — many puzzles are planning problems
- [[combinatorics-optimization]] — planning as search in combinatorial spaces
