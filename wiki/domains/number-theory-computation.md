---
domain: Number Theory Computation
category: Math
verification_type: execution
dataset_scale: Unlimited (procedural generation)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

## Overview

Number theory computation encompasses problems where the answer is a specific integer (or set of integers) derived from number-theoretic operations: primality testing, factorization, modular arithmetic, GCD/LCM, Euler's totient, quadratic residues, Diophantine equations, continued fractions, and more. Verification is direct computation — the answer is a number and correctness is checked by executing the relevant algorithm.

This is a natural RLVR domain because verification is cheap and datasets are unlimited. Number theory problems appear heavily in math competitions (see `math-competition.md`), but the computational flavor here is distinct: the emphasis is on algorithmic thinking rather than creative insight. The domain provides a clean testbed for whether LLMs can learn number-theoretic algorithms via RL.

## Verification Mechanism

**Primary method: Direct computation of the correct answer and comparison.**

```python
import sympy
from sympy.ntheory import factorint, isprime, totient, mobius
from math import gcd

def verify_factorization(n: int, model_factors: dict) -> float:
    """
    Verify integer factorization.
    
    Args:
        n: The integer to factor
        model_factors: Dict mapping prime -> exponent, e.g., {2: 3, 7: 1} for 56
    
    Returns:
        1.0 if factorization is correct, 0.0 otherwise
    """
    # Check: product of factors equals n
    product = 1
    for prime, exp in model_factors.items():
        product *= prime ** exp
    if product != n:
        return 0.0
    
    # Check: all claimed factors are actually prime
    for prime in model_factors:
        if not isprime(prime):
            return 0.0
    
    # Check: factorization is complete (no repeated factors, all primes found)
    gold = factorint(n)
    if model_factors == gold:
        return 1.0
    
    return 0.0


def verify_modular_arithmetic(problem_type: str, model_answer: int, 
                               **params) -> float:
    """
    Verify modular arithmetic computations.
    """
    if problem_type == "modular_exponentiation":
        # Compute b^e mod m
        correct = pow(params["base"], params["exponent"], params["modulus"])
        return 1.0 if model_answer == correct else 0.0
    
    elif problem_type == "modular_inverse":
        # Verify: model_answer * a ≡ 1 (mod m)
        a, m = params["a"], params["m"]
        if (model_answer * a) % m == 1:
            return 1.0
        return 0.0
    
    elif problem_type == "discrete_log":
        # Verify: g^model_answer ≡ h (mod p)
        g, h, p = params["g"], params["h"], params["p"]
        if pow(g, model_answer, p) == h:
            return 1.0
        return 0.0
    
    elif problem_type == "chinese_remainder":
        # Verify: model_answer satisfies all congruences
        for a, m in zip(params["remainders"], params["moduli"]):
            if model_answer % m != a:
                return 0.0
        return 1.0


def verify_diophantine(equation_func, model_solution: tuple) -> float:
    """
    Verify a solution to a Diophantine equation.
    
    Args:
        equation_func: A function that takes the variables and returns
                       the equation value (should be 0 for valid solutions)
        model_solution: Tuple of integers
    """
    # Check all values are integers
    if not all(isinstance(x, int) for x in model_solution):
        return 0.0
    
    # Evaluate the equation
    result = equation_func(*model_solution)
    return 1.0 if result == 0 else 0.0


def verify_gcd_lcm(a: int, b: int, problem_type: str, model_answer: int) -> float:
    """Verify GCD or LCM computation."""
    if problem_type == "gcd":
        return 1.0 if model_answer == gcd(a, b) else 0.0
    elif problem_type == "lcm":
        return 1.0 if model_answer == (a * b) // gcd(a, b) else 0.0


def verify_totient(n: int, model_answer: int) -> float:
    """Verify Euler's totient function computation."""
    return 1.0 if model_answer == int(totient(n)) else 0.0


def verify_primality(n: int, model_answer: bool) -> float:
    """Verify primality claim."""
    return 1.0 if model_answer == isprime(n) else 0.0


def verify_nth_prime(n: int, model_answer: int) -> float:
    """Verify the nth prime number."""
    return 1.0 if model_answer == sympy.prime(n) else 0.0


def verify_sum_of_divisors(n: int, model_answer: int) -> float:
    """Verify sigma function (sum of divisors)."""
    return 1.0 if model_answer == sympy.divisor_sigma(n) else 0.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **Project Euler** | ~900 problems | [projecteuler.net](https://projecteuler.net/) | Many number theory problems with integer answers |
| **MATH** (number theory subset) | ~1,500 | hendrycks/math | Competition-style number theory |
| **OEIS** | ~370K sequences | [oeis.org](https://oeis.org/) | "Compute the nth term" tasks |
| **RSA Factoring Challenge** (historical) | ~50 | RSA Labs (archived) | Large semiprime factoring |
| **Cunningham tables** | ~10K | [homes.cerias.purdue.edu/~ssw/cun](https://homes.cerias.purdue.edu/~ssw/cun/) | Factorizations of b^n ± 1 |
| **Number theory textbook exercises** | ~5K | Various | Crawlable from textbook sites |
| **Procedurally generated** | Unlimited | Custom | Generate using SymPy |

### Procedural Generation

```python
def generate_number_theory_problem(difficulty: str = "medium"):
    """Generate a random number theory problem with verified answer."""
    
    problem_types = {
        "easy": [
            ("gcd", lambda: gen_gcd(max_val=1000)),
            ("primality", lambda: gen_primality(max_val=10000)),
            ("mod_arithmetic", lambda: gen_mod_arith(max_val=100)),
        ],
        "medium": [
            ("factorization", lambda: gen_factorization(n_digits=6)),
            ("totient", lambda: gen_totient(max_val=10000)),
            ("mod_exponentiation", lambda: gen_mod_exp(max_val=10**6)),
            ("crt", lambda: gen_chinese_remainder(n_congruences=3)),
        ],
        "hard": [
            ("factorization", lambda: gen_factorization(n_digits=12)),
            ("discrete_log", lambda: gen_discrete_log(prime_bits=20)),
            ("diophantine", lambda: gen_diophantine_equation()),
            ("quadratic_residue", lambda: gen_quad_residue(prime_bits=16)),
        ],
    }
    
    category, generator = random.choice(problem_types[difficulty])
    return generator()


