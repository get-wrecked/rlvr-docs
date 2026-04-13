---
domain: Code Smell Detection & Refactoring
category: Code
verification_type: execution
dataset_scale: ~50K+ from code smell datasets
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Code Smell Detection & Refactoring

## Overview
Identify code smells (God class, long method, feature envy) and suggest refactorings. Verify smell removal via metrics.

## Verification Mechanism
Compute code metrics before/after refactoring. Verify smell metric (class size, method length, coupling) improves. Tests still pass.

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
[[code-refactoring]], [[code-complexity-analysis]]
