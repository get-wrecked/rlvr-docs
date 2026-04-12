---
domain: Theorem Conjecturing
category: Math
verification_type: execution
dataset_scale: ~10K verifiable conjectures (expandable)
difficulty_range: hard/superhuman
modality: text
status: needs_validation
---

## Overview

Theorem conjecturing asks the model to generate true, non-trivial mathematical statements — not to prove them, but to state them. This is the creative side of mathematics: pattern recognition, generalization, analogy. Verification is the hard part: the conjectured statement must be (a) true (or at least not provably false) and (b) non-trivial (not a tautology or trivial consequence of definitions).

This is a frontier RLVR domain. While some work exists (e.g., Ramanujan Machine, AI Mathematician conjecturing), there is no established RLVR pipeline. The verification mechanism is necessarily composite: use CAS to check specific cases, formal provers to attempt proofs, and heuristic non-triviality filters. The reward signal is noisy but potentially very high-value.

## Verification Mechanism

**Primary method: Multi-stage verification pipeline.**

Verification of conjectures requires checking both correctness and non-triviality, which cannot be done by a single tool.

```python
import sympy as sp
from itertools import product

def verify_conjecture(conjecture_str: str, domain: str = "number_theory") -> float:
    """
    Verify a mathematical conjecture.
    
    Returns:
        1.0 if conjecture is verified as true AND non-trivial
        0.5 if conjecture is true but possibly trivial
        0.0 if conjecture is false or unparseable
    """
    try:
        conjecture = parse_conjecture(conjecture_str)
    except ParseError:
        return 0.0
    
    # Stage 1: Check for triviality
    triviality_score = check_triviality(conjecture)
    if triviality_score > 0.8:
        return 0.0  # Too trivial (tautology, known identity, etc.)
    
    # Stage 2: Empirical verification — check many cases
    empirical_result = empirical_check(conjecture, n_cases=10000)
    if not empirical_result:
        return 0.0  # Found counterexample
    
    # Stage 3: Attempt formal proof (optional, expensive)
    formal_result = attempt_formal_proof(conjecture, timeout=30)
    
    if formal_result == "proved":
        # True and non-trivial (if the proof is non-trivial)
        return 1.0
    elif formal_result == "disproved":
        return 0.0
    else:
        # Could not prove or disprove — empirically true, give partial credit
        return 0.5


def empirical_check(conjecture, n_cases=10000):
    """
    Check the conjecture on many specific instances.
    
    For universally quantified statements (forall n: P(n)),
    check P(n) for n = 1, 2, ..., n_cases.
    
    For existential statements (exists n: P(n)),
    search for a witness.
    """
    if conjecture.quantifier == "forall":
        for case in generate_test_cases(conjecture.variables, n_cases):
            if not evaluate_predicate(conjecture.predicate, case):
                return False  # Counterexample found
        return True  # No counterexample in n_cases trials
    
    elif conjecture.quantifier == "exists":
        for case in generate_test_cases(conjecture.variables, n_cases):
            if evaluate_predicate(conjecture.predicate, case):
                return True  # Witness found
        return False  # No witness found (might still be true)


def check_triviality(conjecture) -> float:
    """
    Heuristic triviality score.
    
    Returns value in [0, 1] where 1 = definitely trivial.
    """
    trivial_patterns = [
        # Tautologies
        lambda c: c.is_tautology(),
        # Direct consequences of definitions
        lambda c: c.is_definitional(),
        # Already in standard reference (e.g., OEIS, known theorems)
        lambda c: c.matches_known_result(),
        # Trivial specialization of known theorem
        lambda c: c.is_specialization_of_known(),
    ]
    
    score = 0.0
    for check in trivial_patterns:
        try:
            if check(conjecture):
                score = max(score, 0.9)
        except:
            pass
    
    # Also check: is this just a numerical identity?
    # e.g., "1 + 1 = 2" is true but trivial
    if conjecture.is_ground_equation():
        score = max(score, 0.95)
    
    return score


def attempt_formal_proof(conjecture, timeout=30):
    """
    Try to prove or disprove using available tools.
    """
    # Try SymPy simplification
    try:
        result = sp.simplify(conjecture.as_sympy())
        if result == True:
            return "proved"
        if result == False:
            return "disproved"
    except:
        pass
    
    # Try Z3 for decidable fragments
    try:
        from z3 import Solver, Int, ForAll, Exists
        z3_formula = conjecture.to_z3()
        solver = Solver()
        solver.set("timeout", timeout * 1000)
        
        # Check if negation is satisfiable (to disprove)
        solver.add(z3_formula.negation())
        result = solver.check()
        if result.r == -1:  # unsat = no counterexample = proved
            return "proved"
        elif result.r == 1:  # sat = counterexample exists = disproved
            return "disproved"
    except:
        pass
    
    return "unknown"
```

### Specific Conjecture Types with Tailored Verification

