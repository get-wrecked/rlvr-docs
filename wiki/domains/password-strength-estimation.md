---
domain: Password Strength Estimation
category: Security
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Password Strength Estimation

## Overview
Estimate password strength (entropy, crack time). Verify against zxcvbn or NIST guidelines.

## Verification Mechanism
Compute entropy / estimated crack time using zxcvbn library, compare to predicted strength rating.

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
[[cryptography-challenges]], [[encryption-decryption]]
