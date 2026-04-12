---
domain: Numerical Stability Fixing
category: Code & Software
verification_type: execution
dataset_scale: ~1K+ from Herbie + FPBench
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Numerical Stability Fixing

## Overview
Given numerically unstable code (floating-point operations that lose precision), produce a stable version. Verification against high-precision reference implementations (mpmath/BigDecimal).

## Verification Mechanism
1. Run both versions on adversarial inputs (very large, very small, near-cancellation)
2. Compare against high-precision reference (mpmath arbitrary precision)
3. Stable version must maintain bounded relative error where original fails
4. Functional equivalence on normal inputs

## Dataset Sources
- Herbie benchmark (floating-point expression improvement)
- FPBench (IEEE 754 benchmarks)
- NumPy/SciPy numerical stability bug reports
- Numerical recipes test cases

## Task Format
**Input**: Numerically unstable code + failing test cases
**Output**: Numerically stable equivalent code

## Difficulty Curriculum
1. Simple reordering (Kahan summation)
2. Log-sum-exp trick
3. Numerically stable matrix operations (Cholesky vs. LU)
4. Catastrophic cancellation avoidance
5. Mixed-precision algorithms

## Connections
[[code-optimization]], [[code-repair]], [[math-numerical]]
