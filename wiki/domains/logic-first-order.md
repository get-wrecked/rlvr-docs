---
domain: First-Order Logic
category: Logic
verification_type: execution
dataset_scale: ~100K+ (TPTP library + procedural generation)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

## Overview

First-order logic (FOL) theorem proving and model checking extends propositional logic with quantifiers (forall, exists), predicates, and functions. Verification uses Automated Theorem Provers (ATPs) like Vampire, E, Z3, or CVC5. The model generates either a proof, a satisfying model, or a claim of (un)satisfiability, and the ATP/SMT solver confirms.

This domain sits at the intersection of logic and mathematics. FOL is the foundation of formal methods, and SMT (Satisfiability Modulo Theories) extends it with arithmetic, arrays, bit-vectors, etc. The TPTP library provides a large, well-curated benchmark. FOL theorem proving has a 50+ year research history with annual competitions (CASC), but LLM-based approaches are nascent.

## Verification Mechanism

**Primary method: ATP/SMT solver confirms the claim or checks the proof.**

```python
import subprocess
import tempfile

def verify_fol_theorem(axioms: list, conjecture: str, model_answer: str,
                       timeout: int = 60) -> float:
    """
    Verify a first-order logic theorem proving task.
    
    The model claims the conjecture is a theorem (provable from axioms)
    or not a theorem (countermodel exists).
    """
    if model_answer.strip().lower().startswith("theorem"):
        # Model claims it's provable — verify with ATP
        return verify_proof_with_atp(axioms, conjecture, timeout)
    elif model_answer.strip().lower().startswith("countermodel"):
        # Model claims it's not a theorem — verify countermodel
        model_structure = parse_model(model_answer)
        return verify_countermodel(axioms, conjecture, model_structure)
    else:
        return 0.0


def verify_proof_with_atp(axioms: list, conjecture: str, timeout: int = 60) -> float:
    """Use Vampire or E prover to confirm the conjecture follows from axioms."""
    # Write TPTP format problem
    tptp_content = ""
    for i, ax in enumerate(axioms):
        tptp_content += f"fof(axiom{i}, axiom, {ax}).\n"
    tptp_content += f"fof(conjecture, conjecture, {conjecture}).\n"
    
    with tempfile.NamedTemporaryFile(suffix='.p', mode='w', delete=False) as f:
        f.write(tptp_content)
        f.flush()
        
        # Run Vampire
        result = subprocess.run(
            ['vampire', '--mode', 'casc', '-t', str(timeout), f.name],
            capture_output=True, text=True, timeout=timeout + 5
        )
        
        if '% SZS status Theorem' in result.stdout:
            return 1.0
        return 0.0


def verify_with_z3(formula_smt2: str, model_answer: str) -> float:
    """
    Verify using Z3 SMT solver.
    For satisfiability: check if the model's assignment satisfies all constraints.
    For validity: check if the negation is unsatisfiable.
    """
    from z3 import Solver, parse_smt2_string, sat, unsat
    
    solver = Solver()
    assertions = parse_smt2_string(formula_smt2)
    solver.add(assertions)
    
    if model_answer.strip().lower() == "sat":
        result = solver.check()
        return 1.0 if result == sat else 0.0
    elif model_answer.strip().lower() == "unsat":
        result = solver.check()
        return 1.0 if result == unsat else 0.0
    else:
        # Model provides a satisfying assignment — verify it
        assignment_constraints = parse_assignment_to_smt2(model_answer)
        solver.add(parse_smt2_string(assignment_constraints))
        result = solver.check()
        return 1.0 if result == sat else 0.0


def verify_countermodel(axioms: list, conjecture: str, model_structure: dict) -> float:
    """
    Verify a countermodel: the model structure must satisfy all axioms
    but falsify the conjecture.
    """
    # Check all axioms are true in the structure
    for ax in axioms:
        if not evaluate_fol_in_structure(ax, model_structure):
            return 0.0  # Axiom violated
    
    # Check conjecture is false
    if evaluate_fol_in_structure(conjecture, model_structure):
        return 0.0  # Conjecture is true — not a countermodel
    
    return 1.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **TPTP** (Thousands of Problems for Theorem Provers) | ~25,000 | [tptp.org](https://www.tptp.org/) | The gold standard FOL benchmark. Problems rated by difficulty. Annual CASC competition. |
| **TPTP FOF subset** | ~12,000 | tptp.org | First-order form subset |
| **SMT-LIB** | ~100K+ | [smtlib.cs.uiowa.edu](https://smtlib.cs.uiowa.edu/) | SMT benchmarks in SMT-LIB2 format. QF_LIA, QF_NRA, etc. |
| **SMT Competition benchmarks** | ~50K | [smt-comp.github.io](https://smt-comp.github.io/) | Annual competition instances |
| **FOLIO** (Han et al., 2022) | 1,435 | [github.com/Yale-LILY/FOLIO](https://github.com/Yale-LILY/FOLIO) | NL premises → FOL reasoning tasks |
| **ProverBot9001 data** | ~15K | [github.com/UCSD-PL/proverbot9001](https://github.com/UCSD-PL/proverbot9001) | Coq proof automation data |
| **LogicNLI** (Tian et al., 2021) | 30,000 | GitHub | Natural language inference via FOL |
| **RuleTaker** (Clark et al., 2020) | ~500K | Allen AI | Synthetic FOL reasoning chains |
| **Procedural generation** | Unlimited | Custom | Generate random FOL formulas with controlled structure |

### Procedural Generation

```python
import random

