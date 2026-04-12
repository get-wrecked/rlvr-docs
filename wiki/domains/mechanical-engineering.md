---
domain: Mechanical Engineering (Statics, Dynamics, Thermodynamics)
category: engineering-core
verification_type: execution
dataset_scale: 100K+ (from engineering textbooks + courses)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Mechanical Engineering (Statics, Dynamics, Thermodynamics)

## Overview
Solve core mechanical engineering problems: force analysis (statics), motion analysis (dynamics), heat transfer, thermodynamic cycles, material stress/strain. All have exact numerical answers derivable from physics laws.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: dict) -> float:
    if task_type == "statics":
        # Check equilibrium: sum of forces = 0, sum of moments = 0
        forces = answer["forces"]
        if abs(sum(f["x"] for f in forces)) > 0.01:
            return 0.0
        if abs(sum(f["y"] for f in forces)) > 0.01:
            return 0.0
        if "moments" in answer:
            if abs(sum(answer["moments"])) > 0.01:
                return 0.0
        
        # Check specific reaction forces match
        correct = compute_reactions(problem)
        for key in correct:
            if abs(answer.get(key, 0) - correct[key]) / max(abs(correct[key]), 0.001) > 0.05:
                return 0.0
        return 1.0
    
    elif task_type == "thermodynamics":
        # Verify using first/second law
        correct = solve_thermo(problem)
        for quantity in ["work", "heat", "efficiency", "entropy_change"]:
            if quantity in problem.get("asked", []):
                if abs(answer.get(quantity, 0) - correct[quantity]) / max(abs(correct[quantity]), 0.001) > 0.05:
                    return 0.0
        return 1.0
    
    elif task_type == "stress_strain":
        correct = compute_stress_strain(problem)
        return 1.0 if abs(answer["stress"] - correct["stress"]) / correct["stress"] < 0.05 else 0.0
```

## Dataset Sources
- **Engineering textbook exercises**: Meriam & Kraige (Statics/Dynamics), Cengel & Boles (Thermo).
- **FE (Fundamentals of Engineering) exam**: Thousands of practice problems.
- **MIT OCW**: 2.001, 2.003, 2.005 problem sets.
- **Engineering competitions**: Formula SAE design calculations.
- **Procedural generation**: Generate random structures/systems, compute solutions.

## Task Format
- **Input**: "A simply-supported beam of length 6m carries a uniform load of 10 kN/m. Find the maximum bending moment and its location."
- **Output**: "Maximum bending moment = 45 kN·m at midspan (3m)"

## Difficulty Curriculum
- Level 1: Free body diagrams, simple equilibrium
- Level 3: Truss analysis, beam bending
- Level 5: Thermodynamic cycles, heat exchangers
- Level 7: Dynamics with friction, vibrations
- Level 9: FEA problem setup, coupled thermo-mechanical

## Connections
- [[physics-simulation]] — physics foundations
- [[engineering-optimization]] — engineering design optimization
- [[cad-modeling]] — mechanical part design
