---
domain: Protein-Protein Interaction Prediction
category: Science
verification_type: exact_match
dataset_scale: ~100K+ from STRING/IntAct
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Protein-Protein Interaction Prediction

## Overview
Predict whether two proteins interact and characterize the interaction interface. Verify against experimentally validated interaction databases (STRING, IntAct, BioGRID).

## Verification Mechanism
Compare predicted interactions against gold-standard database. For interface prediction, compare predicted residues against crystal structure contacts.

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
[[protein-design]], [[biology-sequence]], [[knowledge-graph-completion]]
