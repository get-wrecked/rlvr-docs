---
domain: MNLI + HANS (Heuristic Analysis)
category: Language
verification_type: exact_match
dataset_scale: ~30K HANS probe
difficulty_range: hard
modality: text
status: remembered
---

# MNLI + HANS (Heuristic Analysis)

## Overview
MNLI + HANS (Heuristic Analysis). Data source recalled from training data.

## Verification Mechanism
exact_match verification.

## Dataset Sources & Reconstruction
REMEMBERED: HANS diagnostic dataset for NLI. McCoy et al. Tests lexical overlap, subsequence, constituent heuristics.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
hard

## Limitations & Risks
Data source from memory — verify before use.
