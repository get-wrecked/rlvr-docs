---
domain: Aider-Style Code Editing
category: Code
verification_type: execution
dataset_scale: ~133 exercises from Exercism
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Aider-Style Code Editing

## Overview
Edit existing code files to implement features or fix bugs, given natural language instructions. Unlike code generation (write from scratch), this requires understanding existing code and making targeted edits. Aider's benchmark tests this with Exercism exercises.

## Verification Mechanism
Run test suite after edit. All tests must pass. Diff must be minimal (no unnecessary changes).

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
[[code-repair]], [[code-refactoring]]
