---
domain: Catalyst Design (OC20/OC22)
category: Science
verification_type: simulation
dataset_scale: ~260M+ DFT calculations in OC20
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Catalyst Design (OC20/OC22)

## Overview
Design catalysts for chemical reactions (e.g., CO2 reduction, water splitting). The Open Catalyst project (Meta/CMU) provides the largest dataset of DFT calculations for catalyst surfaces. This is directly relevant to clean energy and climate change.

## Verification Mechanism
Predict adsorption energies for molecules on catalyst surfaces. Verify against DFT reference calculations. Measure MAE against gold DFT values.

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
[[materials-science]], [[chemistry-computation]], [[molecular-generation]]
