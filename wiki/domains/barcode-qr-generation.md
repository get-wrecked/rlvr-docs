---
domain: Barcode & QR Code Generation
category: Misc
verification_type: execution
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Barcode & QR Code Generation

## Overview
Generate barcodes (EAN, UPC, Code128) and QR codes. Verify by decoding back to original data.

## Verification Mechanism
Generate code, decode with zbar/zxing library, verify decoded data matches input exactly.

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
[[compression-encoding]], [[encryption-decryption]]
