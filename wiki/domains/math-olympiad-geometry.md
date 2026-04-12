---
domain: Olympiad Geometry
category: math-competition
verification_type: execution
dataset_scale: 50K+ (from competition archives)
difficulty_range: medium/hard/superhuman
modality: multimodal
status: verified
---

# Olympiad Geometry

## Overview
Solve competition geometry problems: prove properties, compute lengths/angles/areas in geometric configurations. This is the domain addressed by AlphaGeometry. Verification: numerical answer check or formal geometric proof verification.

## Verification Mechanism
```python
import sympy
from sympy.geometry import *

def verify(task_type: str, problem: dict, answer: Any) -> float:
    if task_type == "compute":
        # Check numerical answer
        correct = problem["numerical_answer"]
        if isinstance(correct, float):
            return 1.0 if abs(answer - correct) < 1e-4 else 0.0
        else:
            return 1.0 if sympy.simplify(sympy.sympify(answer) - sympy.sympify(correct)) == 0 else 0.0
    
    elif task_type == "construct":
        # Verify geometric construction using coordinate geometry
        points = answer["points"]
        for constraint in problem["constraints"]:
            if not verify_geometric_constraint(points, constraint):
                return 0.0
        return 1.0
    
    elif task_type == "prove":
        # AlphaGeometry-style: verify using DD (deductive database)
        # or coordinate bashing
        statement = problem["statement"]
        # Assign random coordinates satisfying given conditions
        for _ in range(100):
            coords = random_configuration(problem["given"])
            if not evaluate_statement(statement, coords):
                return 0.0  # Counterexample found
        return 0.9  # High confidence but not formal proof
```

## Dataset Sources
- **IMO Geometry Problems**: From International Mathematical Olympiad archives.
- **AlphaGeometry benchmark (IMO-AG-30)**: 30 IMO geometry problems.
- **GeoQA**: 5K geometry QA with diagrams.
- **Geometry3K**: 3K SAT-level geometry problems.
- **UniGeo**: Geometry proving and calculation.
- **AoPS Geometry Forum**: Thousands of problems with solutions.
- **EGMO, APMO, BMO**: Regional olympiad archives.
- **Formalized Geometry**: Formalized Olympiad geometry in Lean4.

## Task Format
- **Input**: "In triangle ABC, let D be the foot of the altitude from A. If AB=13, AC=14, BC=15, find AD." (+ optional diagram)
- **Output**: "AD = 11.2" or "AD = 168/15"

## Difficulty Curriculum
- Level 1: Basic triangle computations (area, perimeter)
- Level 3: Angle chasing, similar triangles
- Level 5: Circle geometry (power of a point, radical axes)
- Level 7: Projective geometry, inversions
- Level 9: IMO-level geometry proofs

## Connections
- [[geometry-computation]] — computational geometry
- [[geometric-construction]] — compass/straightedge
- [[math-competition]] — competition math
- [[math-formal-proofs]] — formalized geometry proofs
