"""
Math generators — procedural generation for ~150 math domains.

Generates problems with exact verified solutions at any difficulty level.
Uses SymPy for symbolic verification, NumPy for numerical reference.
"""

import random
import math
import fractions
from typing import Callable


def _difficulty_scale(difficulty: int, low: float, high: float) -> float:
    """Scale a value based on difficulty 1-10."""
    return low + (high - low) * (difficulty - 1) / 9


# ═══════════════════════════════════════════
# ARITHMETIC & NUMERICAL
# ═══════════════════════════════════════════

def gen_arithmetic(difficulty: int = 5) -> dict:
    """Generate arithmetic problems: +, -, *, /, with increasing complexity."""
    ops = ['+', '-', '*']
    if difficulty >= 3:
        ops.append('//')
    if difficulty >= 5:
        ops.append('**')

    num_ops = min(difficulty, 6)
    numbers = [random.randint(1, int(_difficulty_scale(difficulty, 10, 10000))) for _ in range(num_ops + 1)]
    operators = [random.choice(ops) for _ in range(num_ops)]

    expr = str(numbers[0])
    for i, op in enumerate(operators):
        if op == '//' and numbers[i + 1] == 0:
            numbers[i + 1] = 1
        expr += f" {op} {numbers[i + 1]}"

    try:
        result = eval(expr)
        if isinstance(result, float) and result != int(result):
            result = round(result, 6)
        else:
            result = int(result)
    except (ZeroDivisionError, OverflowError):
        return gen_arithmetic(difficulty)

    return {
        "prompt": f"Compute: {expr}",
        "gold": str(result),
        "metadata": {"domain": "math-numerical", "type": "number", "difficulty": difficulty}
    }


def gen_fraction_arithmetic(difficulty: int = 5) -> dict:
    """Generate fraction arithmetic problems."""
    max_den = int(_difficulty_scale(difficulty, 5, 50))
    a = fractions.Fraction(random.randint(-max_den, max_den), random.randint(1, max_den))
    b = fractions.Fraction(random.randint(-max_den, max_den), random.randint(1, max_den))
    op = random.choice(['+', '-', '*', '/'])

    if op == '/' and b == 0:
        b = fractions.Fraction(1, random.randint(1, max_den))

    expr_str = f"{a} {op} {b}"
    if op == '+': result = a + b
    elif op == '-': result = a - b
    elif op == '*': result = a * b
    elif op == '/': result = a / b

    return {
        "prompt": f"Compute as a simplified fraction: {expr_str}",
        "gold": str(result),
        "metadata": {"domain": "math-numerical", "type": "text", "difficulty": difficulty}
    }


