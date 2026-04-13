---
domain: Git Commit Message Generation
category: Code
verification_type: diff
dataset_scale: ~10M+ from open-source repos
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Git Commit Message Generation

## Overview
Generate commit messages from code diffs. Verify mentions changed files/functions and describes the change type.

## Verification Mechanism
Check message references files in diff. Verify change type (fix/feat/refactor) matches diff analysis.

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
[[code-documentation]], [[merge-conflict-resolution]]
