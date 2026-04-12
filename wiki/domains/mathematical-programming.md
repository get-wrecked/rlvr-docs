---
domain: Mathematical Programming (LP/IP/QP/SDP)
category: optimization
verification_type: execution
dataset_scale: 100K+ (MIPLIB, QPLIB, SDPLIB)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Mathematical Programming (LP/IP/QP/SDP)

## Overview
Formulate and solve mathematical optimization problems: linear programming (LP), integer programming (IP/MIP), quadratic programming (QP), semidefinite programming (SDP), convex optimization. Verification: check feasibility (constraints satisfied) and optimality (dual gap, or compare to known optimal).

## Verification Mechanism
```python
import cvxpy as cp
import numpy as np

def verify(problem_type: str, problem: dict, solution: dict) -> float:
    # Check feasibility
    for constraint in problem["constraints"]:
        if not evaluate_constraint(constraint, solution["variables"]):
            return 0.0
    
    # Compute objective value
    obj_value = evaluate_objective(problem["objective"], solution["variables"])
    
    if "optimal_value" in problem:
        # Compare to known optimal
        gap = abs(obj_value - problem["optimal_value"]) / max(abs(problem["optimal_value"]), 1e-10)
        if problem_type in ["lp", "qp", "sdp"]:
            return 1.0 if gap < 0.001 else max(0, 1 - gap)
        elif problem_type in ["ip", "mip"]:
            return 1.0 if gap < 0.01 else max(0, 1 - gap)
    
    # If no known optimal, use dual bound if available
    if "dual_bound" in solution:
        primal = obj_value
        dual = solution["dual_bound"]
        gap = abs(primal - dual) / max(abs(primal), 1e-10)
        return 1.0 if gap < 0.01 else max(0, 1 - gap)
    
    # Feasible but optimality unknown
    return 0.5

def verify_formulation(nl_description: str, formulation: str, 
                       reference_formulation: str) -> float:
    """Verify that the formulation correctly models the problem."""
    # Solve both formulations, compare optimal values
    ref_result = solve(reference_formulation)
    proposed_result = solve(formulation)
    
    if ref_result.optimal_value is None or proposed_result.optimal_value is None:
        return 0.0
    
    gap = abs(ref_result.optimal_value - proposed_result.optimal_value)
    return 1.0 if gap < 0.001 else 0.0
```

## Dataset Sources
- **MIPLIB**: 1000+ integer programming benchmark instances.
- **NETLIB LP**: Standard LP benchmark collection.
- **QPLIB**: Quadratic programming library.
- **SDPLIB**: Semidefinite programming library.
- **CVX examples**: Convex optimization examples (Boyd & Vandenberghe).
- **OR-Library**: Operations research benchmarks.
- **Textbook exercises**: Boyd & Vandenberghe, Bertsimas & Tsitsiklis.
- **Procedural generation**: Generate random LP/IP instances with controlled properties.

## Task Format
- **Input**: "A factory produces two products A and B. Product A requires 2 hours of labor and 1 kg of material... Maximize profit."
- **Output**: LP formulation + optimal solution (x_A = 30, x_B = 20, profit = 340)

## Difficulty Curriculum
- Level 1: 2-variable LP (graphical method)
- Level 3: Standard LP/QP with 10-50 variables
- Level 5: Integer programming, branch and bound
- Level 7: Large-scale LP/MIP (1000+ variables)
- Level 9: SDP, robust optimization, stochastic programming

## Limitations & Risks
- IP/MIP are NP-hard. For large instances, accept near-optimal (within known gap).
- Formulation step (NL → math) is harder to verify than solving. Use solution comparison.
- Solver timeout issues for hard instances.

## Connections
- [[combinatorics-optimization]] — discrete optimization
- [[scheduling]] — scheduling as mathematical programming
- [[engineering-optimization]] — engineering applications
- [[economic-modeling]] — economic optimization
