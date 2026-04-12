---
domain: Propositional Logic / SAT
category: Logic
verification_type: execution
dataset_scale: Unlimited (procedural generation)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

## Overview

Propositional logic and Boolean satisfiability (SAT) is one of the most natural RLVR domains: given a Boolean formula, find a satisfying assignment (or prove unsatisfiability). Verification is trivial — plug the assignment into the formula and evaluate. SAT is NP-complete, so hard instances exist at every scale, and the phase transition phenomenon provides a natural difficulty curriculum.

Despite being a perfect fit for RLVR, this domain is underexplored in the LLM context. Most SAT solving is done by dedicated solvers (MiniSat, CaDiCaL, Kissat). The research question is whether LLMs can learn useful heuristics for SAT via RL that complement or improve upon traditional solvers, or whether the domain serves as a clean reasoning benchmark.

## Verification Mechanism

**Primary method: Direct evaluation of the assignment on the formula.**

```python
def verify_sat_assignment(formula_cnf: list, assignment: dict) -> float:
    """
    Verify a SAT assignment.
    
    Args:
        formula_cnf: CNF formula as list of clauses. 
                     Each clause is a list of signed integers.
                     E.g., [[1, -2, 3], [-1, 4]] means (x1 OR NOT x2 OR x3) AND (NOT x1 OR x4)
        assignment: dict mapping variable number to True/False.
                    E.g., {1: True, 2: False, 3: True, 4: True}
    
    Returns:
        1.0 if assignment satisfies all clauses, 0.0 otherwise.
    """
    for clause in formula_cnf:
        clause_satisfied = False
        for literal in clause:
            var = abs(literal)
            if var not in assignment:
                return 0.0  # Variable not assigned
            value = assignment[var]
            if literal > 0 and value:
                clause_satisfied = True
                break
            elif literal < 0 and not value:
                clause_satisfied = True
                break
        if not clause_satisfied:
            return 0.0  # Unsatisfied clause
    
    return 1.0


def verify_unsat_proof(formula_cnf: list, proof: str, proof_format: str = "drat") -> float:
    """
    Verify an unsatisfiability proof using a proof checker.
    DRAT (Deletion Resolution Asymmetric Tautology) is the standard format.
    """
    import subprocess, tempfile
    
    # Write DIMACS CNF file
    with tempfile.NamedTemporaryFile(suffix='.cnf', mode='w', delete=False) as cnf_file:
        n_vars = max(abs(l) for clause in formula_cnf for l in clause)
        cnf_file.write(f"p cnf {n_vars} {len(formula_cnf)}\n")
        for clause in formula_cnf:
            cnf_file.write(' '.join(map(str, clause)) + ' 0\n')
        cnf_path = cnf_file.name
    
    # Write proof file
    with tempfile.NamedTemporaryFile(suffix='.drat', mode='w', delete=False) as proof_file:
        proof_file.write(proof)
        proof_path = proof_file.name
    
    # Verify with drat-trim
    result = subprocess.run(
        ['drat-trim', cnf_path, proof_path],
        capture_output=True, text=True, timeout=60
    )
    
    return 1.0 if 's VERIFIED' in result.stdout else 0.0


def verify_sat_task(formula_cnf: list, model_output: str) -> float:
    """
    Full verification: model either provides a satisfying assignment or claims UNSAT.
    """
    if model_output.strip().upper().startswith("UNSAT"):
        # Check with a SAT solver to confirm
        from pysat.solvers import Glucose4
        solver = Glucose4()
        for clause in formula_cnf:
            solver.add_clause(clause)
        if solver.solve():
            return 0.0  # Model said UNSAT but it's SAT — wrong
        return 1.0  # Correctly identified as UNSAT
    else:
        # Parse assignment from model output
        assignment = parse_assignment(model_output)
        return verify_sat_assignment(formula_cnf, assignment)
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **SATLIB** | ~3,600 instances | [cs.ubc.ca/~hoos/SATLIB](https://www.cs.ubc.ca/~hoos/SATLIB/) | Benchmark library. Random, structured, industrial. |
| **SAT Competition benchmarks** | ~10K | [satcompetition.org](https://satcompetition.org/) | Annual competition instances (2002-2024) |
| **Random 3-SAT** | Unlimited | Procedural | Phase transition at clause/variable ratio ~4.267 |
| **Random k-SAT** | Unlimited | Procedural | Generalized to arbitrary clause width |
| **Graph coloring → SAT** | Unlimited | Reduction | Encode graph coloring as SAT |
| **Pigeonhole formulas** | Unlimited | Procedural | Classic hard UNSAT instances |
| **Tseitin formulas** | Unlimited | Procedural | Structured hard instances from graphs |
| **SATBench** (various) | Varies | Research papers | Various SAT benchmarks for ML |
| **Industrial SAT** | ~5K | SAT competitions | From hardware/software verification. Very hard. |

### Procedural Generation with Difficulty Control

```python
import random

