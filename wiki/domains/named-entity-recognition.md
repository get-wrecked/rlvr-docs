---
domain: Named Entity Recognition
category: Language
verification_type: exact_match
dataset_scale: ~100K+ from CoNLL/OntoNotes
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Named Entity Recognition

## Overview
Identify named entities (PERSON, ORG, LOC, DATE) in text. Verify against gold-standard annotations.

## Verification Mechanism
Compare entity spans and labels against gold annotations. Compute precision/recall/F1.

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
[[information-extraction]], [[entity-linking]]
