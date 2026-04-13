---
domain: Causal Graph Discovery from Data
category: ML
verification_type: exact_match
dataset_scale: ~100+ from CauseMe/Tuebingen pairs
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Causal Graph Discovery from Data

## Overview
Discover causal relationships (directed acyclic graphs) from observational data. Verify against known ground-truth causal structures. This is fundamental to scientific reasoning.

## Verification Mechanism
Compare predicted DAG against ground-truth causal graph using SHD (Structural Hamming Distance). Verify edge directions correct. Can also verify via conditional independence tests implied by the graph.

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
[[bayesian-network-reasoning]], [[causal-reasoning]]
