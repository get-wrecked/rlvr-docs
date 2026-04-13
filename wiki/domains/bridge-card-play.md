---
domain: Bridge Card Play
category: Games
verification_type: outcome
dataset_scale: ~1M+ from bridge databases
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Bridge Card Play

## Overview
Play bridge (bidding + card play). Verify via double-dummy analysis (minimax optimal play).

## Verification Mechanism
Compare tricks taken against double-dummy solver (Bo Haglund's DDS). Verify bidding against system rules.

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
[[card-games]], [[negotiation-bargaining]]