def gen_factorization(n_digits=6):
    """Generate a factorization problem."""
    # Generate random primes and multiply them
    n_primes = random.randint(2, min(5, n_digits))
    primes = [sympy.randprime(10**(n_digits//n_primes - 1), 10**(n_digits//n_primes)) 
              for _ in range(n_primes)]
    n = 1
    for p in primes:
        n *= p
    
    return {
        "problem": f"Find the prime factorization of {n}.",
        "answer": factorint(n),
        "type": "factorization"
    }


def gen_chinese_remainder(n_congruences=3):
    """Generate a Chinese Remainder Theorem problem."""
    # Choose coprime moduli
    moduli = []
    for _ in range(n_congruences):
        while True:
            m = random.randint(2, 20)
            if all(gcd(m, existing) == 1 for existing in moduli):
                moduli.append(m)
                break
    
    remainders = [random.randint(0, m - 1) for m in moduli]
    
    # Compute answer via CRT
    from sympy.ntheory.modular import crt
    answer = crt(moduli, remainders)[0]
    
    congruences = [f"x ≡ {r} (mod {m})" for r, m in zip(remainders, moduli)]
    
    return {
        "problem": f"Find the smallest positive integer x satisfying:\n" + "\n".join(congruences),
        "answer": int(answer),
        "type": "chinese_remainder"
    }
```

## Task Format

**Input**: Natural language number theory problem.

```
Problem: Find the prime factorization of 2024.
Expected answer: 2^3 × 11 × 23

Problem: Compute 7^222 mod 31.
Expected answer: 28

Problem: Find the smallest positive integer x such that:
  x ≡ 2 (mod 3)
  x ≡ 3 (mod 5)
  x ≡ 2 (mod 7)
Expected answer: 23
```

**Output**: The computed answer, possibly with work shown.

## Difficulty Curriculum

| Level | Problem Type | Size of Inputs | Scale |
|-------|-------------|----------------|-------|
| Easy | GCD, primality of small numbers, basic mod | Numbers < 1,000 | Unlimited |
| Medium | Factorization (6 digits), CRT, totient | Numbers < 10^6 | Unlimited |
| Hard | Factorization (12+ digits), discrete log, Diophantine | Numbers < 10^12 | Unlimited |
| Very Hard | Factorization (20+ digits), Pell equations | Numbers < 10^20 | Unlimited |
| Superhuman | RSA-scale factorization, unsolved Diophantine equations | Numbers > 10^50 | N/A (intractable) |

**Key insight**: Difficulty scales with the size of the numbers. An LLM must learn to execute multi-step algorithms mentally, which gets harder as numbers grow. This provides a smooth difficulty ramp.

## Limitations & Risks

1. **Calculator dependence**: LLMs are bad at arithmetic with large numbers. A model that "understands" Euler's totient formula may still fail because it can't multiply 17 × 23 correctly. This conflates algorithmic understanding with arithmetic skill.
2. **Memorization of small cases**: Models may memorize prime factorizations of common numbers from training data rather than learning algorithms. Testing on random large numbers mitigates this.
3. **Asymptotic hardness**: Factoring and discrete log are believed to be computationally hard (basis of cryptography). Expecting an LLM to factor RSA-size numbers is unreasonable. The difficulty range must be calibrated to what can be done in ~100 mental arithmetic steps.
4. **Limited reasoning depth**: Many number theory algorithms (Euclidean algorithm, extended GCD, CRT) require a fixed sequence of steps. LLMs can learn these patterns, but the chain-of-thought length needed grows with input size.
5. **Narrow domain**: Pure number theory computation, while clean, may not transfer to broader mathematical reasoning.

## Connections

- **math-competition.md**: Number theory competition problems require both computational skill and creative insight. This domain provides the computational substrate.
- **math-numerical.md**: Overlaps at the easy end (basic arithmetic). Number theory adds structure (primes, modular arithmetic).
- **abstract-algebra.md**: Modular arithmetic is group/ring theory in disguise. Quadratic residues connect to field theory.
- **math-formal-proofs.md**: Number-theoretic results can be formally verified in Lean4 (Mathlib has extensive number theory).
- **logic-propositional.md**: Factoring can be encoded as SAT (though this is not practical for large numbers).
