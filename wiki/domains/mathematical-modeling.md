---
domain: Mathematical Modeling (Applied)
category: math-applied
verification_type: execution
dataset_scale: 10K+ (from modeling competitions + textbooks)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Mathematical Modeling (Applied)

## Overview
Given a real-world scenario, build a mathematical model (equations, differential equations, optimization formulations) and solve it to produce a numerical answer. Verification: the answer matches known results or simulation.

Key distinction from pure math: the task includes the modeling step (choosing what to model and how), not just solving given equations.

## Verification Mechanism
```python
def verify(problem: dict, model_and_solution: dict) -> float:
    score = 0.0
    checks = 0
    
    # Check final answer
    if "expected_answer" in problem:
        checks += 1
        tolerance = problem.get("tolerance", 0.05)  # 5% relative error
        actual = model_and_solution["answer"]
        expected = problem["expected_answer"]
        if abs(actual - expected) / max(abs(expected), 1e-10) <= tolerance:
            score += 1
    
    # Check model validity (if model is provided as equations)
    if "model_equations" in model_and_solution:
        # Verify equations are dimensionally consistent
        checks += 1
        if check_dimensional_consistency(model_and_solution["model_equations"]):
            score += 1
        
        # Verify boundary conditions are satisfied
        checks += 1
        if check_boundary_conditions(model_and_solution, problem.get("boundary_conditions", [])):
            score += 1
    
    # If model includes code, execute it
    if "code" in model_and_solution:
        checks += 1
        result = execute_in_sandbox(model_and_solution["code"], timeout=60)
        if result is not None and close_enough(result, problem["expected_answer"]):
            score += 1
    
    return score / checks if checks > 0 else 0.0
```

## Dataset Sources
- **MCM/ICM (COMAP)**: Mathematical Contest in Modeling — decades of problems with solution papers.
- **PUTNAM problems**: Applied math problems from the competition.
- **MIT OpenCourseWare**: Applied math course problem sets.
- **SIAM problems**: Applied math problems from the society.
- **Textbooks**: Strogatz (Nonlinear Dynamics), Edelstein-Keshet (Mathematical Biology), etc.
- **Kaggle**: Some competitions require mathematical modeling.
- **Physics/engineering textbooks**: Word problems requiring modeling.

## Task Format
- **Input**: "A cylindrical tank of radius 2m and height 5m is filled with water. A hole of radius 1cm is drilled at the bottom. How long until the tank is empty? (Assume Torricelli's law.)"
- **Output**: Mathematical model (differential equation) + solution (numerical answer)

## Difficulty Curriculum
- Level 1: Direct application of known formulas (area, volume)
- Level 3: Single ODE models (exponential growth, Torricelli's law)
- Level 5: Systems of ODEs, optimization with constraints
- Level 7: PDE models, multi-physics coupling
- Level 9: MCM/ICM-level open-ended modeling challenges

## Limitations & Risks
- Open-ended modeling problems may have multiple valid approaches with different answers. Focus on problems with known unique answers for RLVR.
- Tolerance on numerical answers must be calibrated per problem.
- Dimensional analysis is a powerful verification check (wrong dimensions = wrong model).

## Connections
- [[physics-simulation]] — physics models
- [[engineering-optimization]] — engineering applications
- [[math-numerical]] — numerical computation
- [[probability-statistics]] — statistical modeling
