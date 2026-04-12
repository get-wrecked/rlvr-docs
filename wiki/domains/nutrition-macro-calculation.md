---
domain: Nutrition & Macro Calculation
category: Expert
verification_type: exact_match
dataset_scale: ~300K+ foods in USDA FoodData Central
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Nutrition & Macro Calculation

## Overview
Compute meal macronutrients (calories, protein, fat, carbs) from ingredients. Verify against USDA database.

## Verification Mechanism
Look up each ingredient in USDA FoodData Central, compute totals, compare to answer.

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
[[recipe-scaling]], [[unit-conversion]]