```python
def verify_number_theory_conjecture(conjecture: str) -> float:
    """
    Example conjectures:
    - "For all primes p > 3, p^2 - 1 is divisible by 24"
    - "The sum of first n cubes equals the square of the sum of first n integers"
    """
    # Parse the conjecture
    parsed = parse_nt_conjecture(conjecture)
    
    # Check up to large bounds
    for n in range(1, 100000):
        if not parsed.evaluate(n):
            return 0.0  # Counterexample
    
    # Check if this is a known result
    if matches_known_theorem(parsed):
        return 0.0  # Already known — not novel
    
    # Check non-triviality: does the statement have content?
    if parsed.is_vacuously_true():
        return 0.0
    
    return 1.0 if attempt_proof(parsed) else 0.5


def verify_sequence_conjecture(conjecture: str) -> float:
    """
    Conjectures about integer sequences.
    E.g., "The nth term of the sequence satisfying a(n) = a(n-1) + a(n-2) 
           with a(0)=2, a(1)=1 satisfies a(n) = L(n) where L(n) is the 
           nth Lucas number."
    
    Verify by:
    1. Computing the sequence both ways (recurrence and closed form)
    2. Checking agreement for many terms
    3. Cross-referencing with OEIS
    """
    seq_recurrence = compute_from_recurrence(conjecture.recurrence, n_terms=1000)
    seq_formula = compute_from_formula(conjecture.formula, n_terms=1000)
    
    if seq_recurrence != seq_formula:
        return 0.0
    
    # Check OEIS for novelty
    oeis_match = lookup_oeis(seq_recurrence[:10])
    if oeis_match and conjecture.formula in oeis_match.formulas:
        return 0.0  # Already known
    
    return 1.0
```

## Dataset Sources

| Dataset | Source | Notes |
|---------|--------|-------|
| **OEIS** (On-Line Encyclopedia of Integer Sequences) | [oeis.org](https://oeis.org/) | ~370K sequences. Rich source for conjecture generation: find patterns, state them. |
| **Ramanujan Machine** conjectures | [github.com/RamanujanMachine](https://github.com/RamanujanMachine) | Computer-generated conjectures about fundamental constants |
| **Lean4 Mathlib** (as negative examples) | GitHub | Known theorems = "already conjectured and proved." Useful for triviality filtering. |
| **Open problems in mathematics** | Various surveys | Provide targets: can the model rediscover known open problems? |
| **Mathematical competition conjectures** | AoPS, Olympiad folklore | "Determine all functions satisfying..." type problems |
| **Experimental mathematics literature** | Various journals | Conjectures from computational experiments |
| **Sloane's Gap and OEIS analysis** | [oeis.org/wiki](https://oeis.org/wiki/Main_Page) | Patterns in sequences that suggest conjectures |

**Note**: This domain has no standard benchmark dataset. Creating one is itself a research contribution.

## Task Format

**Input**: A mathematical context or seed.

```
Context: Consider the sequence defined by a(1) = 1, a(2) = 1, and 
a(n) = a(n-1) + a(n-2) + a(floor(n/2)) for n >= 3.

Task: State a non-trivial true conjecture about this sequence.
```

```
Context: Let p be a prime number greater than 3.

Task: State a non-trivial true conjecture about p^2 mod 12.
```

**Output**: A precise mathematical statement.

```
Conjecture: For all primes p > 3, p^2 ≡ 1 (mod 24).

Justification: Every prime p > 3 is of the form 6k ± 1. 
Then p^2 = 36k^2 ± 12k + 1 = 12k(3k ± 1) + 1. 
Since k or 3k ± 1 is even, 12k(3k ± 1) is divisible by 24.
```

## Difficulty Curriculum

| Level | Type | Verification | Scale |
|-------|------|-------------|-------|
| Easy | Pattern in small sequences (OEIS lookup) | Compute first 1000 terms + OEIS check | ~10K |
| Medium | Number theory identities | Empirical check + CAS proof | ~2K |
| Hard | Novel conjectures about known objects | Empirical + failed formal proof (i.e., genuinely open) | ~500 |
| Superhuman | New research-level conjectures | Expert human evaluation needed | Open-ended |

## Limitations & Risks

1. **Triviality detection is unsolved**: Determining whether a conjecture is "interesting" or "non-trivial" is inherently subjective. No algorithm can reliably distinguish `p^2 ≡ 1 (mod 24)` (non-trivial, teaches something) from `n + 0 = n` (trivial). Heuristic filters will have both false positives and false negatives.
2. **Verification is incomplete**: Empirical checking can only test finitely many cases. A conjecture that holds for n = 1 to 1,000,000 may fail at n = 1,000,001 (e.g., some Mertens-conjecture-style failures). The reward signal is inherently noisy.
3. **Reward hacking via triviality**: The model may learn to generate trivially true statements that pass the triviality filter. Adversarial triviality checking (using another model to judge) is one mitigation.
4. **No ground truth dataset**: Unlike other domains, there is no standard "conjecture dataset" to train on. The task definition itself is ambiguous.
5. **Very sparse reward**: Most randomly generated mathematical statements are either trivially true, trivially false, or undecidable. Finding the sweet spot of "true, interesting, and verifiable" is needle-in-haystack.
6. **Philosophical issues**: What counts as a "conjecture"? A restatement of a known theorem in different notation? A trivial corollary? The boundary is fuzzy.

## Connections

- **math-formal-proofs.md**: The natural pipeline is conjecture -> formalize -> prove. Conjecturing feeds the proof-generation pipeline.
- **number-theory-computation.md**: Many conjectures are about number-theoretic objects. Computation provides the empirical basis.
- **math-symbolic.md**: CAS verification of identities is a key subroutine.
- **abstract-algebra.md**: Conjectures about algebraic structures (group orders, ring properties) use CAS verification.
