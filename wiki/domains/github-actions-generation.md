---
domain: GitHub Actions Workflow Generation
category: Code
verification_type: constraint
dataset_scale: ~1M+ workflows on GitHub
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# GitHub Actions Workflow Generation

## Overview
Generate GitHub Actions CI/CD workflows. Verify YAML schema validity and logical correctness.

## Verification Mechanism
Validate against GitHub Actions schema. Check job dependencies form valid DAG. Verify trigger conditions.

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
[[cicd-pipeline]], [[build-configuration]]
