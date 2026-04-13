---
domain: Terminal Command Composition
category: Agent
verification_type: execution
dataset_scale: ~10K+ from NLC2CMD/NL2Bash
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# Terminal Command Composition

## Overview
Compose multi-step terminal command pipelines from natural language. Verify output matches expected.

## Verification Mechanism
Execute command pipeline in sandbox, compare stdout to expected output.

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
[[shell-commands]], [[file-system-tasks]]
