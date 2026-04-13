---
domain: Counterfactual Reasoning
category: Language
verification_type: exact_match
dataset_scale: ~10K+ from TimeTravel/CRASS
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Counterfactual Reasoning

## Overview
Reason about counterfactual scenarios: 'If X had happened instead, what would the outcome be?' TimeTravel and CRASS benchmarks test this with story-based counterfactuals where the correct outcome can be determined.

## Verification Mechanism
Compare predicted counterfactual outcome against gold-standard answer derived from story logic.

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
[[causal-reasoning]], [[commonsense-reasoning]]
