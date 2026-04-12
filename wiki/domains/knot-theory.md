---
domain: Knot Theory Computation
category: math-topology
verification_type: execution
dataset_scale: 10K+ (from knot databases)
difficulty_range: medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Knot Theory Computation

## Overview
Compute knot invariants: Alexander polynomial, Jones polynomial, HOMFLY polynomial, crossing number, unknotting number, classify knots. Verification: compare against known values in knot tables, or compute independently.

## Verification Mechanism
```python
# Using SnapPy or KnotInfo
import snappy

def verify(task_type: str, knot_repr: str, answer: Any) -> float:
    if task_type == "invariant":
        knot = snappy.Link(knot_repr)
        
        if answer["invariant"] == "alexander_polynomial":
            correct = knot.alexander_polynomial()
            return 1.0 if answer["value"] == str(correct) else 0.0
        
        elif answer["invariant"] == "jones_polynomial":
            correct = knot.jones_polynomial()
            return 1.0 if answer["value"] == str(correct) else 0.0
    
    elif task_type == "classification":
        knot = snappy.Link(knot_repr)
        correct_name = knot.identify()
        return 1.0 if answer == str(correct_name) else 0.0
    
    elif task_type == "crossing_number":
        # Verify claimed crossing number
        knot = snappy.Link(knot_repr)
        # Lower bound from invariants
        if answer < lower_bound_from_invariants(knot):
            return 0.0
        # Check if a diagram with that many crossings exists
        if answer == known_crossing_number(knot_repr):
            return 1.0
        return 0.5  # Unknown whether optimal
```

## Dataset Sources
- **KnotInfo**: Database of knot invariants for all knots up to 12 crossings.
- **Knot Atlas**: Comprehensive knot database.
- **SnapPy census**: Hyperbolic 3-manifold census.
- **Knot theory textbooks**: Adams, Cromwell — exercises with solutions.
- **Procedural generation**: Generate random knot diagrams.

## Task Format
- **Input**: Knot diagram (Gauss code or PD notation) + "Compute the Jones polynomial"
- **Output**: Jones polynomial expression

## Difficulty Curriculum
- Level 1: Identify unknot, compute crossing number for simple knots
- Level 3: Compute Alexander polynomial
- Level 5: Compute Jones/HOMFLY polynomial
- Level 7: Determine if two knots are equivalent
- Level 9: Novel invariant computation, open problems in knot theory

## Connections
- [[topology-computation]] — topological invariants
- [[abstract-algebra]] — algebraic invariants
- [[graph-theory]] — knot diagrams as graphs
