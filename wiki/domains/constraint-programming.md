---
domain: Constraint Programming (MiniZinc/CP)
category: formal-methods
verification_type: constraint
dataset_scale: 10K+ (from MiniZinc Challenge + CSPLib)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Constraint Programming (MiniZinc/CP)

## Overview
Model and solve constraint satisfaction/optimization problems using constraint programming paradigms: write MiniZinc/OPL models, use global constraints (alldifferent, cumulative, circuit), and find optimal solutions. Distinct from SAT/SMT — CP uses high-level modeling with propagation and global constraints. Verification: check all constraints satisfied, compare objective to known optimal.

## Verification Mechanism
```python
def verify(model: str, data: str, solution: dict) -> float:
    # Parse MiniZinc model and evaluate constraints
    mzn_result = run_minizinc(model, data, solution, timeout=60)
    
    if not mzn_result.feasible:
        return 0.0  # Solution violates constraints
    
    if mzn_result.optimization:
        # Compare to known optimal
        optimal = run_minizinc_solve(model, data, timeout=300)
        if optimal.objective is not None:
            gap = abs(solution["objective"] - optimal.objective) / max(abs(optimal.objective), 1e-10)
            return max(0, 1 - gap)
    
    return 1.0  # Feasible solution for satisfaction problem
```

## Dataset Sources
- **MiniZinc Challenge**: Annual CP competition — hundreds of benchmark instances across dozens of problem types.
- **CSPLib**: 90+ standard constraint satisfaction problem types with instances.
- **OR-Library CP instances**: Scheduling, timetabling, bin packing.
- **Numberjack/Choco benchmarks**: CP solver test suites.
- **MiniZinc examples**: Official examples repository with 100+ models.
- **Procedural generation**: Generate random CSP instances with controlled tightness/density.

## Task Format
- **Input**: Problem description + "Write a MiniZinc model and find a solution"
- **Output**: MiniZinc model code + solution assignment

## Difficulty Curriculum
- Level 1: Simple constraint satisfaction (4-Queens, graph coloring with 5 nodes)
- Level 3: Medium CSPs with global constraints (alldifferent, table)
- Level 5: Optimization problems (job shop scheduling, vehicle routing)
- Level 7: Complex modeling with cumulative, circuit, reification
- Level 9: MiniZinc Challenge-level problems

## Limitations & Risks
- CP solving can be slow — verification of feasibility is fast, but finding optima is NP-hard.
- Multiple optimal solutions may exist. Accept any optimal or near-optimal.

## Connections
- [[smt-solving]] — related but different paradigm (theories vs. global constraints)
- [[scheduling]] — many scheduling problems are best modeled as CP
- [[combinatorics-optimization]] — CP is a key optimization approach
- [[logic-puzzles]] — many puzzles are naturally expressed as CSPs
