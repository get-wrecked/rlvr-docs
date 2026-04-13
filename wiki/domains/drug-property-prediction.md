---
domain: Drug Property Prediction (ADMET)
category: Science
verification_type: exact_match
dataset_scale: ~100K+ from TDC/MoleculeNet
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Drug Property Prediction (ADMET)

## Overview
Predict drug-like properties: solubility, permeability, toxicity, metabolic stability, bioavailability. The Therapeutics Data Commons (TDC) provides standardized benchmarks. Critical for drug discovery pipelines.

## Verification Mechanism
Compare predicted properties against experimental measurements. AUROC for classification tasks, RMSE for regression. Verified against curated experimental data.

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
[[molecular-generation]], [[pharmacokinetics]], [[protein-ligand-docking]]
