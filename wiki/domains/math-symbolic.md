---
domain: Symbolic Mathematics
category: Math
verification_type: execution
dataset_scale: ~500K+ (unlimited via procedural generation)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

## Overview

Symbolic mathematics involves manipulation of mathematical expressions — simplification, integration, differentiation, solving equations, series expansion, factoring, etc. — where the answer is an expression rather than a single number. The key RLVR advantage: Computer Algebra Systems (CAS) like SymPy, Mathematica, or SageMath can verify answers by checking symbolic equivalence, making verification automatic and reliable.

This domain is partially battle-tested. While symbolic math appears in AMPS and various benchmarks, dedicated RLVR training on symbolic manipulation (as opposed to numerical computation) has received less attention than competition math. However, the verification infrastructure is mature and the dataset generation potential is enormous.

## Verification Mechanism

**Primary method: CAS-based symbolic equivalence checking.**

The core challenge: two expressions can look completely different but be mathematically identical (e.g., `sin(x)^2 + cos(x)^2` vs `1`). CAS handles this.

```python
import sympy as sp

def verify_symbolic(problem_type: str, model_answer: str, gold_answer: str, 
                    variables: list = None) -> float:
    """
    Verify symbolic math answers using SymPy.
    
    Args:
        problem_type: "simplify", "integrate", "solve", "differentiate", "factor", etc.
        model_answer: The model's symbolic expression (LaTeX or SymPy string)
        gold_answer: The reference answer
        variables: List of variable names used in the expression
    """
    try:
        model_expr = parse_expression(model_answer)  # LaTeX -> SymPy
        gold_expr = parse_expression(gold_answer)
    except (SyntaxError, ValueError):
        return 0.0
    
    # Method 1: Direct symbolic simplification
    diff = sp.simplify(model_expr - gold_expr)
    if diff == 0:
        return 1.0
    
    # Method 2: Expand and compare
    if sp.expand(model_expr) == sp.expand(gold_expr):
        return 1.0
    
    # Method 3: Numerical spot-checking
    # Evaluate both at multiple random points
    if variables is None:
        variables = list(model_expr.free_symbols | gold_expr.free_symbols)
    
    if variables:
        match = True
        for _ in range(20):  # 20 random test points
            point = {v: random.uniform(-10, 10) for v in variables}
            try:
                val_model = complex(model_expr.subs(point).evalf())
                val_gold = complex(gold_expr.subs(point).evalf())
                if abs(val_model - val_gold) > 1e-6 * max(1, abs(val_gold)):
                    match = False
                    break
            except (ValueError, ZeroDivisionError):
                continue  # Skip problematic points
        if match:
            return 1.0
    
    return 0.0


def verify_integration(model_answer: str, integrand: str, variable: str,
                       bounds: tuple = None) -> float:
    """
    Verify an indefinite or definite integral.
    For indefinite: differentiate the answer and check it equals the integrand.
    For definite: evaluate and compare numerically.
    """
    x = sp.Symbol(variable)
    model_expr = parse_expression(model_answer)
    integrand_expr = parse_expression(integrand)
    
    if bounds is None:
        # Indefinite integral: d/dx(answer) should equal integrand
        derivative = sp.diff(model_expr, x)
        diff = sp.simplify(derivative - integrand_expr)
        if diff == 0:
            return 1.0
        # Numerical check
        return numerical_equivalence_check(derivative, integrand_expr, [x])
    else:
        # Definite integral: evaluate and compare
        a, b = bounds
        model_val = model_expr  # should be a number
        gold_val = sp.integrate(integrand_expr, (x, a, b))
        return 1.0 if abs(float(model_val - gold_val)) < 1e-6 else 0.0


def verify_equation_solving(model_solutions: list, equation: str, variable: str) -> float:
    """
    Verify solutions to an equation by substituting back.
    """
    x = sp.Symbol(variable)
    lhs, rhs = equation.split('=')
    expr = parse_expression(lhs) - parse_expression(rhs)
    
    # Check each claimed solution
    for sol in model_solutions:
        sol_expr = parse_expression(sol)
        result = sp.simplify(expr.subs(x, sol_expr))
        if result != 0:
            return 0.0  # Wrong solution
    
    # Check completeness: model should find ALL solutions
    gold_solutions = sp.solve(expr, x)
    if len(model_solutions) < len(gold_solutions):
        return 0.5  # Partial credit: correct but incomplete
    
    return 1.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **AMPS** (Hendrycks et al., 2021) | ~5M | [github.com/hendrycks/math](https://github.com/hendrycks/math) | Mathematica-generated. Includes symbolic problems. |
| **SymbolicMathematics** (Lample & Charton, 2020) | ~100M+ | [github.com/facebookresearch/SymbolicMathematics](https://github.com/facebookresearch/SymbolicMathematics) | Integration, ODE solving. Procedurally generated. |
| **Integration benchmark** (Lample & Charton) | ~500K | Same repo | Forward (differentiate) + backward (integrate) pairs |
| **MATH** symbolic subset | ~3,000 | hendrycks/math | Algebra, precalculus involving symbolic manipulation |
| **Rubi integration rules** | ~7,000 rules | [rulebasedintegration.org](https://rulebasedintegration.org/) | Rule-based integration test suite |
| **Sympy test suite** | ~10K | [github.com/sympy/sympy](https://github.com/sympy/sympy) | Test cases for CAS operations |
| **Wolfram Problem Generator** | Unlimited | Wolfram Alpha API | Can generate problems at any difficulty |

### Procedural Generation

Symbolic math is uniquely suited to "forward generation" — create the answer first, then derive the problem:

```python
def generate_integration_problem():
    """Generate integral by differentiating a random function."""
    x = sp.Symbol('x')
    
    # Build a random antiderivative
    terms = []
    for _ in range(random.randint(1, 4)):
        func_type = random.choice(['poly', 'trig', 'exp', 'log'])
        if func_type == 'poly':
            terms.append(random.randint(-5, 5) * x**random.randint(1, 6))
        elif func_type == 'trig':
            terms.append(random.choice([sp.sin, sp.cos])(random.randint(1, 3) * x))
        elif func_type == 'exp':
            terms.append(sp.exp(random.randint(-2, 2) * x))
        elif func_type == 'log':
            terms.append(sp.log(x**2 + random.randint(1, 5)))
    
    antiderivative = sum(terms)
    integrand = sp.diff(antiderivative, x)
    
    return {
        "problem": f"Find the indefinite integral of {sp.latex(integrand)} dx",
        "answer": sp.latex(antiderivative),
        "verification": "differentiate_and_compare"
    }

