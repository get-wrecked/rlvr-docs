---
domain: recipe-scaling
category: miscellaneous
verification_type: exact_match
dataset_scale: ~large (recipe datasets + synthetic)
difficulty_range: easy/medium
modality: text
status: verified
---

# Recipe Scaling

## Overview

Recipe scaling tasks require adjusting recipe ingredient quantities proportionally when changing the number of servings. Given a recipe for N servings, produce the adjusted recipe for M servings. This is fundamentally a proportional arithmetic problem with domain-specific complications: unit conversions (tablespoons to cups), rounding to practical measurements, and ingredient-specific non-linear scaling (baking times, seasoning adjustments).

The domain has simple, exact verification for the core task (proportional scaling) and is highly practical — recipe scaling is a common real-world need.

## Verification Mechanism

**Type: Exact match (numerical)**

**Core verification:**
```python
def verify_scaling(original_qty: float, original_servings: int,
                   target_servings: int, model_qty: float,
                   tolerance: float = 0.01) -> bool:
    expected = original_qty * target_servings / original_servings
    return abs(model_qty - expected) < tolerance
```

**For each ingredient:** Extract quantity from model output, compare against expected scaled quantity.

**Extended verification for unit conversion:**
```python
def verify_with_unit_conversion(original: str, target: str, scale_factor: float) -> float:
    orig_qty, orig_unit = parse_ingredient(original)  # "2 cups" -> (2, "cups")
    tgt_qty, tgt_unit = parse_ingredient(target)
    
    # Convert both to base unit and compare
    orig_base = to_base_unit(orig_qty, orig_unit)  # cups -> ml
    tgt_base = to_base_unit(tgt_qty, tgt_unit)
    expected_base = orig_base * scale_factor
    
    return 1.0 if abs(tgt_base - expected_base) / expected_base < 0.02 else 0.0
```

**Reward: Per-ingredient accuracy averaged across all ingredients.**

**Verification confidence: HIGH for proportional scaling.** Arithmetic is exact. Unit conversion is deterministic given a conversion table. The main challenge is parsing ingredient strings from model output, which requires a recipe ingredient parser (several exist: ingredient-parser-nlp, recipe-scrapers).

## Dataset Sources

- **Recipe1M+ dataset:** 1 million+ recipes with structured ingredient lists. Gold standard for recipe data.
- **RecipeNLG:** 2.2 million recipes with structured data.
- **Allrecipes, Food Network web scraping:** Massive recipe collections with serving sizes and ingredient quantities.
- **Open Recipe Format (ORF):** Standardized recipe data format.
- **recipe-scrapers Python library:** Scrapes structured recipe data from 100+ cooking websites.
- **Synthetic generation:** Take any recipe, apply random scale factor, create (original, target_servings, expected_output) triples.

## Task Format

**Input:**
```
Scale this recipe from 4 servings to 10 servings:

Chocolate Chip Cookies (4 servings)
- 1 cup all-purpose flour
- 1/2 cup butter, softened
- 1/3 cup sugar
- 1/4 cup brown sugar
- 1 egg
- 1 tsp vanilla extract
- 1/2 tsp baking soda
- 1 cup chocolate chips
```

**Output:**
```
Chocolate Chip Cookies (10 servings)
- 2 1/2 cups all-purpose flour
- 1 1/4 cups butter, softened
- 5/6 cup sugar
- 5/8 cup brown sugar
- 2 1/2 eggs
- 2 1/2 tsp vanilla extract
- 1 1/4 tsp baking soda
- 2 1/2 cups chocolate chips
```

**Verification:** Each ingredient quantity checked against expected (original * 10/4).

## Difficulty Curriculum

1. **Easy:** Integer scale factors (double, triple), round quantities, no unit conversion needed.
2. **Medium:** Non-integer scale factors (4 servings to 6), fractional quantities, basic unit awareness (3 teaspoons = 1 tablespoon).
3. **Hard:** Unit conversion required (scale up from teaspoons to cups), practical rounding (don't ask for 2.37 eggs — round to 2 or 2.5), mixed number handling.
4. **Very hard:** Non-linear scaling (baking time doesn't scale linearly, salt and spices scale sub-linearly), substitution suggestions when scaled quantities are impractical.

Note: Non-linear scaling tasks move verification from exact to approximate, since "correct" scaling of baking time is debatable.

## Limitations & Risks

- **Shallow reasoning:** At its core, this is multiplication and division. The reasoning depth is limited.
- **Ingredient parsing is messy:** Natural language ingredient descriptions are highly variable ("2 large eggs, beaten" vs. "eggs, 2 large" vs. "2 eggs"). Robust parsing is needed.
- **Fractional representation:** "5/6 cup" is mathematically correct but practically useless in a kitchen. Should the model round to "3/4 cup + 2 tablespoons"? The "correct" answer depends on precision requirements.
- **Non-linear scaling is subjective:** Whether baking time should scale linearly, by square root, or not at all depends on the recipe and pan sizes. These tasks lack clear ground truth.
- **Low ceiling for capability improvement:** Once the model can do proportional arithmetic and unit conversion, there is little further capability to build.

## Connections

- [[unit-conversion]] — Unit conversion is the core sub-skill.
- [[financial-calculation]] — Both involve precise arithmetic with domain-specific formulas.
- [[date-time-computation]] — Similar "apply rules to compute correct answer" structure.
