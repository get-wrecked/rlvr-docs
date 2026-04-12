---
domain: Chemical Equation Balancing
category: science-chemistry
verification_type: rule
dataset_scale: unlimited (procedural generation)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Chemical Equation Balancing

## Overview

Chemical equation balancing finds integer stoichiometric coefficients such that the number of atoms of each element is equal on both sides of a reaction equation. For example: `Fe + O2 -> Fe2O3` becomes `4Fe + 3O2 -> 2Fe2O3`. This is a core chemistry skill with perfect verifiability: a balanced equation either conserves all atoms or it does not.

RLVR applicability is excellent. Verification is deterministic (count atoms on each side), the output is a short list of integers, training data can be generated procedurally in unlimited quantities, and difficulty scales naturally from simple to complex. This is one of the cleanest verification domains in all of RLVR.

## Verification Mechanism

```python
import re
from collections import Counter

def parse_formula(formula: str) -> Counter:
    """Parse a chemical formula into element counts. e.g., 'Fe2O3' -> {'Fe': 2, 'O': 3}"""
    elements = Counter()
    # Handle parenthesized groups recursively
    pattern = r'([A-Z][a-z]?)(\d*)'
    for match in re.finditer(pattern, formula):
        elem, count = match.groups()
        if elem:
            elements[elem] += int(count) if count else 1
    return elements

def count_atoms(side: str, coefficients: list) -> Counter:
    """Count total atoms on one side of the equation."""
    total = Counter()
    compounds = [c.strip() for c in side.split('+')]
    for compound, coeff in zip(compounds, coefficients):
        formula_counts = parse_formula(compound)
        for elem, count in formula_counts.items():
            total[elem] += count * coeff
    return total

def verify_balanced(equation: str, coefficients: list) -> dict:
    """
    equation: "Fe + O2 -> Fe2O3"
    coefficients: [4, 3, 2] (one per compound, left-to-right)
    """
    left, right = equation.split('->')
    left_compounds = [c.strip() for c in left.split('+')]
    right_compounds = [c.strip() for c in right.split('+')]

    n_left = len(left_compounds)
    left_coeffs = coefficients[:n_left]
    right_coeffs = coefficients[n_left:]

    left_atoms = count_atoms(left, left_coeffs)
    right_atoms = count_atoms(right, right_coeffs)

    balanced = left_atoms == right_atoms

    # Check coefficients are positive integers
    valid_coeffs = all(isinstance(c, int) and c > 0 for c in coefficients)

    # Check coefficients are in lowest terms (GCD = 1)
    from math import gcd
    from functools import reduce
    overall_gcd = reduce(gcd, coefficients)
    lowest_terms = (overall_gcd == 1)

    return {
        "balanced": balanced,
        "valid_coefficients": valid_coeffs,
        "lowest_terms": lowest_terms,
        "left_atoms": dict(left_atoms),
        "right_atoms": dict(right_atoms),
        "reward": 1.0 if (balanced and valid_coeffs and lowest_terms) else 0.0
    }
```

## Dataset Sources

- **Procedural generation (unlimited)**: Generate random balanced equations by:
  1. Pick a reaction template from known reaction types
  2. Assign stoichiometric coefficients
  3. Present the unbalanced equation as the problem
  This provides unlimited training data with guaranteed correct answers.
- **Textbook problem sets**: Chemistry textbooks contain curated balancing problems at various difficulty levels. Can be digitized (copyright permitting).
- **WebQC / Wolfram Alpha**: Equation balancing tools that can verify solutions. Not datasets per se, but verification oracles.
- **NIST Chemical Reaction Database**: Contains real chemical reactions that can be reformatted as balancing problems.
- **Competition chemistry problems**: Chemistry Olympiad problems include balancing as a subproblem, often for complex redox reactions.

## Task Format

- **Input**: Unbalanced chemical equation.
```
Balance this chemical equation:

Al + HCl -> AlCl3 + H2
```
- **Output**: Balanced equation with integer coefficients.
```
2Al + 6HCl -> 2AlCl3 + 3H2

Coefficients: [2, 6, 2, 3]
```

## Difficulty Curriculum

- Level 1: Simple reactions with 2-3 compounds, single element to balance (H2 + O2 -> H2O)
- Level 2: Combination/decomposition reactions with 3-4 compounds
- Level 3: Single replacement reactions (Zn + HCl -> ZnCl2 + H2)
- Level 4: Double replacement reactions with 4 compounds
- Level 5: Combustion reactions of organic compounds (CxHyOz + O2 -> CO2 + H2O)
- Level 6: Redox reactions requiring oxidation state analysis
- Level 7: Reactions with polyatomic ions and parenthesized groups (Ca(OH)2)
- Level 8: Reactions with 6+ compounds and large coefficients
- Level 9: Complex redox reactions (permanganate titrations, electrochemistry half-reactions), reactions with fractional oxidation states

## Limitations & Risks

- **Formula parsing**: Correctly parsing chemical formulas with parentheses, hydrates, and charges requires careful implementation. `Ca3(PO4)2` and `CuSO4.5H2O` need robust parsers.
- **Multiple valid solutions**: If coefficients are not required to be in lowest terms, infinitely many solutions exist (multiply all by any positive integer). Requiring lowest terms makes the answer unique.
- **Polyatomic ions**: Students are taught to balance polyatomic ions as units, but the verification only checks atom counts. Both approaches give the same result, but the reasoning path differs.
- **Not all equations are balanceable**: Some presented equations represent impossible reactions. The model should detect and flag these rather than producing nonsensical coefficients.
- **Trivial vs. interesting**: Simple balancing (adjusting one or two coefficients) can be done by brute-force enumeration without understanding chemistry. The educational value is in redox balancing, which requires understanding electron transfer.
- **Net ionic equations**: Some problems involve spectator ions and net ionic equations, adding a layer of complexity beyond simple atom counting.

## Connections

- [[forward-reaction-prediction]] — Reaction prediction must produce balanced equations; balancing is a necessary subskill
- [[chemistry-retrosynthesis]] — Retrosynthetic routes require balanced equations at each step
- [[spectrometry-interpretation]] — Mass balance connects spectral analysis to stoichiometry
- [[linear-algebra]] — Balancing is equivalent to finding the null space of the composition matrix (a linear algebra problem)
