---
domain: Symbolic Regression
category: math-applied
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Symbolic Regression

## Overview
Given a set of (x, y) data points, find a closed-form mathematical expression f(x) that fits the data. Unlike numerical regression (which finds parameters for a fixed model), symbolic regression finds the model structure itself. Verification: evaluate the expression on data points and check closeness.

## Verification Mechanism
```python
import numpy as np
from sympy import sympify, lambdify

def verify(data_points: list[tuple], expression: str, 
           hidden_points: list[tuple], tolerance: float = 1e-4) -> float:
    try:
        expr = sympify(expression)
        f = lambdify('x', expr, 'numpy')
    except:
        return 0.0
    
    # Check on provided data points
    for x, y in data_points:
        try:
            predicted = float(f(x))
            if abs(predicted - y) > tolerance:
                return 0.0
        except:
            return 0.0
    
    # Check on hidden test points (generalization)
    correct = 0
    for x, y in hidden_points:
        try:
            predicted = float(f(x))
            if abs(predicted - y) <= tolerance:
                correct += 1
        except:
            pass
    
    generalization = correct / len(hidden_points)
    
    # Bonus for simplicity (shorter expressions preferred)
    complexity = expression_complexity(expr)
    simplicity_bonus = max(0, 1 - complexity / 50) * 0.2
    
    return 0.8 * generalization + simplicity_bonus
```

## Dataset Sources
- **Feynman Symbolic Regression Database**: 100 equations from the Feynman Lectures on Physics. Ground truth equations are known — test if the model can rediscover them.
- **SRBench**: Symbolic regression benchmark with 252 datasets.
- **Penn Machine Learning Benchmarks**: Regression datasets that may have underlying closed-form expressions.
- **Procedural generation**: Sample random expressions, evaluate at random points, use as tasks. Control difficulty by expression complexity.
- **Physical law discovery**: Data from physics experiments where the underlying law is known.

## Task Format
- **Input**: Table of (x, y) pairs (or multivariate) + "Find a mathematical expression f(x) that fits this data"
- **Output**: Mathematical expression (e.g., "3*x**2 + sin(x) - 1")

## Difficulty Curriculum
- Level 1: Linear and polynomial relationships (y = ax + b)
- Level 3: Trigonometric and exponential (y = A*sin(Bx) + C)
- Level 5: Compositions and products (y = x * exp(-x^2))
- Level 7: Multi-variable expressions (z = x^2*y + sin(x*y))
- Level 9: Implicit equations, differential equation solutions
- Level 10: Rediscovering physical laws from raw experimental data

## Limitations & Risks
- Multiple expressions may fit the data. The held-out test points help disambiguate but don't guarantee uniqueness.
- Tolerance threshold matters — too tight rejects correct expressions with numerical noise, too loose accepts overfitting.
- Complexity regularization is somewhat subjective (what counts as "simpler"?).
- Very well-suited for RLVR because it combines pattern recognition, mathematical knowledge, and creative search.

## Connections
- [[math-symbolic]] — symbolic computation
- [[physics-simulation]] — discovering physical laws
- [[program-synthesis-from-io]] — closely related (find program from I/O, here find expression from data)