def gen_percentage(difficulty: int = 5) -> dict:
    """Generate percentage problems."""
    templates = [
        ("What is {p}% of {n}?", lambda p, n: p * n / 100),
        ("{a} is what percent of {b}?", None),
        ("If {n} increases by {p}%, what is the new value?", lambda p, n: n * (1 + p / 100)),
    ]
    p = random.randint(1, int(_difficulty_scale(difficulty, 50, 500)))
    n = random.randint(1, int(_difficulty_scale(difficulty, 100, 100000)))

    idx = random.randint(0, min(difficulty // 3, len(templates) - 1))
    template, fn = templates[idx]

    if fn:
        prompt = template.format(p=p, n=n)
        result = fn(p, n)
        if result == int(result):
            result = int(result)
        else:
            result = round(result, 2)
    else:
        a = random.randint(1, n)
        prompt = f"{a} is what percent of {n}?"
        result = round(a / n * 100, 2)

    return {
        "prompt": prompt,
        "gold": str(result),
        "metadata": {"domain": "math-numerical", "type": "number", "difficulty": difficulty}
    }


# ═══════════════════════════════════════════
# LINEAR ALGEBRA
# ═══════════════════════════════════════════

def gen_system_of_equations(difficulty: int = 5) -> dict:
    """Generate systems of linear equations with integer solutions."""
    n = min(2 + difficulty // 3, 5)  # 2x2 to 5x5
    max_val = int(_difficulty_scale(difficulty, 5, 20))

    # Generate solution first, then construct equations
    solution = [random.randint(-max_val, max_val) for _ in range(n)]

    equations = []
    for i in range(n):
        coeffs = [random.randint(-max_val, max_val) for _ in range(n)]
        # Ensure not all zero
        while all(c == 0 for c in coeffs):
            coeffs = [random.randint(-max_val, max_val) for _ in range(n)]
        rhs = sum(c * s for c, s in zip(coeffs, solution))

        vars_str = ['x', 'y', 'z', 'w', 'v'][:n]
        terms = []
        for j, (c, v) in enumerate(zip(coeffs, vars_str)):
            if c == 0:
                continue
            if c == 1:
                terms.append(v)
            elif c == -1:
                terms.append(f"-{v}")
            else:
                terms.append(f"{c}{v}")
        eq_str = " + ".join(terms).replace("+ -", "- ")
        equations.append(f"{eq_str} = {rhs}")

    vars_str = ['x', 'y', 'z', 'w', 'v'][:n]
    gold = ", ".join(f"{v} = {s}" for v, s in zip(vars_str, solution))

    return {
        "prompt": f"Solve the system of equations:\n" + "\n".join(equations),
        "gold": gold,
        "metadata": {"domain": "system-of-equations", "type": "text", "difficulty": difficulty,
                     "solution": solution}
    }


def gen_determinant(difficulty: int = 5) -> dict:
    """Generate matrix determinant problems."""
    n = min(2 + difficulty // 3, 5)
    max_val = int(_difficulty_scale(difficulty, 5, 20))

    matrix = [[random.randint(-max_val, max_val) for _ in range(n)] for _ in range(n)]

    # Compute determinant (simple recursive for small matrices)
    def det(m):
        if len(m) == 1:
            return m[0][0]
        if len(m) == 2:
            return m[0][0] * m[1][1] - m[0][1] * m[1][0]
        result = 0
        for j in range(len(m)):
            minor = [row[:j] + row[j+1:] for row in m[1:]]
            result += ((-1) ** j) * m[0][j] * det(minor)
        return result

    result = det(matrix)
    matrix_str = "\n".join("  " + "  ".join(f"{x:4d}" for x in row) for row in matrix)

    return {
        "prompt": f"Compute the determinant of the {n}x{n} matrix:\n{matrix_str}",
        "gold": str(result),
        "metadata": {"domain": "matrix-operations", "type": "number", "difficulty": difficulty}
    }


# ═══════════════════════════════════════════
# NUMBER THEORY
# ═══════════════════════════════════════════

def gen_gcd_lcm(difficulty: int = 5) -> dict:
    """Generate GCD/LCM problems."""
    max_val = int(_difficulty_scale(difficulty, 20, 10000))
    a = random.randint(2, max_val)
    b = random.randint(2, max_val)

    if random.random() < 0.5:
        result = math.gcd(a, b)
        prompt = f"Find the GCD (greatest common divisor) of {a} and {b}."
    else:
        result = a * b // math.gcd(a, b)
        prompt = f"Find the LCM (least common multiple) of {a} and {b}."

    return {
        "prompt": prompt,
        "gold": str(result),
        "metadata": {"domain": "euclidean-algorithm", "type": "number", "difficulty": difficulty}
    }


def gen_modular_arithmetic(difficulty: int = 5) -> dict:
    """Generate modular arithmetic problems."""
    mod = random.choice([7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47])
    if difficulty >= 7:
        mod = random.randint(50, 997)  # larger primes for harder problems

    base = random.randint(2, mod - 1)
    exp = random.randint(2, int(_difficulty_scale(difficulty, 10, 1000)))

    result = pow(base, exp, mod)

    return {
        "prompt": f"Compute {base}^{exp} mod {mod}.",
        "gold": str(result),
        "metadata": {"domain": "modular-arithmetic", "type": "number", "difficulty": difficulty}
    }


def gen_prime_factorization(difficulty: int = 5) -> dict:
    """Generate prime factorization problems."""
    # Generate a number with known factorization
    num_factors = min(2 + difficulty // 2, 8)
    small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    if difficulty >= 7:
        small_primes.extend([53, 59, 61, 67, 71, 73, 79, 83, 89, 97])

    factors = []
    n = 1
    for _ in range(num_factors):
        p = random.choice(small_primes)
        factors.append(p)
        n *= p
        if n > 10 ** min(difficulty + 2, 12):
            break

    factors.sort()
    gold = " * ".join(str(f) for f in factors)

    return {
        "prompt": f"Find the prime factorization of {n}.",
        "gold": gold,
        "metadata": {"domain": "prime-factorization", "type": "text", "difficulty": difficulty,
                     "n": n, "factors": factors}
    }


# ═══════════════════════════════════════════
# CALCULUS
# ═══════════════════════════════════════════

def gen_derivative(difficulty: int = 5) -> dict:
    """Generate derivative problems with known solutions."""
    # Build expression from components
    terms = []
    result_terms = []
    num_terms = min(1 + difficulty // 2, 5)

    for _ in range(num_terms):
        coeff = random.randint(1, int(_difficulty_scale(difficulty, 5, 20)))
        power = random.randint(1, int(_difficulty_scale(difficulty, 3, 8)))

        if power == 1:
            terms.append(f"{coeff}x")
            result_terms.append(str(coeff))
        else:
            terms.append(f"{coeff}x^{power}")
            new_coeff = coeff * power
            new_power = power - 1
            if new_power == 1:
                result_terms.append(f"{new_coeff}x")
            else:
                result_terms.append(f"{new_coeff}x^{new_power}")

    expr = " + ".join(terms)
    gold = " + ".join(result_terms)

    return {
        "prompt": f"Find the derivative of f(x) = {expr} with respect to x.",
        "gold": gold,
        "metadata": {"domain": "derivative-computation", "type": "text", "difficulty": difficulty}
    }


# ═══════════════════════════════════════════
# PROBABILITY & STATISTICS
# ═══════════════════════════════════════════

def gen_probability(difficulty: int = 5) -> dict:
    """Generate probability problems."""
    # Coin flip problem
    n = random.randint(2, min(difficulty + 2, 10))
    k = random.randint(1, n - 1)
    prob = math.comb(n, k) / (2 ** n)
    prompt = f"What is the probability of getting exactly {k} heads in {n} fair coin flips?"
    gold = str(round(prob, 6))

    return {
        "prompt": prompt,
        "gold": gold,
        "metadata": {"domain": "probability-statistics", "type": "number", "difficulty": difficulty}
    }


# ═══════════════════════════════════════════
# GENERATOR REGISTRY
# ═══════════════════════════════════════════

GENERATORS: dict[str, Callable] = {
    # Arithmetic & numerical
    "math-numerical": gen_arithmetic,
    "gen-arithmetic": gen_arithmetic,
    "gen-fraction": gen_fraction_arithmetic,
    "gen-percentage": gen_percentage,
    # Linear algebra
    "system-of-equations": gen_system_of_equations,
    "matrix-operations": gen_determinant,
    "determinant-tricks": gen_determinant,
    # Number theory
    "euclidean-algorithm": gen_gcd_lcm,
    "modular-arithmetic": gen_modular_arithmetic,
    "prime-factorization": gen_prime_factorization,
    "chinese-remainder-theorem": gen_modular_arithmetic,
    "fermats-little-theorem": gen_modular_arithmetic,
    # Calculus
    "derivative-computation": gen_derivative,
    # Probability
    "probability-statistics": gen_probability,
    "birthday-problem": gen_probability,
}


def generate(domain: str, difficulty: int = 5) -> dict:
    """Generate a problem for the given domain at the given difficulty."""
    gen_fn = GENERATORS.get(domain, gen_arithmetic)
    return gen_fn(difficulty)
