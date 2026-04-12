---
domain: Boolean Function Learning
category: theoretical-cs
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Boolean Function Learning

## Overview
Learn a Boolean function from a truth table or partial input-output examples. Express the function as a Boolean formula (DNF, CNF, decision tree, BDD). Verification: evaluate on all 2^n inputs (for small n) or on held-out examples (for larger n).

## Verification Mechanism
```python
def verify(examples: list[tuple], hidden_function: Callable, 
           formula: str, n_vars: int) -> float:
    # Parse the Boolean formula
    try:
        f = parse_boolean(formula)
    except:
        return 0.0
    
    if n_vars <= 20:
        # Exhaustive check for small functions
        correct = 0
        total = 2 ** n_vars
        for x in range(total):
            bits = [(x >> i) & 1 for i in range(n_vars)]
            if f(bits) == hidden_function(bits):
                correct += 1
        return correct / total
    else:
        # Random sampling for large functions
        correct = 0
        for _ in range(10000):
            bits = [random.randint(0, 1) for _ in range(n_vars)]
            if f(bits) == hidden_function(bits):
                correct += 1
        return correct / 10000

# Bonus: check minimality
def check_minimality(formula: str, function: Callable, n_vars: int) -> float:
    """Check if the formula is minimal (fewest terms/gates)."""
    actual_size = formula_size(formula)
    # Espresso minimization for reference
    minimal_size = espresso_minimize(function, n_vars)
    return min(1.0, minimal_size / actual_size)
```

## Dataset Sources
- **Boolean function benchmarks**: ISCAS, MCNC, Espresso benchmark circuits.
- **PAC learning datasets**: Theoretical CS learning theory benchmarks.
- **Circuit minimization benchmarks**: Logic synthesis benchmarks.
- **Procedural generation**: Generate random Boolean functions of controlled complexity. Unlimited.

## Task Format
- **Input**: Truth table: "000→0, 001→1, 010→1, 011→0, 100→1, 101→0, 110→0, 111→1. Find a Boolean formula."
- **Output**: "x0 XOR x1 XOR x2" (parity function)

## Difficulty Curriculum
- Level 1: 3-4 variables, simple AND/OR/NOT
- Level 3: 6-8 variables, requires DNF/CNF
- Level 5: 10-15 variables with structure (symmetry, threshold)
- Level 7: 20+ variables, requires pattern recognition
- Level 9: Learn from partial data with generalization guarantees

## Limitations & Risks
- Multiple equivalent formulas may represent the same function. Accept any correct one.
- Exhaustive verification is only feasible for small n. For larger n, probabilistic verification.
- Connection to computational learning theory (PAC learning, VC dimension).

## Connections
- [[logic-propositional]] — Boolean logic
- [[program-synthesis-from-io]] — synthesis from examples
- [[circuit-design]] — Boolean circuits
- [[chip-design-rtl]] — hardware implementation
