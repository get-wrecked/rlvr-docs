---
domain: Matrix Decomposition
category: Math
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Matrix Decomposition

## Overview
Compute LU, QR, SVD, Cholesky, eigendecompositions. Verify by reconstruction: A = L*U, A = Q*R, etc.

## Verification Mechanism
Reconstruct original matrix from decomposition factors, verify ||A - reconstruct||_F < tolerance.

## Dataset Sources
See wiki for specific URLs and download instructions.

## Task Format
**Input**: Problem specification
**Output**: Solution in appropriate format

## Difficulty Curriculum
Scales from basic to expert-level within the domain.

## Limitations & Risks
Domain-specific edge cases may require careful handling.

## Connections
[[linear-algebra-computation]], [[tensor-computation]]
