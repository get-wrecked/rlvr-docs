---
domain: ML Engineering Competition (MLE-bench)
category: ML
verification_type: execution
dataset_scale: ~75 Kaggle competitions
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# ML Engineering Competition (MLE-bench)

## Overview
Solve Kaggle-style ML competitions end-to-end: understand the problem, explore data, engineer features, train models, tune hyperparameters, generate submissions. MLE-bench packages 75 real Kaggle competitions with automated grading. Tests the full ML engineering pipeline.

## Verification Mechanism
Competition metric (AUC, RMSE, F1, etc.) on held-out test set. Score compared against medal thresholds from actual competition leaderboards. Fully automated.

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
[[feature-engineering]], [[hyperparameter-optimization]], [[data-science-eda]]
