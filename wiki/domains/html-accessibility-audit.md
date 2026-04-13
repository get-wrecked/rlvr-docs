---
domain: HTML Accessibility Audit
category: Code
verification_type: execution
dataset_scale: ~10K+ from WebAIM examples
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# HTML Accessibility Audit

## Overview
Identify accessibility issues in HTML (missing alt text, poor contrast, missing ARIA labels). Verify via axe-core.

## Verification Mechanism
Run axe-core accessibility checker on HTML, compare detected issues to known ground truth.

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