def generate_random_3sat(n_vars: int, clause_ratio: float = 4.267):
    """
    Generate random 3-SAT instance near the phase transition.
    
    At clause_ratio ≈ 4.267, ~50% of instances are satisfiable.
    Below this ratio: almost certainly SAT (easy to satisfy).
    Above this ratio: almost certainly UNSAT (easy to prove UNSAT if small).
    Right at the threshold: hardest instances.
    
    Args:
        n_vars: Number of Boolean variables
        clause_ratio: Ratio of clauses to variables
    """
    n_clauses = int(n_vars * clause_ratio)
    clauses = []
    for _ in range(n_clauses):
        vars_in_clause = random.sample(range(1, n_vars + 1), 3)
        clause = [v * random.choice([1, -1]) for v in vars_in_clause]
        clauses.append(clause)
    return clauses

def generate_curriculum():
    """Generate a difficulty curriculum for SAT."""
    levels = [
        # Easy: small SAT instances well below phase transition
        {"n_vars": 10, "ratio": 3.0, "label": "easy"},
        # Medium: moderate size at phase transition
        {"n_vars": 20, "ratio": 4.267, "label": "medium"},
        # Hard: larger at phase transition
        {"n_vars": 50, "ratio": 4.267, "label": "hard"},
        # Very hard: large phase transition
        {"n_vars": 100, "ratio": 4.267, "label": "very_hard"},
        # Superhuman: industrial-scale
        {"n_vars": 1000, "ratio": 4.267, "label": "superhuman"},
    ]
    return levels
```

## Task Format

**Input**: CNF formula in DIMACS or natural language.

```
Problem: Find a satisfying assignment for the following Boolean formula in CNF:
(x1 OR NOT x2 OR x3) AND (NOT x1 OR x2 OR NOT x3) AND (x2 OR x3 OR x4) 
AND (NOT x1 OR NOT x3 OR NOT x4) AND (x1 OR NOT x2 OR x4)

Variables: x1, x2, x3, x4
```

**Output**:
```
x1 = True, x2 = True, x3 = False, x4 = True
```

Alternative format (DIMACS-style):
```
Input: p cnf 4 5
1 -2 3 0
-1 2 -3 0
2 3 4 0
-1 -3 -4 0
1 -2 4 0

Output: v 1 1 -1 1 0
(meaning x1=T, x2=T, x3=F, x4=T)
```

## Difficulty Curriculum

| Level | n_vars | Clause ratio | Approx. solver time | Notes |
|-------|--------|-------------|---------------------|-------|
| Easy | 5-15 | 3.0-3.5 | <1ms | Almost certainly SAT, easy to find |
| Medium | 15-30 | 4.0-4.5 | 1-100ms | Phase transition, genuine search needed |
| Hard | 30-75 | 4.267 | 100ms-10s | Requires backtracking |
| Very Hard | 75-200 | 4.267 | 10s-minutes | Serious search tree |
| Superhuman | 200+ at phase transition, industrial | 4.267+ | Hours+ | Beyond current LLM capability |

## Limitations & Risks

1. **LLMs are bad at SAT**: Current LLMs perform very poorly on even small SAT instances (>20 variables). The combinatorial search required is fundamentally different from pattern matching. RLVR may help, but breakthroughs are not guaranteed.
2. **Encoding matters enormously**: The same logical problem can be encoded as SAT in many ways. A model learning SAT directly may not transfer to the underlying problem.
3. **Solver comparison trap**: SAT solvers (CaDiCaL, Kissat) are extraordinarily optimized. An LLM will likely never beat them on raw SAT. The value proposition must be different — e.g., the LLM learns to *formulate* the problem or guide the solver.
4. **UNSAT verification is hard**: Verifying a satisfying assignment is O(n). Verifying unsatisfiability requires a proof (DRAT format), which is much harder for a model to produce.
5. **Degenerate solutions on easy instances**: Well below the phase transition, almost any random assignment works. The model may learn random guessing rather than reasoning.

## Connections

- **logic-first-order.md**: FOL generalizes propositional logic. SAT is a special case of SMT.
- **logic-puzzles.md**: Many constraint puzzles (Sudoku, etc.) can be encoded as SAT instances.
- **combinatorics-optimization.md**: Many combinatorial optimization problems reduce to SAT/MaxSAT.
- **logic-formal-specification.md**: Model checking often reduces to SAT (bounded model checking).
- **automated-reasoning.md**: SAT solving is a core primitive in automated reasoning.
