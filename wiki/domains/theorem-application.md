---
domain: Theorem Application & Problem Selection
category: math-reasoning
verification_type: execution
dataset_scale: 100K+ (from textbooks and courses)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Theorem Application & Problem Selection

## Overview
Given a mathematical problem, identify which theorem/technique to apply and execute it correctly. This tests the strategic reasoning aspect of mathematics — not just computation, but knowing WHAT to compute.

The verification insight: even if we can't check the reasoning strategy directly, we can check the final answer, which is determined by the strategy.

## Verification Mechanism
```python
def verify(problem: str, answer: Any, gold_answer: Any, 
           method_hint: str = None) -> float:
    # Primary: check the answer
    if answers_equivalent(answer, gold_answer):
        score = 1.0
    else:
        score = 0.0
    
    # Bonus: if method/technique identification is part of the task
    if method_hint and "identified_method" in answer:
        if answer["identified_method"].lower() == method_hint.lower():
            score = min(1.0, score + 0.2)
    
    return score

def answers_equivalent(a, b):
    """Check mathematical equivalence via CAS."""
    try:
        a_sym = sympy.sympify(a)
        b_sym = sympy.sympify(b)
        return sympy.simplify(a_sym - b_sym) == 0
    except:
        return str(a).strip() == str(b).strip()
```

## Dataset Sources
- **MATH dataset**: 12.5K problems tagged by subject area.
- **Textbook problem sets**: Organized by chapter (technique).
- **Brilliant.org exercises**: Progressive difficulty with technique tags.
- **MIT OCW problem sets**: Organized by topic/technique.
- **Procedural generation**: Generate problems that require specific techniques.

## Task Format
- **Input**: "Evaluate the integral ∫₀^∞ e^{-x²} dx"
- **Output**: "Using the Gaussian integral / polar coordinates: √π/2"

## Difficulty Curriculum
- Level 1: Direct formula application
- Level 3: Requires technique selection (substitution vs. parts vs. partial fractions)
- Level 5: Requires creative technique application (unexpected substitution)
- Level 7: Requires combining multiple techniques
- Level 9: Requires novel technique invention

## Limitations & Risks
- Checking strategy directly is hard — focus on checking answers.
- Multiple valid solution strategies may exist. Accept any that gives the right answer.

## Connections
- [[math-competition]] — competition problems require strategy
- [[math-symbolic]] — symbolic computation
- [[math-numerical]] — numerical verification of answers
