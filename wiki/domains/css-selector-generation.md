---
domain: CSS Selector Generation
category: Code
verification_type: execution
dataset_scale: unlimited (from any HTML)
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# CSS Selector Generation

## Overview
Generate CSS selectors that match specified elements in an HTML document. Verify by querying DOM.

## Verification Mechanism
Apply selector to HTML via BeautifulSoup/lxml, verify it selects exactly the target elements (no more, no fewer).

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
[[html-css-generation]], [[web-scraping]]
