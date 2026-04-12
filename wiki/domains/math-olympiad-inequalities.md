---
domain: Mathematical Inequalities
category: math-competition
verification_type: execution
dataset_scale: 50K+ (from competition archives + textbooks)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Mathematical Inequalities

## Overview
Prove or find tight bounds for mathematical inequalities. For finding bounds: given an expression, determine its minimum/maximum value. Verification: compute the value at the proposed extremum and verify it actually achieves the bound, plus verify the inequality holds at random test points.

## Verification Mechanism
```python
import sympy
import numpy as np

def verify(task_type: str, problem: dict, answer: dict) -> float:
    if task_type == "find_minimum":
        expr = sympy.sympify(problem["expression"])
        variables = problem["variables"]
        constraints = problem.get("constraints", [])
        
        # Check proposed minimum value
        claimed_min = answer["minimum"]
        achieving_point = answer["achieving_point"]
        
        # Verify the value at the proposed point
        value_at_point = float(expr.subs(achieving_point))
        if abs(value_at_point - claimed_min) > 1e-6:
            return 0.0
        
        # Verify constraints are satisfied
        for c in constraints:
            if not eval_constraint(c, achieving_point):
                return 0.0
        
        # Statistical check: verify expr >= claimed_min at random points
        violations = 0
        for _ in range(10000):
            random_point = sample_feasible_point(variables, constraints)
            if random_point is not None:
                random_value = float(expr.subs(random_point))
                if random_value < claimed_min - 1e-6:
                    violations += 1
        
        if violations > 0:
            return 0.0  # Not actually the minimum
        return 1.0
    
    elif task_type == "prove_inequality":
        # For "prove a >= b" tasks, verify at many points
        lhs = sympy.sympify(problem["lhs"])
        rhs = sympy.sympify(problem["rhs"])
        diff = lhs - rhs
        
        # Check if agent found correct equality condition
        if "equality_point" in answer:
            equality_val = float(diff.subs(answer["equality_point"]))
            if abs(equality_val) > 1e-6:
                return 0.5  # Wrong equality condition
        
        # Statistical verification
        for _ in range(10000):
            point = sample_random_point(problem["variables"], problem.get("domain"))
            if float(diff.subs(point)) < -1e-6:
                return 0.0  # Inequality violated
        
        return 1.0
```

## Dataset Sources
- **Olympiad inequality collections**: Thousands of competition inequalities.
- **AoPS inequality forum**: Massive collection with solutions.
- **Titu Andreescu's books**: Competition inequality textbooks.
- **Journal of Inequalities and Applications**: Research-level inequalities.
- **Procedural generation**: Generate random polynomial/transcendental expressions, use optimization to find bounds.

## Task Format
- **Input**: "Find the minimum value of x^2 + y^2 + z^2 subject to x + y + z = 1, x,y,z > 0"
- **Output**: "Minimum is 1/3, achieved at x=y=z=1/3"

## Difficulty Curriculum
- Level 1: AM-GM direct application
- Level 3: Cauchy-Schwarz, Power Mean
- Level 5: Schur, Muirhead, SOS decomposition
- Level 7: Olympiad-level multi-variable inequalities
- Level 9: Open problems in inequality theory

## Limitations & Risks
- Statistical testing can miss very tight failures. Use many samples + verify at algebraic extrema.
- Symbolic solvers (SymPy) can verify simple cases exactly. Use for ground truth when possible.
- For high dimensions, sampling may not explore the full space. Use optimization-based verification.

## Connections
- [[math-competition]] — competition math
- [[combinatorics-optimization]] — optimization connection
- [[math-symbolic]] — symbolic manipulation for proofs
