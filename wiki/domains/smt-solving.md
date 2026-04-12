---
domain: SMT Solving / Satisfiability Modulo Theories
category: formal-methods
verification_type: execution
dataset_scale: 100K+ (SMT-LIB benchmarks)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# SMT Solving / Satisfiability Modulo Theories

## Overview
SMT extends SAT solving with theories: integers, reals, bit-vectors, arrays, strings, datatypes. Given a formula in SMT-LIB format, find a satisfying assignment or prove unsatisfiability. Verification: plug the assignment back into the formula, or verify the UNSAT proof.

## Verification Mechanism
```python
from z3 import *

def verify(formula_smtlib: str, response: dict) -> float:
    solver = Solver()
    solver.from_string(formula_smtlib)
    
    if response["verdict"] == "sat":
        # Check satisfying assignment
        model = response["model"]  # variable -> value mapping
        for var, value in model.items():
            solver.add(Symbol(var) == value)
        result = solver.check()
        return 1.0 if result == sat else 0.0
    
    elif response["verdict"] == "unsat":
        # Verify unsatisfiability (run solver to confirm)
        result = solver.check()
        return 1.0 if result == unsat else 0.0
    
    return 0.0
```

For SAT assignments specifically:
```python
def verify_sat_assignment(cnf: list[list[int]], assignment: dict[int, bool]) -> float:
    for clause in cnf:
        satisfied = False
        for lit in clause:
            var = abs(lit)
            val = assignment.get(var, False)
            if (lit > 0 and val) or (lit < 0 and not val):
                satisfied = True
                break
        if not satisfied:
            return 0.0
    return 1.0
```

## Dataset Sources
- **SMT-LIB**: Official benchmark library with 100K+ benchmarks across theories.
  - QF_LIA: Quantifier-free linear integer arithmetic
  - QF_BV: Bit-vectors
  - QF_S: Strings
  - QF_AUFLIA: Arrays + integers
- **SAT Competition benchmarks**: Annual SAT competition instances.
- **Software verification benchmarks**: SV-COMP instances (software model checking).
- **Procedural generation**: Random formula generation with controlled difficulty (clause/variable ratio for SAT, theory combination complexity for SMT).
- **Hardware verification instances**: Circuit equivalence checking benchmarks.

## Task Format
- **Input**: SMT-LIB formula (or natural language description of constraints)
- **Output**: "SAT" + satisfying assignment, or "UNSAT"

## Difficulty Curriculum
- Level 1: Small propositional SAT (10 variables, 30 clauses)
- Level 3: QF_LIA with 50 variables
- Level 5: Bit-vector formulas from software verification
- Level 7: Quantified formulas, theory combinations
- Level 9: Hard industrial instances (10K+ variables)
- Level 10: Open instances from SAT/SMT competitions

## Limitations & Risks
- Verifying SAT is easy (check assignment). Verifying UNSAT is harder (need to verify proof certificate or trust an independent solver).
- For UNSAT verification, run Z3/CVC5 independently as oracle. If both agree, high confidence.
- At extreme difficulty, even state-of-the-art solvers time out. These are genuinely hard problems.

## Connections
- [[logic-propositional]] — SAT is a special case of SMT
- [[logic-first-order]] — FOL is more expressive but related
- [[logic-formal-specification]] — specifications often reduce to SMT queries
- [[combinatorics-optimization]] — many optimization problems encode as SMT
