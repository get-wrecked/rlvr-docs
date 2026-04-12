---
domain: Color Palette Generation (WCAG)
category: Misc
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Color Palette Generation (WCAG)

## Overview
Generate color palettes satisfying WCAG contrast ratios for accessibility. Verify via contrast computation.

## Verification Mechanism
Compute contrast ratio between foreground/background pairs using WCAG formula, verify >= 4.5:1 (AA) or 7:1 (AAA).

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
[[accessibility-compliance]], [[html-css-generation]]
