---
domain: Modular Arithmetic & Finite Field Computation
category: math-algebra
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Modular Arithmetic & Finite Field Computation

## Overview
Compute in modular arithmetic and finite fields: modular exponentiation, extended Euclidean algorithm, Chinese Remainder Theorem, discrete logarithms, polynomial arithmetic over finite fields, elliptic curve point operations. Verification: direct computation.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: Any) -> float:
    if task_type == "modular_exp":
        correct = pow(problem["base"], problem["exp"], problem["mod"])
        return 1.0 if answer == correct else 0.0
    
    elif task_type == "mod_inverse":
        if (answer * problem["a"]) % problem["n"] == 1:
            return 1.0
        return 0.0
    
    elif task_type == "crt":
        # Verify CRT solution satisfies all congruences
        for mod, remainder in problem["congruences"]:
            if answer % mod != remainder:
                return 0.0
        return 1.0
    
    elif task_type == "ec_point_add":
        # Verify elliptic curve point addition
        p, a, b = problem["p"], problem["a"], problem["b"]
        P, Q = problem["P"], problem["Q"]
        correct = ec_add(P, Q, a, p)
        return 1.0 if answer == correct else 0.0
    
    elif task_type == "gf_polynomial":
        # Finite field polynomial arithmetic
        correct = gf_poly_op(problem["op"], problem["poly1"], problem["poly2"], problem["field"])
        return 1.0 if answer == correct else 0.0
```

## Dataset Sources
- **Number theory textbooks**: Hardy & Wright, Ireland & Rosen.
- **Cryptography courses**: Extensive exercises.
- **Competition math**: Number theory problems from AMC/AIME/IMO.
- **Procedural generation**: Unlimited — sample random integers, compute.
- **Elliptic curve databases**: LMFDB (L-functions and Modular Forms DataBase).

## Task Format
- **Input**: "Compute 7^1000 mod 13"
- **Output**: "9"

## Difficulty Curriculum
- Level 1: Basic modular arithmetic (+, ×, mod)
- Level 3: Extended Euclidean, modular inverse
- Level 5: CRT, Euler's theorem applications
- Level 7: Finite field polynomial arithmetic
- Level 9: Elliptic curve operations, quadratic reciprocity

## Connections
- [[number-theory-computation]] — number theory
- [[mathematical-cryptanalysis]] — crypto uses modular arithmetic
- [[abstract-algebra]] — algebraic structures
