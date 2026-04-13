---
domain: Adversarial Robustness
category: ML
verification_type: execution
dataset_scale: unlimited (from any model + data)
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Adversarial Robustness

## Overview
Generate adversarial examples that fool classifiers while being imperceptible to humans (L_p bounded perturbations). Or defend against adversarial attacks. Both attacker and defender have verifiable objectives.

## Verification Mechanism
Attack: model misclassifies AND perturbation is within L_p ball (measurable). Defense: model maintains accuracy on adversarial examples AND clean accuracy stays high.

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
[[image-classification]], [[ml-pipeline-optimization]]
