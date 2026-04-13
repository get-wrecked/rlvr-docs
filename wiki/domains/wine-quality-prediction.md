---
domain: Wine / Food Quality Prediction
category: Expert
verification_type: exact_match
dataset_scale: ~6K+ from UCI Wine Quality dataset
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Wine / Food Quality Prediction

## Overview
Predict wine quality scores from chemical properties (acidity, sugar, sulfur, alcohol). Verify against expert ratings.

## Verification Mechanism
Compare predicted quality score to gold expert rating. Compute RMSE or accuracy within ±1.

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
[[data-science-eda]], [[chemistry-computation]]
