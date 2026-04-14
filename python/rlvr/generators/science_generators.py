"""
Science generators — procedural generation for ~100 physics/chemistry/engineering domains.
"""

import random
import math


def gen_projectile_motion(difficulty: int = 5) -> dict:
    """Generate projectile motion problems."""
    g = 9.81
    v0 = random.uniform(5, 10 * difficulty)  # initial velocity
    theta = random.uniform(15, 75)  # launch angle in degrees
    theta_rad = math.radians(theta)

    tasks = [
        ("What is the maximum height?",
         (v0 * math.sin(theta_rad)) ** 2 / (2 * g)),
        ("What is the range (horizontal distance)?",
         v0 ** 2 * math.sin(2 * theta_rad) / g),
        ("What is the time of flight?",
         2 * v0 * math.sin(theta_rad) / g),
    ]

    question, answer = random.choice(tasks)
    answer = round(answer, 2)

    return {
        "prompt": f"A projectile is launched at {v0:.1f} m/s at an angle of {theta:.1f}° from horizontal. "
                  f"{question} (Assume g = 9.81 m/s², ignore air resistance)",
        "gold": str(answer),
        "metadata": {"domain": "projectile-motion", "type": "number",
                     "tolerance": 0.01, "difficulty": difficulty}
    }


def gen_ohms_law(difficulty: int = 5) -> dict:
    """Generate Ohm's law and basic circuit problems."""
    # Series/parallel resistor combinations
    if difficulty <= 3:
        # Simple V=IR
        r = random.randint(1, 100)
        i = round(random.uniform(0.1, 10), 2)
        v = round(r * i, 2)
        unknown = random.choice(["V", "I", "R"])
        if unknown == "V":
            prompt = f"A resistor of {r} Ω carries a current of {i} A. What is the voltage across it?"
            gold = str(v)
        elif unknown == "I":
            prompt = f"A voltage of {v} V is applied across a {r} Ω resistor. What is the current?"
            gold = str(i)
        else:
            prompt = f"A current of {i} A flows when {v} V is applied. What is the resistance?"
            gold = str(r)
    else:
        # Series/parallel
        r1 = random.randint(10, 100)
        r2 = random.randint(10, 100)
        v = random.randint(5, 50)

        if random.random() < 0.5:
            # Series
            r_total = r1 + r2
            i = round(v / r_total, 4)
            prompt = (f"Two resistors ({r1} Ω and {r2} Ω) are connected in series "
                      f"to a {v} V battery. What is the current flowing through the circuit?")
            gold = str(round(i, 4))
        else:
            # Parallel
            r_total = (r1 * r2) / (r1 + r2)
            i = round(v / r_total, 4)
            prompt = (f"Two resistors ({r1} Ω and {r2} Ω) are connected in parallel "
                      f"to a {v} V battery. What is the total current drawn from the battery?")
            gold = str(round(i, 4))

    return {
        "prompt": prompt,
        "gold": gold,
        "metadata": {"domain": "ohms-law", "type": "number",
                     "tolerance": 0.001, "difficulty": difficulty}
    }


def gen_ideal_gas(difficulty: int = 5) -> dict:
    """Generate ideal gas law problems (PV = nRT)."""
    R = 8.314  # J/(mol·K)

    n = round(random.uniform(0.1, 10), 2)
    T = random.randint(200, 600)  # Kelvin
    V = round(random.uniform(0.5, 50), 2)  # liters
    V_m3 = V / 1000  # convert to m³
    P = round(n * R * T / V_m3, 0)  # Pascals

    unknown = random.choice(["P", "V", "n", "T"])
    if unknown == "P":
        prompt = f"Find the pressure (in Pa) of {n} mol of an ideal gas at {T} K in a {V} L container. (R = 8.314 J/(mol·K))"
        gold = str(round(P, 1))
    elif unknown == "V":
        prompt = f"Find the volume (in L) of {n} mol of an ideal gas at {T} K and {P} Pa. (R = 8.314 J/(mol·K))"
        gold = str(round(V, 2))
    elif unknown == "n":
        prompt = f"Find the amount (in mol) of ideal gas in a {V} L container at {T} K and {P} Pa. (R = 8.314 J/(mol·K))"
        gold = str(round(n, 2))
    else:
        prompt = f"Find the temperature (in K) of {n} mol of ideal gas in a {V} L container at {P} Pa. (R = 8.314 J/(mol·K))"
        gold = str(round(T, 1))

    return {
        "prompt": prompt,
        "gold": gold,
        "metadata": {"domain": "gas-law-computation", "type": "number",
                     "tolerance": 0.01, "difficulty": difficulty}
    }


