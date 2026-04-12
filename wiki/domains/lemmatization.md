---
domain: Lemmatization
category: Language
verification_type: exact_match
dataset_scale: ~1M+ from UniMorph
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Lemmatization

## Overview
Reduce inflected words to dictionary lemmas. Verify against morphological databases.

## Verification Mechanism
Compare output lemma to UniMorph/WordNet reference. Handle irregular forms.

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
[[morphological-inflection]], [[spelling-grammar]]
