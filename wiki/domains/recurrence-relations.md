---
domain: Recurrence Relations
category: math-discrete
verification_type: exact_match
dataset_scale: 360K sequences (OEIS), unlimited procedural
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Recurrence Relations

## Overview

Recurrence relation tasks involve analyzing, solving, and computing with sequences defined by recursive formulas. Given a recurrence like `a(n) = 2a(n-1) + a(n-2)` with initial conditions, the model must find closed-form solutions, compute specific terms, identify the generating function, or determine asymptotic behavior.

RLVR verification is robust: the model's closed-form solution or computed terms can be checked by evaluating both the recurrence and the proposed solution for many values of n. If they agree for (say) 100 terms, the solution is almost certainly correct. For integer sequences, exact match verification is trivial.

## Verification Mechanism

```python
def verify_closed_form(recurrence_fn, closed_form_fn, initial_conditions: dict,
                        n_terms: int = 100) -> dict:
    """
    Verify a closed-form solution by comparing against the recurrence for many terms.
    recurrence_fn: function(n, memo) -> a(n) computed recursively
    closed_form_fn: function(n) -> a(n) computed via closed form
    initial_conditions: {0: val0, 1: val1, ...}
    """
    memo = dict(initial_conditions)

    def compute_recurrence(n):
        if n in memo:
            return memo[n]
        memo[n] = recurrence_fn(n, memo)
        return memo[n]

    mismatches = 0
    for n in range(n_terms):
        rec_val = compute_recurrence(n)
        closed_val = closed_form_fn(n)
        # Allow floating-point tolerance for irrational closed forms
        if isinstance(closed_val, float):
            if abs(rec_val - closed_val) > 1e-6 * max(1, abs(rec_val)):
                mismatches += 1
        else:
            if rec_val != closed_val:
                mismatches += 1

    all_match = (mismatches == 0)

    return {
        "terms_checked": n_terms,
        "mismatches": mismatches,
        "all_match": all_match,
        "reward": 1.0 if all_match else 0.0
    }

def verify_nth_term(predicted: int, n: int, recurrence_fn, 
                     initial_conditions: dict) -> dict:
    """Verify a specific term of a recurrence."""
    memo = dict(initial_conditions)
    def compute(k):
        if k in memo:
            return memo[k]
        memo[k] = recurrence_fn(k, memo)
        return memo[k]

    gold = compute(n)
    correct = (predicted == gold)
    return {"exact_match": correct, "gold": gold, "reward": 1.0 if correct else 0.0}

# Example: Fibonacci
def fib_recurrence(n, memo):
    return memo[n-1] + memo[n-2]

def fib_closed(n):
    """Binet's formula."""
    phi = (1 + 5**0.5) / 2
    psi = (1 - 5**0.5) / 2
    return round((phi**n - psi**n) / 5**0.5)
```

## Dataset Sources

- **OEIS (On-Line Encyclopedia of Integer Sequences)**: 360,000+ sequences, many defined by recurrences. Each entry includes the recurrence formula, closed form (if known), generating function, and computed terms. The definitive resource.
- **Textbook problems**: Concrete Mathematics (Graham, Knuth, Patashnik), Generatingfunctionology (Wilf), and discrete mathematics textbooks provide hundreds of solved recurrence problems.
- **Competition mathematics**: AMC/AIME/Putnam problems frequently involve recurrence analysis.
- **Procedural generation (linear recurrences)**: Generate random linear recurrences `a(n) = c1*a(n-1) + c2*a(n-2) + ... + ck*a(n-k)` with random integer coefficients. Closed-form solutions can be computed via characteristic polynomial roots. Unlimited supply.
- **MATH dataset**: Contains problems involving sequences and series, including recurrence-based problems.

## Task Format

- **Input**: A recurrence relation with initial conditions and a question.
```
Given the recurrence:
  a(0) = 1, a(1) = 1
  a(n) = a(n-1) + 2*a(n-2) for n >= 2

Find a closed-form expression for a(n).
```
- **Output**: Closed-form solution.
```
a(n) = (2^(n+1) + (-1)^n) / 3

Derivation: Characteristic equation r^2 = r + 2, so r^2 - r - 2 = 0,
giving r = 2 or r = -1. General solution: a(n) = A*2^n + B*(-1)^n.
From a(0)=1: A + B = 1. From a(1)=1: 2A - B = 1.
Solving: A = 2/3, B = 1/3. So a(n) = (2^(n+1) + (-1)^n) / 3.
```

Computation task:
```
The sequence is defined by: a(1) = 3, a(n) = 2*a(n-1) + n for n > 1.
What is a(10)?
```

## Difficulty Curriculum

- Level 1: First-order linear recurrences with constant coefficients (a(n) = c*a(n-1) + d)
- Level 2: Second-order linear homogeneous (Fibonacci-type: a(n) = c1*a(n-1) + c2*a(n-2))
- Level 3: Second-order with non-homogeneous term (a(n) = c1*a(n-1) + c2*a(n-2) + f(n))
- Level 4: Higher-order linear recurrences (order 3-5)
- Level 5: Recurrences with non-constant coefficients (a(n) = n*a(n-1) + ...)
- Level 6: Divide-and-conquer recurrences (Master Theorem: T(n) = aT(n/b) + f(n))
- Level 7: Nonlinear recurrences (a(n) = a(n-1)^2 + ...), where closed forms may not exist
- Level 8: Systems of recurrences (coupled sequences), recurrences over two variables
- Level 9: Identifying the recurrence from sequence terms alone (sequence recognition), recurrences requiring generating function techniques

## Limitations & Risks

- **Floating-point verification**: Closed forms involving irrational numbers (phi, sqrt(2)) require careful numerical comparison. For large n, floating-point errors accumulate.
- **Non-existence of closed forms**: Many recurrences have no elementary closed form. The model should recognize when a closed form does not exist and provide the generating function or asymptotic formula instead.
- **Simplification ambiguity**: Multiple equivalent closed forms exist for the same sequence. `(2^n + (-1)^n*2^n)` and `2^n*(1+(-1)^n)` are the same. The verification must check numerical values, not symbolic forms.
- **Large term computation**: Computing a(10^6) for a nonlinear recurrence may require O(n) time. For very large n, only recurrences with matrix exponentiation or closed-form solutions are feasible.
- **Sequence recognition is hard**: Given just the first few terms, many different recurrences could match. OEIS helps but is not exhaustive.

## Connections

- [[combinatorial-enumeration]] — Many counting sequences satisfy recurrences (Catalan numbers, Bell numbers, etc.)
- [[competitive-programming]] — Dynamic programming problems are recurrences in disguise
- [[automated-theorem-proving]] — Proving recurrence identities can be formulated as theorem proving
- [[abstract-algebra]] — Linear recurrences connect to polynomial ring theory via characteristic polynomials
