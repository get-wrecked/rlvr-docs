---
domain: Payroll & Tax Withholding Computation
category: Expert
verification_type: exact_match
dataset_scale: ~1K+ from IRS pub 15-T scenarios
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Payroll & Tax Withholding Computation

## Overview
Compute gross-to-net pay including federal/state taxes, FICA, deductions, benefits.

## Verification Mechanism
Verify each withholding against IRS/state tax tables. Verify net = gross - all deductions.

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
[[tax-computation]], [[accounting-bookkeeping]]
