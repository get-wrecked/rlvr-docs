---
domain: Mathematical Conjecture Testing & Counterexample Finding
category: math
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Mathematical Conjecture Testing & Counterexample Finding

## Overview
Given a mathematical conjecture ("For all integers n > 2, n^2 + 1 is never divisible by 4"), determine if it's true or find a counterexample. If the agent claims true, verify symbolically. If the agent provides a counterexample, verify by substitution. This develops strong mathematical intuition and computational reasoning.

## Verification Mechanism
```python
import sympy

def verify(conjecture: dict, response: dict) -> float:
    if response["verdict"] == "counterexample":
        # Plug in the counterexample and check it violates the conjecture
        values = response["values"]  # e.g., {"n": 7}
        claim = conjecture["predicate"]  # e.g., "n**2 + 1 is not divisible by 4"
        result = evaluate_predicate(claim, values)
        if result == False:  # Counterexample genuinely violates the claim
            return 1.0
        else:
            return 0.0  # Not actually a counterexample
    
    elif response["verdict"] == "true":
        # If we have a known answer, check agreement
        if conjecture["ground_truth"] == "true":
            return 1.0
        elif conjecture["ground_truth"] == "false":
            return 0.0  # Claimed true but there exists a counterexample
        else:
            # Unknown — partial credit for proof attempt
            return 0.5
    
    return 0.0

def evaluate_predicate(pred_str: str, values: dict) -> bool:
    """Evaluate the predicate with given values using SymPy."""
    expr = sympy.sympify(pred_str)
    return bool(expr.subs(values))
```

## Dataset Sources
- **Procedural generation**: Generate random conjectures, use CAS to determine truth/find counterexamples.
  - "For all n in [1, 1000], P(n)" where P is a random predicate
  - Mix of true and false conjectures (with controlled counterexample difficulty)
- **OEIS (Online Encyclopedia of Integer Sequences)**: Mining for conjectures about sequences.
- **MathOverflow/Math StackExchange**: Questions asking "Is this true?" with verified answers.
- **Counterexample collections**: Books like "Counterexamples in Analysis" by Gelbaum & Olmsted.

## Task Format
- **Input**: "Conjecture: For all prime p > 3, p^2 - 1 is divisible by 24. Is this true? If false, provide a counterexample."
- **Output**: "True" or "False, counterexample: p = ..."

## Difficulty Curriculum
- Level 1: Simple arithmetic conjectures (counterexample findable by small enumeration)
- Level 3: Number theory conjectures requiring insight
- Level 5: Conjectures requiring proof by induction/contradiction
- Level 7: Open-form conjectures from competition mathematics
- Level 9: Conjectures connected to unsolved problems

## Limitations & Risks
- For "true" conjectures, full verification requires a proof (hard to check automatically in NL form). Workaround: only give full credit for counterexample-finding tasks, or require formal proofs.
- For "false" conjectures, verification is trivial (just evaluate the counterexample).
- Asymmetry: it's easy to verify "false + counterexample" but hard to verify "true + proof." Lean toward tasks with known counterexamples.

## Connections
- [[math-competition]] — competition math often involves conjecture testing
- [[math-formal-proofs]] — formal proofs can verify "true" claims
- [[number-theory-computation]] — computational number theory is key for counterexample search
