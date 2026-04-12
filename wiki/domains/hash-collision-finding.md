---
domain: Hash Collision Finding
category: Security
verification_type: execution
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Hash Collision Finding

## Overview
Find inputs producing hash collisions for specified hash functions. Verify by computing both hashes.

## Verification Mechanism
Compute hash(input1) and hash(input2), verify they are equal and inputs are different.

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
[[cryptography-challenges]], [[proof-of-work-puzzles]]