def generate_fol_problem(n_predicates=3, n_constants=4, n_axioms=5, depth=2):
    """
    Generate a random FOL problem.
    Strategy: generate random axioms, derive a consequence, use it as the conjecture.
    """
    predicates = [f"p{i}" for i in range(n_predicates)]  # unary/binary predicates
    constants = [f"c{i}" for i in range(n_constants)]
    variables = ['X', 'Y', 'Z']
    
    axioms = []
    for _ in range(n_axioms):
        axiom = generate_random_fol_sentence(predicates, constants, variables, depth)
        axioms.append(axiom)
    
    # Use ATP to find a non-trivial consequence
    conjecture = find_nontrivial_consequence(axioms, predicates, constants)
    
    return {"axioms": axioms, "conjecture": conjecture, "status": "theorem"}

def generate_smt_problem(theory="QF_LIA", n_vars=5, n_constraints=8):
    """Generate SMT problem in a given theory."""
    if theory == "QF_LIA":  # Quantifier-free linear integer arithmetic
        vars = [f"x{i}" for i in range(n_vars)]
        constraints = []
        for _ in range(n_constraints):
            # Random linear constraint: a1*x1 + a2*x2 + ... <= b
            coeffs = [random.randint(-5, 5) for _ in vars]
            bound = random.randint(-10, 10)
            op = random.choice(["<=", ">=", "="])
            lhs = " + ".join(f"(* {c} {v})" for c, v in zip(coeffs, vars) if c != 0)
            constraints.append(f"({op} (+ {lhs}) {bound})")
        return constraints
```

## Task Format

**Input**: FOL axioms + conjecture in TPTP syntax or natural language.

```
TPTP format:
fof(axiom1, axiom, ![X]: (human(X) => mortal(X))).
fof(axiom2, axiom, human(socrates)).
fof(conjecture, conjecture, mortal(socrates)).

Task: Is the conjecture a theorem? If yes, provide a proof sketch. If no, provide a countermodel.
```

```
Natural language format:
Axioms:
1. All humans are mortal.
2. All Greeks are human.
3. Socrates is Greek.

Conjecture: Socrates is mortal.

Task: Determine if the conjecture follows from the axioms. Justify your answer.
```

**Output**: "Theorem" or "Not a theorem" with justification.

## Difficulty Curriculum

| Level | Source | ATP solve time | Scale |
|-------|--------|----------------|-------|
| Easy | Propositional-like FOL, RuleTaker depth 1-2 | <0.1s | Unlimited |
| Medium | TPTP rating 0.0-0.3, SMT QF_LIA small | 0.1-1s | ~5K |
| Hard | TPTP rating 0.3-0.7, SMT QF_NRA | 1s-60s | ~5K |
| Very Hard | TPTP rating 0.7-1.0 | 60s-timeout | ~3K |
| Superhuman | Open TPTP problems, undecidable fragments | N/A | ~500 |

TPTP rates each problem from 0.0 (trivial) to 1.0 (unsolved) based on which provers can solve it, providing a natural curriculum.

## Limitations & Risks

1. **LLMs struggle with formal syntax**: Generating correct TPTP or SMT-LIB syntax is itself a challenge. Models may produce syntactically invalid output. A preprocessing step to fix syntax may help.
2. **Undecidability**: Full FOL is undecidable (validity, satisfiability). The ATP may time out on valid theorems. This means the verifier can produce false negatives — the model's answer may be correct but unverifiable within the time limit.
3. **Proof generation is much harder than claim generation**: Saying "theorem" is easy; generating the actual proof (in TSTP format) that the proof checker accepts is much harder. The task design must be carefully scoped.
4. **Solver dominance**: ATPs are extremely powerful on their native benchmarks. The value of an LLM is in *translation* (NL to FOL), *guidance* (helping the ATP focus), or *generalization* (handling problems outside the ATP's reach).
5. **Limited LLM training data**: FOL and TPTP syntax are rare in pretraining corpora compared to natural language or even programming languages.

## Connections

- **logic-propositional.md**: Propositional logic is the quantifier-free Boolean fragment of FOL.
- **math-formal-proofs.md**: Lean/Coq proofs involve higher-order logic, which subsumes FOL. FOL theorem proving can be seen as a simpler version.
- **logic-formal-specification.md**: Formal specification checking often reduces to FOL/SMT queries.
- **automated-reasoning.md**: FOL theorem proving is a core automated reasoning task.
- **logic-puzzles.md**: Many logic puzzles can be encoded in FOL.
