---
domain: Crystal Structure Prediction
category: Science
verification_type: simulation
dataset_scale: ~200K+ from Materials Project/ICSD
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Crystal Structure Prediction

## Overview
Predict the crystal structure that a given chemical composition will adopt. This is one of the grand challenges of materials science. Verify predicted structure against known structures or DFT relaxation.

## Verification Mechanism
Compare predicted structure to known structure in database (RMSD < threshold) or verify DFT-relaxed energy is near ground state.

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
[[materials-science]], [[crystallography]], [[alloy-composition]]
