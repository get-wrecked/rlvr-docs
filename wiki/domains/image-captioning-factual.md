---
domain: Factual Image Captioning
category: Vision
verification_type: exact_match
dataset_scale: ~100K+ from COCO Captions
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Factual Image Captioning

## Overview
Generate factual captions for images. Verify key objects and relationships against object detection ground truth.

## Verification Mechanism
Run object detector on image, verify caption mentions all detected objects. Check spatial relationships.

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
[[visual-question-answering]], [[image-classification]]