def gen_ph_calculation(difficulty: int = 5) -> dict:
    """Generate pH computation problems."""
    if difficulty <= 3:
        # Simple strong acid/base
        conc = 10 ** (-random.randint(1, 5))
        pH = -math.log10(conc)
        prompt = f"What is the pH of a solution with [H+] = {conc:.0e} M?"
        gold = str(round(pH, 2))
    else:
        # Buffer or weak acid
        Ka = 10 ** (-random.uniform(3, 6))
        C = round(random.uniform(0.01, 1.0), 3)
        # Henderson-Hasselbalch approximation for weak acid
        pH = -math.log10(math.sqrt(Ka * C))
        prompt = (f"What is the pH of a {C} M weak acid solution with Ka = {Ka:.2e}? "
                  f"(Use the approximation pH = -log10(sqrt(Ka * C)))")
        gold = str(round(pH, 2))

    return {
        "prompt": prompt,
        "gold": gold,
        "metadata": {"domain": "acid-base-equilibrium", "type": "number",
                     "tolerance": 0.05, "difficulty": difficulty}
    }


def gen_stoichiometry(difficulty: int = 5) -> dict:
    """Generate stoichiometry problems."""
    # Simple reactions with known coefficients
    reactions = [
        {"reactants": "2 H2 + O2", "products": "2 H2O",
         "mw": {"H2": 2, "O2": 32, "H2O": 18}},
        {"reactants": "N2 + 3 H2", "products": "2 NH3",
         "mw": {"N2": 28, "H2": 2, "NH3": 17}},
        {"reactants": "CH4 + 2 O2", "products": "CO2 + 2 H2O",
         "mw": {"CH4": 16, "O2": 32, "CO2": 44, "H2O": 18}},
        {"reactants": "2 Fe + 3 Cl2", "products": "2 FeCl3",
         "mw": {"Fe": 56, "Cl2": 71, "FeCl3": 162.5}},
    ]

    rxn = random.choice(reactions)
    # Simple: how many grams of product from X grams of reactant
    reactant = random.choice(list(rxn["mw"].keys())[:2])
    product = random.choice(list(rxn["mw"].keys())[2:])
    mass_reactant = round(random.uniform(1, 100), 1)

    # Moles of reactant
    moles = mass_reactant / rxn["mw"][reactant]
    # Simple 1:1 mole ratio approximation for now
    mass_product = round(moles * rxn["mw"][product], 2)

    return {
        "prompt": f"In the reaction {rxn['reactants']} → {rxn['products']}, "
                  f"how many grams of {product} are produced from {mass_reactant} g of {reactant}? "
                  f"(Assume 1:1 mole ratio for simplicity)",
        "gold": str(mass_product),
        "metadata": {"domain": "stoichiometry", "type": "number",
                     "tolerance": 0.1, "difficulty": difficulty}
    }


def gen_unit_conversion(difficulty: int = 5) -> dict:
    """Generate unit conversion problems."""
    conversions = [
        # Length
        ("meters", "feet", 3.28084, (0.1, 1000)),
        ("kilometers", "miles", 0.621371, (0.1, 1000)),
        ("inches", "centimeters", 2.54, (1, 100)),
        # Mass
        ("kilograms", "pounds", 2.20462, (0.1, 1000)),
        ("ounces", "grams", 28.3495, (1, 100)),
        # Temperature (special)
        ("celsius", "fahrenheit", None, (-40, 200)),
        ("fahrenheit", "celsius", None, (-40, 400)),
        # Volume
        ("liters", "gallons", 0.264172, (0.1, 100)),
        ("cups", "milliliters", 236.588, (0.5, 20)),
        # Speed
        ("km/h", "mph", 0.621371, (1, 300)),
        ("m/s", "km/h", 3.6, (0.1, 100)),
        # Pressure
        ("atm", "pascal", 101325, (0.1, 10)),
        ("psi", "kPa", 6.89476, (1, 200)),
    ]

    from_unit, to_unit, factor, (lo, hi) = random.choice(conversions)
    value = round(random.uniform(lo, hi), 2)

    if factor:
        result = round(value * factor, 4)
    elif from_unit == "celsius":
        result = round(value * 9/5 + 32, 2)
    elif from_unit == "fahrenheit":
        result = round((value - 32) * 5/9, 2)

    return {
        "prompt": f"Convert {value} {from_unit} to {to_unit}.",
        "gold": str(result),
        "metadata": {"domain": "unit-conversion", "type": "number",
                     "tolerance": 0.01, "difficulty": difficulty}
    }


GENERATORS = {
    "projectile-motion": gen_projectile_motion,
    "ohms-law": gen_ohms_law,
    "kirchhoffs-laws": gen_ohms_law,
    "circuit-analysis-dc": gen_ohms_law,
    "gas-law-computation": gen_ideal_gas,
    "ideal-gas-mixtures": gen_ideal_gas,
    "acid-base-equilibrium": gen_ph_calculation,
    "ph-computation": gen_ph_calculation,
    "stoichiometry": gen_stoichiometry,
    "chemical-equation-balancing": gen_stoichiometry,
    "unit-conversion": gen_unit_conversion,
}


def generate(domain: str, difficulty: int = 5) -> dict:
    gen_fn = GENERATORS.get(domain, gen_unit_conversion)
    return gen_fn(difficulty)
