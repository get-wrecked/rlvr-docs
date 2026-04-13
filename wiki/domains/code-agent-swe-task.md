---
domain: Code Agent Task Completion (SWE-agent style)
category: Agent
verification_type: execution
dataset_scale: ~2K+ from SWE-bench
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# Code Agent Task Completion (SWE-agent style)

## Overview
Act as a coding agent: read issue descriptions, navigate codebases, edit files, run tests, and iterate until the issue is resolved. This is the full loop: understand problem → find relevant code → make edits → verify. SWE-agent, Devin, and similar systems target this.

## Verification Mechanism
The repo's test suite must pass after the agent's edits. Specifically, the tests related to the issue must go from failing to passing, with no regressions.

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
[[repository-level-coding]], [[code-repair]], [[computer-use]]
