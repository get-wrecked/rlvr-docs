---
domain: Math Numerical Computation
category: Math
verification_type: exact_match
dataset_scale: ~1M+ problems (procedural generation essentially unlimited)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

## Overview

Numerical computation tasks ask the model to produce a specific number as the answer: arithmetic, algebra, calculus evaluations, linear algebra computations, etc. Unlike competition math, these problems are typically "textbook" rather than "puzzle" — they test procedural fluency rather than creative insight. The key advantage for RLVR is that verification is trivial (compare numbers) and datasets can be generated at essentially unlimited scale via templated problem generators.

This domain is extremely well-validated. GSM8K (grade-school math) was used in the original InstructGPT and later RLVR work. MATH Level 1-3 problems fall here. DeepSeek-Math, Qwen-Math, and many others train on this domain.

## Verification Mechanism

**Primary method: Numerical exact match with tolerance.**

```python
def verify_numerical(problem, model_answer, gold_answer, rtol=1e-5, atol=1e-8):
    # Step 1: Extract numeric value from model output
    extracted = extract_numeric_answer(model_answer)
    if extracted is None:
        return 0.0
    
    # Step 2: Parse gold answer
    gold_val = parse_number(gold_answer)  # handles fractions, scientific notation, etc.
    
    # Step 3: Compare
    # For integer problems (GSM8K), require exact integer match
    if is_integer_problem(problem):
        return 1.0 if int(round(extracted)) == int(gold_val) else 0.0
    
    # For real-valued problems, use relative + absolute tolerance
    if abs(extracted - gold_val) <= atol + rtol * abs(gold_val):
        return 1.0
    
    return 0.0

def extract_numeric_answer(text):
    """Extract the final numeric answer from model output."""
    # Priority 1: \boxed{...}
    boxed = re.findall(r'\\boxed\{([^}]+)\}', text)
    if boxed:
        return parse_number(boxed[-1])
    
    # Priority 2: "The answer is X"
    answer_match = re.search(r'[Tt]he (?:final )?answer is[:\s]*([0-9.,/\-]+)', text)
    if answer_match:
        return parse_number(answer_match.group(1))
    
    # Priority 3: Last number in the text
    numbers = re.findall(r'-?\d+\.?\d*', text)
    if numbers:
        return float(numbers[-1])
    
    return None
```

**For symbolic answers that resolve to numbers** (e.g., "compute sin(pi/4)"):
```python
def verify_symbolic_numeric(model_answer, gold_answer):
    # Parse both as sympy, evaluate numerically
    model_expr = sympy.sympify(extract_expression(model_answer))
    gold_expr = sympy.sympify(gold_answer)
    
    model_val = complex(model_expr.evalf())
    gold_val = complex(gold_expr.evalf())
    
    return 1.0 if abs(model_val - gold_val) < 1e-6 else 0.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **GSM8K** (Cobbe et al., 2021) | 8,792 | [github.com/openai/grade-school-math](https://github.com/openai/grade-school-math) | Grade-school word problems. Integer answers. Verified gold answers. |
| **GSM8K-Hard** | 1,319 | HuggingFace | GSM8K with larger numbers to prevent memorization |
| **MATH** (Hendrycks, 2021) Level 1-3 | ~5,000 | [github.com/hendrycks/math](https://github.com/hendrycks/math) | Algebra, precalculus, number theory — easier tiers |
| **AMPS** (Hendrycks, 2021) | ~5M | Same repo | Khan Academy + Mathematica-generated problems |
| **SVAMP** (Patel et al., 2021) | 1,000 | [github.com/arkilpatel/SVAMP](https://github.com/arkilpatel/SVAMP) | Simple word problems, robustness-focused |
| **ASDiv** (Miao et al., 2020) | 2,305 | GitHub | Diverse arithmetic word problems |
| **MultiArith** | 600 | NLP datasets | Multi-step arithmetic |
| **MathQA** (Amini et al., 2019) | 37,297 | [math-qa.github.io](https://math-qa.github.io/) | GRE/GMAT-style quantitative problems |
| **TAL-SCQ5K** | 5,000 | HuggingFace | Chinese math competition, multiple choice |
| **Procedurally generated** | Unlimited | Custom generators | Template: "Compute [operation] of [random values]" |

### Procedural Generation

The main advantage of numerical math: you can generate unlimited training data.

```python
# Example: Generate calculus evaluation problems
import sympy as sp
import random

def generate_integral_problem():
    x = sp.Symbol('x')
    # Random polynomial integrand
    degree = random.randint(1, 5)
    coeffs = [random.randint(-10, 10) for _ in range(degree + 1)]
    integrand = sum(c * x**i for i, c in enumerate(coeffs))
    
    a, b = sorted(random.sample(range(-5, 6), 2))
    answer = sp.integrate(integrand, (x, a, b))
    
    return {
        "problem": f"Evaluate the definite integral of {sp.latex(integrand)} from {a} to {b}.",
        "answer": str(answer)
    }
```

## Task Format

**Input**: Natural language math problem.

```
Problem: A store sells notebooks for $3.50 each and pens for $1.25 each. 
Maria buys 4 notebooks and 7 pens. How much does she spend in total?

Expected answer: 22.75
```

```
Problem: Compute the derivative of f(x) = 3x^4 - 2x^2 + 7x - 1 at x = 2.

Expected answer: 95
```

**Output**: Chain-of-thought followed by the numerical answer.

## Difficulty Curriculum

| Level | Type | Example | Scale |
|-------|------|---------|-------|
| Easy | Single-step arithmetic | "What is 347 + 289?" | Unlimited |
| Medium | Multi-step word problems | GSM8K-style | ~10K+ |
| Medium-Hard | Algebra, basic calculus | MATH Level 2-3 | ~5K |
| Hard | Multi-variable calculus, linear algebra | University-level computation | ~2K (but gen. unlimited) |

Curriculum strategy: start with single-step arithmetic and GSM8K, progress to multi-step problems, then to calculus/linear algebra computations.

## Limitations & Risks

1. **Ceiling effect**: Current frontier models (GPT-4, Claude) already achieve >95% on GSM8K. The easy end of this domain is nearly saturated. RLVR gains are marginal unless you use harder problems.
2. **Procedural ≠ understanding**: Models can learn computational shortcuts without genuine mathematical understanding. A model that can compute integrals may not understand what an integral means.
3. **Floating-point edge cases**: For real-valued answers, the tolerance threshold matters. Too tight rejects correct answers with rounding differences. Too loose accepts wrong answers.
4. **Answer format variance**: "22.75" vs "$22.75" vs "22 dollars and 75 cents" vs "22.750" — the extraction function must handle all of these.
5. **Reward hacking on easy problems**: If the curriculum includes too many trivial problems, the model learns to spend minimal reasoning tokens and rush to an answer — which then fails on harder problems.

## Connections

- **math-competition.md**: Competition math is the "hard mode" of numerical math. Many competition problems have numerical answers but require creative insight, not just computation.
- **math-symbolic.md**: Symbolic math produces expressions, not numbers. But evaluating a symbolic result numerically can serve as a cross-check.
- **probability-statistics.md**: Many probability problems have exact numerical answers (expected values, probabilities as fractions).
- **number-theory-computation.md**: Number theory computations (GCD, modular arithmetic) are a specialized subset.
