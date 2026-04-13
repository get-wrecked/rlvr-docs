---
domain: Multi-Tool Orchestration (GAIA-style)
category: Agent
verification_type: exact_match
dataset_scale: ~450+ from GAIA benchmark
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Multi-Tool Orchestration (GAIA-style)

## Overview
Answer questions that require orchestrating multiple tools: web search, code execution, file processing, API calls, and reasoning over results. GAIA (General AI Assistants) tests this with questions like 'What was the GDP per capita of the country that won the most medals at the 2022 Winter Olympics?' requiring search + calculation.

## Verification Mechanism
Compare final answer against gold-standard verified answer. The answer is typically a specific number, name, or short phrase.

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
[[multi-hop-web-research]], [[computer-use]], [[web-navigation]]
