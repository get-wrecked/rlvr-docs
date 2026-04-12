---
domain: Code Complexity Analysis
category: Code
verification_type: exact_match
dataset_scale: unlimited (from any codebase)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Code Complexity Analysis

## Overview
Compute cyclomatic complexity, cognitive complexity, Halstead metrics, Big-O bounds for given code.

## Verification Mechanism
Compare computed metrics against tools (radon, lizard, SonarQube). For Big-O, verify against empirical timing on scaled inputs.

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
[[code-optimization]], [[code-refactoring]]
