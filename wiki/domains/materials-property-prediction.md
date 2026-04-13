---
domain: Materials Property Prediction
category: Science
verification_type: exact_match
dataset_scale: ~130K+ from Materials Project
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Materials Property Prediction

## Overview
Predict material properties (band gap, formation energy, bulk modulus, stability) from composition or structure. The Materials Project provides DFT-verified reference values for 130K+ materials.

## Verification Mechanism
Compare predicted property values against DFT reference from Materials Project. MAE/RMSE against validated values.

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
[[materials-science]], [[crystal-structure-prediction]]
