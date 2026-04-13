---
domain: Receipt / Invoice Parsing
category: Vision
verification_type: exact_match
dataset_scale: ~50K+ from SROIE/CORD datasets
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Receipt / Invoice Parsing

## Overview
Extract structured data from receipt/invoice images (vendor, date, items, totals). Verify against annotations.

## Verification Mechanism
Compare extracted fields against gold-standard annotations. Verify total = sum of line items.

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
[[document-ocr-extraction]], [[accounting-bookkeeping]]
