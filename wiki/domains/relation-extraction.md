---
domain: Relation Extraction
category: Language
verification_type: exact_match
dataset_scale: ~100K+ from TACRED/DocRED
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Relation Extraction

## Overview
Extract semantic relations between entities in text (born-in, works-for, capital-of). Verify against labeled data.

## Verification Mechanism
Compare extracted (subject, relation, object) triples against gold annotations.

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
[[information-extraction]], [[knowledge-graph-completion]]
