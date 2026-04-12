---
domain: Combinatorial Enumeration
category: math-combinatorics
verification_type: exact_match
dataset_scale: 360K+ sequences (OEIS), unlimited procedural
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Combinatorial Enumeration

## Overview

Combinatorial enumeration counts the number of discrete objects satisfying given constraints: permutations, combinations, partitions, lattice paths, graph colorings, tilings, Catalan structures, etc. The answer is always a non-negative integer, making verification trivially exact. This is one of the purest RLVR domains: compute a number, check if it is right.

The domain is rich because counting problems span an enormous range of difficulty, from simple "n choose k" calculations to open research problems. Verification is perfect (integer equality), and problems can be generated procedurally with known closed-form solutions or verified computationally for small parameters.

## Verification Mechanism

```python
def verify_enumeration(predicted: int, gold: int) -> dict:
    """Exact integer match -- the simplest possible verification."""
    correct = (predicted == gold)
    return {
        "exact_match": correct,
        "predicted": predicted,
        "gold": gold,
        "reward": 1.0 if correct else 0.0
    }

def verify_enumeration_with_formula(predicted: int, n: int,
                                      formula_fn: callable) -> dict:
    """Verify against a known closed-form formula."""
    gold = formula_fn(n)
    correct = (predicted == gold)
    return {
        "exact_match": correct,
        "gold": gold,
        "reward": 1.0 if correct else 0.0
    }

# Common combinatorial formulas for verification
from math import comb, factorial

def catalan(n):
    return comb(2*n, n) // (n + 1)

def bell(n):
    """Bell numbers via triangle."""
    if n == 0: return 1
    tri = [[0] * (n + 1) for _ in range(n + 1)]
    tri[0][0] = 1
    for i in range(1, n + 1):
        tri[i][0] = tri[i-1][i-1]
        for j in range(1, i + 1):
            tri[i][j] = tri[i][j-1] + tri[i-1][j-1]
    return tri[n][0]

def stirling2(n, k):
    """Stirling numbers of the second kind."""
    if n == 0 and k == 0: return 1
    if n == 0 or k == 0: return 0
    return k * stirling2(n-1, k) + stirling2(n-1, k-1)

def derangements(n):
    """Number of derangements (subfactorial)."""
    if n == 0: return 1
    if n == 1: return 0
    return (n - 1) * (derangements(n-1) + derangements(n-2))
```

## Dataset Sources

- **OEIS (On-Line Encyclopedia of Integer Sequences)**: 360,000+ integer sequences with formulas, references, and programs. The definitive resource for combinatorial sequences. Each entry provides generating functions, recurrences, and computed terms.
- **Competition mathematics**: AMC, AIME, Putnam, IMO problems frequently involve counting. Problems with known solutions from competitions dating back decades.
- **Project Euler**: Many problems involve combinatorial enumeration (e.g., "count lattice paths", "count partitions"). Solutions are verified via the platform.
- **Textbook problems**: Brualdi, Stanley, van Lint & Wilson, and other combinatorics textbooks provide hundreds of graded problems.
- **Procedural generation**: Given a counting formula (binomial coefficients, Catalan numbers, Stirling numbers, etc.), generate unlimited instances by varying parameters.
- **MATH dataset (Hendrycks et al.)**: Contains a combinatorics/counting section with ~1,000 competition-level problems.

## Task Format

- **Input**: A counting problem in natural language.
```
How many ways can 8 people be seated around a circular table,
where two seatings are considered the same if one is a rotation of the other?
```
- **Output**: An integer.
```
5040

Explanation: Circular permutations of n objects = (n-1)! = 7! = 5040
```

Another example:
```
How many lattice paths from (0,0) to (6,4) using only unit steps
right (R) and up (U), that never go above the line y = (2/3)x?
```

## Difficulty Curriculum

- Level 1: Direct application of permutations and combinations (n!, C(n,k))
- Level 2: Permutations with repetition, multiset combinations
- Level 3: Circular permutations, necklaces, basic inclusion-exclusion
- Level 4: Derangements, Stirling numbers, partitions of small numbers
- Level 5: Catalan number problems (ballot problem, parenthesization, binary trees)
- Level 6: Burnside's lemma / Polya enumeration (counting under group actions)
- Level 7: Generating function techniques (extracting coefficients)
- Level 8: Advanced inclusion-exclusion, Mobius inversion, permanent computation
- Level 9: Open combinatorial problems, asymptotic enumeration, counting for large parameters requiring modular arithmetic

## Limitations & Risks

- **Large numbers**: Combinatorial answers grow exponentially. For large parameters, answers may have hundreds of digits. The model must handle big integers correctly.
- **Modular arithmetic**: Competition problems often ask for answers modulo 10^9+7. This adds a layer of computation that can introduce errors.
- **Problem parsing**: Natural language counting problems are notoriously ambiguous. "How many ways to arrange..." may or may not care about order, depending on context. The problem must be precisely stated.
- **Overcount / undercount**: Counting errors come from missing cases or double-counting. These are fundamentally reasoning errors that cannot be caught by the verifier until the final answer is checked.
- **Computational verification limits**: For problems without known closed forms, verification requires brute-force enumeration, which is only feasible for small parameters (n < 20-30 typically).
- **Reward sparsity**: Since the answer is a single integer, there is no partial credit. The model either gets it right or wrong. This makes learning difficult for hard problems.

## Connections

- [[recurrence-relations]] — Many counting problems reduce to solving recurrences (Catalan, Fibonacci, etc.)
- [[abstract-algebra]] — Burnside/Polya enumeration requires group theory
- [[competitive-programming]] — Competition math overlaps heavily with combinatorial enumeration
- [[probability-statistics]] — Counting is the foundation of discrete probability
