---
domain: Repository-Level Code Understanding
category: Code
verification_type: execution
dataset_scale: ~2K+ from SWE-bench/Aider
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# Repository-Level Code Understanding

## Overview
Understand and modify code across entire repositories — not single functions but multi-file changes involving understanding of architecture, APIs, test infrastructure, and codebase conventions. SWE-bench Verified requires editing real open-source repos to fix real GitHub issues. This is the gold standard for measuring real-world coding ability.

## Verification Mechanism
Run the repo's test suite. The specific failing test(s) referenced in the issue must pass after the edit, and no previously passing tests may break. Fully automated via containerized test execution.

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
[[code-repair]], [[code-generation]], [[test-generation]]
