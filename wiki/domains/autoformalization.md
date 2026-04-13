---
domain: Autoformalization (NL→Formal Math)
category: Math
verification_type: rule
dataset_scale: ~10K+ from ProofNet/MiniF2F
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# Autoformalization (NL→Formal Math)

## Overview
Translate informal mathematical statements and proofs into formal representations (Lean 4/Isabelle). ProofNet and MiniF2F provide benchmark problems. This bridges the gap between how mathematicians think and what proof assistants require.

## Verification Mechanism
Formalized statement must be parseable by Lean 4/Isabelle. Formalized proof must type-check. Semantic equivalence checked via round-trip or spot verification.

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
[[theorem-formalization]], [[math-formal-proofs]]