def generate_simplification_problem():
    """Generate simplification by expanding a factored expression."""
    x = sp.Symbol('x')
    # Create a "nice" factored form
    factors = [(x + random.randint(-5, 5)) for _ in range(random.randint(2, 4))]
    factored = sp.Mul(*factors)
    expanded = sp.expand(factored)
    
    return {
        "problem": f"Factor the expression: {sp.latex(expanded)}",
        "answer": sp.latex(factored),
        "verification": "symbolic_equivalence"
    }
```

## Task Format

**Input**: Natural language or LaTeX problem statement.

```
Problem: Compute the indefinite integral: ∫ (3x^2 + 2x) * e^(x^3 + x^2) dx

Expected answer: e^(x^3 + x^2) + C
```

```
Problem: Simplify: (sin^2(x) + cos^2(x))^3 - 3*sin^2(x)*cos^2(x)

Expected answer: 1 - 3*sin^2(x)*cos^2(x)
(or equivalently: (1/4)*(1 + 3*cos(4x))/... depending on target form)
```

**Output**: A symbolic expression, ideally in LaTeX.

## Difficulty Curriculum

| Level | Type | Example | Scale |
|-------|------|---------|-------|
| Easy | Polynomial arithmetic, basic simplification | "Expand (x+2)(x-3)" | Unlimited |
| Medium | Integration by substitution, partial fractions | "∫ x/(x^2+1) dx" | Unlimited |
| Hard | Integration by parts, trig substitution | "∫ x^2 * e^x * sin(x) dx" | Unlimited |
| Very Hard | No closed-form antiderivatives, symbolic ODE solving | "Solve y'' + y = tan(x)" | ~10K |

## Limitations & Risks

1. **Equivalence checking is undecidable in general**: While SymPy handles common cases well, there exist expressions whose equivalence cannot be decided by any algorithm (Richardson's theorem). In practice this rarely matters, but edge cases exist.
2. **Form ambiguity**: "Simplify" is subjective. Is `2*sin(x)*cos(x)` simpler than `sin(2x)`? The problem must specify the target form, or the verifier must accept all equivalent forms.
3. **LaTeX parsing fragility**: Converting model-generated LaTeX to SymPy expressions is error-prone. Ambiguous notation (`sin^2 x` vs `sin(x^2)`) must be handled carefully.
4. **Constants of integration**: For indefinite integrals, the answer is only unique up to a constant. The verifier must handle this by differentiating and comparing, not by direct expression comparison.
5. **Limited RLVR precedent**: While the verification infrastructure is solid, there are fewer published results of RL training specifically on symbolic manipulation compared to numerical math or competition math.
6. **CAS as a crutch**: If the verifier can solve the problem (SymPy can integrate it), why do we need the model? The answer is that CAS is slow and brittle on hard problems, and cannot handle natural language input. But for easy problems this is a valid concern.

## Connections

- **math-numerical.md**: Numerical evaluation can serve as a cross-check for symbolic results.
- **math-competition.md**: Many competition problems involve symbolic manipulation as a key step.
- **math-formal-proofs.md**: Symbolic computation can be formally verified in Lean/Coq (e.g., `norm_num`, `ring` tactics do symbolic verification).
- **abstract-algebra.md**: Abstract algebra involves symbolic manipulation of group/ring elements — a more general form.
