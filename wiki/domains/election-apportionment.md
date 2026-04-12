---
domain: Election Apportionment
category: Expert
verification_type: exact_match
dataset_scale: ~1K+ from historical election data
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Election Apportionment

## Overview
Compute seat allocation under various methods (D'Hondt, Sainte-Lague, Hamilton, Huntington-Hill).

## Verification Mechanism
Apply apportionment algorithm to vote totals, verify seat assignments match method specification, verify total seats correct.

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
[[voting-social-choice]], [[mathematical-programming]]
