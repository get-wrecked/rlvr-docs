---
domain: Phase Diagram Reasoning
category: science-physics
verification_type: rule
dataset_scale: 1K-10K substances (NIST, CRC Handbook)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Phase Diagram Reasoning

## Overview

Phase diagram reasoning determines the thermodynamic phase (solid, liquid, gas, supercritical fluid, or specific crystalline phases) of a substance at given temperature and pressure conditions. This includes reading and interpolating phase diagrams, applying the Clausius-Clapeyron equation, using Antoine equation for vapor pressure, and identifying phase boundaries, triple points, and critical points.

RLVR verification leverages known thermodynamic data: phase diagrams for common substances are well-characterized, and the correct phase at any (T, P) point can be determined from established reference data. For many substances, analytical equations (Antoine, Wagner) accurately compute phase boundaries.

## Verification Mechanism

```python
import numpy as np

def antoine_vapor_pressure(T_celsius: float, A: float, B: float, C: float) -> float:
    """Antoine equation: log10(P_mmHg) = A - B/(C + T)"""
    return 10 ** (A - B / (C + T_celsius))

def determine_phase_water(T_kelvin: float, P_pascal: float) -> str:
    """Simplified phase determination for water using known boundaries."""
    T_C = T_kelvin - 273.15

    # Triple point: 273.16 K, 611.657 Pa
    # Critical point: 647.096 K, 22.064 MPa

    if T_kelvin >= 647.096 and P_pascal >= 22.064e6:
        return "supercritical"

    # Vapor pressure from Antoine equation (valid 1-100C)
    # Antoine params for water: A=8.07131, B=1730.63, C=233.426
    if 1 <= T_C <= 100:
        p_sat = antoine_vapor_pressure(T_C, 8.07131, 1730.63, 233.426)
        p_sat_pa = p_sat * 133.322  # mmHg to Pa

        if P_pascal > p_sat_pa * 1.01:  # above vapor pressure
            if T_kelvin < 273.15:
                return "solid"
            else:
                return "liquid"
        elif P_pascal < p_sat_pa * 0.99:
            return "gas"
        else:
            return "liquid-gas_equilibrium"

    if T_kelvin < 273.15 and P_pascal < 611.657:
        return "gas"  # below triple point pressure = sublimation
    if T_kelvin < 273.15:
        return "solid"
    return "gas"  # simplified; real logic is more complex

def verify_phase(predicted_phase: str, T: float, P: float,
                  substance: str, gold_phase: str) -> dict:
    """Verify phase determination."""
    pred_clean = predicted_phase.strip().lower()
    gold_clean = gold_phase.strip().lower()

    # Allow synonyms
    synonyms = {
        "gas": ["gas", "vapor", "gaseous", "vapour"],
        "liquid": ["liquid"],
        "solid": ["solid", "ice", "crystalline"],
        "supercritical": ["supercritical", "supercritical fluid", "scf"],
    }
    pred_canonical = pred_clean
    gold_canonical = gold_clean
    for canonical, variants in synonyms.items():
        if pred_clean in variants:
            pred_canonical = canonical
        if gold_clean in variants:
            gold_canonical = canonical

    match = pred_canonical == gold_canonical

    return {
        "correct": match,
        "predicted": pred_canonical,
        "gold": gold_canonical,
        "reward": 1.0 if match else 0.0
    }
```

## Dataset Sources

- **NIST Chemistry WebBook**: Thermodynamic data for ~7,000 substances including phase transition temperatures, vapor pressures, critical points. Free online access. The primary reference.
- **CRC Handbook of Chemistry and Physics**: Comprehensive physical property data for thousands of substances. Includes phase diagrams, triple points, critical constants.
- **NIST/TRC ThermoML**: Machine-readable thermodynamic property database with experimental measurements.
- **DIPPR 801**: Physical property data for ~2,200 compounds with recommended values and uncertainties. Licensed but widely used in engineering.
- **ASM International Phase Diagram Database**: Alloy phase diagrams (binary, ternary). Metal-focused but very comprehensive (~40,000 diagrams).
- **Procedural generation**: For common substances with known Antoine parameters, generate unlimited (T, P, phase) training triples by evaluating the vapor pressure equation.
- **Textbook problems**: General chemistry and physical chemistry textbooks contain hundreds of phase diagram problems.

## Task Format

- **Input**: Substance, temperature, and pressure.
```
What phase is water at T = 120C and P = 1 atm (101.325 kPa)?
```
- **Output**: Phase identification with reasoning.
```
Gas (steam).
At 1 atm, water boils at 100C. Since 120C > 100C and we are at 1 atm,
water is in the gas phase.
```

More complex example:
```
CO2 at T = -60C and P = 5 atm. What phase?
Triple point of CO2: -56.6C, 5.18 atm. At 5 atm < 5.18 atm and T < -56.6C,
CO2 is in the solid phase (dry ice). It would sublimate directly to gas.
```

## Difficulty Curriculum

- Level 1: Identify phase of water at common conditions (room temperature, boiling point, freezing point)
- Level 2: Read a phase diagram to determine phase at a given (T, P) point for water or CO2
- Level 3: Identify triple point and critical point from data; determine phase near these points
- Level 4: Use Antoine equation to compute vapor pressure and determine phase
- Level 5: Determine phase for less common substances using provided thermodynamic data
- Level 6: Phase transitions with Clausius-Clapeyron equation (compute dT/dP along phase boundary)
- Level 7: Binary mixture phase behavior (Raoult's law, azeotropes)
- Level 8: Ternary phase diagrams, eutectic systems, solid solutions
- Level 9: Metallic alloy phase diagrams (Fe-C, Cu-Zn), non-equilibrium phases, high-pressure exotic phases

## Limitations & Risks

- **Data coverage**: NIST has excellent data for ~7,000 common substances but poor coverage for exotic or novel compounds. Phase behavior of complex mixtures is often unavailable.
- **Simplified models**: Antoine equation is accurate over limited temperature ranges. Extrapolation outside valid ranges gives wrong answers. More complex equations (Wagner, PPDS) are needed for wider ranges.
- **Phase diagram complexity**: Real phase diagrams can have dozens of solid phases (polymorphs). Ice alone has 19 known phases. Simplified models miss these.
- **Mixture behavior**: Phase behavior of mixtures is far more complex than pure substances. Raoult's law breaks down for non-ideal solutions. Activity coefficients are needed but add substantial complexity.
- **Kinetic vs. thermodynamic**: Phase diagrams show equilibrium phases. Metastable phases (supercooled liquids, supersaturated solutions) exist in practice but are not "correct" according to the equilibrium diagram.
- **Units and conventions**: Temperature in K vs. C vs. F, pressure in Pa vs. atm vs. bar vs. mmHg. Unit conversion errors are a major source of mistakes.

## Connections

- [[chemistry-computation]] — General chemistry calculations include phase-related problems
- [[chemical-equation-balancing]] — Both are core chemistry skills
- [[climate-science]] — Climate modeling depends on water phase transitions in the atmosphere
- [[materials-science]] — Materials selection depends on phase stability at operating conditions
