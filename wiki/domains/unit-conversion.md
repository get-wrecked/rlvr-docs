---
domain: unit-conversion
category: miscellaneous
verification_type: exact_match
dataset_scale: ~infinite (template-generated)
difficulty_range: easy/medium
modality: text
status: verified
---

# Unit Conversion

## Overview

Unit conversion tasks require converting quantities between measurement units: length (miles to kilometers), weight (pounds to grams), temperature (Fahrenheit to Celsius), volume (gallons to liters), speed, pressure, energy, data sizes, cooking measurements, and many other systems. Verification is exact numerical comparison.

This domain is simple but foundational — unit conversion errors cause real-world failures (NASA's Mars Climate Orbiter was lost due to a unit conversion error). It exercises precise arithmetic and knowledge of conversion factors.

## Verification Mechanism

**Type: Exact match (numerical)**

```python
CONVERSION_DB = {
    ('miles', 'kilometers'): 1.60934,
    ('pounds', 'kilograms'): 0.453592,
    ('fahrenheit', 'celsius'): lambda f: (f - 32) * 5/9,
    ('gallons_us', 'liters'): 3.78541,
    ('inches', 'centimeters'): 2.54,
    ('acres', 'hectares'): 0.404686,
    # ... hundreds more
}

def verify_conversion(value: float, from_unit: str, to_unit: str,
                      model_answer: float, tolerance: float = 0.001) -> float:
    expected = convert(value, from_unit, to_unit)
    relative_error = abs(model_answer - expected) / abs(expected) if expected != 0 else abs(model_answer)
    if relative_error < tolerance:
        return 1.0
    elif relative_error < tolerance * 10:
        return 0.5  # Close but not precise enough
    return 0.0
```

**Tolerance:** Allow small floating-point differences. For practical purposes, 0.1% relative error is a reasonable threshold. Some tasks may specify precision requirements ("answer to 2 decimal places").

**Verification confidence: VERY HIGH.** Conversion factors are exact physical constants or defined ratios. Verification is a simple numerical comparison. No ambiguity.

## Dataset Sources

- **Template generation (primary):** `Convert {random_value} {random_unit} to {compatible_unit}`. Can generate unlimited problems.
- **Unit conversion lookup tables:** NIST provides authoritative conversion factors. Also: GNU units database (3000+ units).
- **Physics/engineering textbooks:** Textbook problems involving unit conversion, often in multi-step calculations.
- **Cooking measurement conversions:** Tablespoons, cups, teaspoons — practical and frequently needed.
- **Historical units:** Furlongs, cubits, stone — adds variety and tests knowledge breadth.
- **Currency conversion (snapshot):** Using fixed historical rates, not live rates.
- **Data size conversions:** Bytes, KB, MB, GB, TB — common in computing with the KB=1000 vs KiB=1024 subtlety.

## Task Format

**Input:**
```
Convert 72 degrees Fahrenheit to Celsius. Round to 1 decimal place.
```

**Output:**
```
22.2
```

**Input (multi-step):**
```
A car travels at 65 miles per hour. What is its speed in meters per second? 
Round to 2 decimal places.
```

**Output:**
```
29.06
```

**Input (complex):**
```
Convert 2.5 atmospheres of pressure to:
a) Pascals
b) mmHg (torr)
c) PSI
```

## Difficulty Curriculum

1. **Easy:** Single-step conversions with common units (miles to km, pounds to kg, F to C).
2. **Medium:** Multi-step conversions (mph to m/s requires miles-to-meters and hours-to-seconds), compound units (kg/m^3 to lb/ft^3).
3. **Hard:** Obscure units (nautical miles, troy ounces, BTU), dimensional analysis chains, unit systems (CGS to SI), conversions requiring domain knowledge (light-years, astronomical units).
4. **Very hard:** Conversions in context (physics problems where unit conversion is embedded in a larger calculation), ambiguous unit names (gallon — US or Imperial? ton — short, long, or metric?).

## Limitations & Risks

- **Very low reasoning ceiling:** Unit conversion is fundamentally multiplication/division. Once the model knows the conversion factors, the task is trivial. Limited value for building deep reasoning.
- **Precision ambiguity:** How many decimal places? Must specify in the task or accept a range.
- **Unit name ambiguity:** "Ton" means different things in different contexts. "Gallon" differs between US and Imperial. Must be explicit.
- **Not a standalone training domain:** Best used as a component within broader math or science tasks, not as a primary RL training signal.
- **Temperature is special:** Fahrenheit/Celsius conversion involves addition, not just multiplication. Kelvin adds another wrinkle. These are easy to mess up.

## Connections

- [[recipe-scaling]] — Recipe scaling often requires unit conversion.
- [[financial-calculation]] — Currency conversion is a specific unit conversion case.
- [[date-time-computation]] — Time unit conversions (hours to seconds, etc.) overlap.
- [[math-competition]] — Dimensional analysis is a fundamental math skill.
