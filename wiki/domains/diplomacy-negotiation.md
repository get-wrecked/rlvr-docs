---
domain: Diplomacy (Natural Language Negotiation)
category: Games
verification_type: outcome
dataset_scale: unlimited (self-play)
difficulty_range: superhuman
modality: text
status: strong_hypothesis
---

# Diplomacy (Natural Language Negotiation)

## Overview
Play the board game Diplomacy using natural language negotiation. Meta's CICERO system showed this is feasible. Requires strategic reasoning, deception detection, alliance formation, and natural language persuasion — all with verifiable game outcomes.

## Verification Mechanism
Game score (supply center count) and win/draw/loss. The game has deterministic adjudication of orders. Negotiation quality is measured by game outcome, not by judging the text.

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
[[negotiation-bargaining]], [[multi-agent-coordination]], [[board-games]]
