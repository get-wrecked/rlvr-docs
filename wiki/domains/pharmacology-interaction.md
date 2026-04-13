---
domain: Drug Interaction Checking
category: Expert
verification_type: exact_match
dataset_scale: ~100K+ from DrugBank/FDA
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Drug Interaction Checking

## Overview
Identify drug-drug interactions and contraindications. Verify against DrugBank/FDA interaction databases.

## Verification Mechanism
Look up drug pair in reference database, verify predicted interaction matches known interaction profile.

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
[[medical-diagnosis]], [[knowledge-graph-completion]]
