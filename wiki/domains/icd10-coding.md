---
domain: ICD-10 Diagnosis Coding
category: Expert
verification_type: exact_match
dataset_scale: ~70K codes from ICD-10-CM
difficulty_range: hard
modality: text
status: remembered
---

# ICD-10 Diagnosis Coding

## Overview
ICD-10 Diagnosis Coding. Data source recalled from training data.

## Verification Mechanism
exact_match verification.

## Dataset Sources & Reconstruction
REMEMBERED: Map clinical descriptions to ICD-10 codes. WHO ICD-10 is public. Verify against code manual.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
hard

## Limitations & Risks
Data source from memory — verify before use.
