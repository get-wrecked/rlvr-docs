---
domain: Image Compression Optimization
category: Misc
verification_type: diff
dataset_scale: unlimited (from any images)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Image Compression Optimization

## Overview
Compress images to target file size while maximizing visual quality (SSIM/PSNR). Verify metrics.

## Verification Mechanism
Measure output file size (must be under target), compute SSIM/PSNR against original, reward higher quality.

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
[[compression-encoding]], [[data-visualization]]
