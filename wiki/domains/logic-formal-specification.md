---
domain: Formal Specification
category: Logic / Formal Verification
verification_type: execution
dataset_scale: ~5K existing specs; unlimited via procedural generation
difficulty_range: medium/hard/superhuman
modality: code
status: needs_validation
---

## Overview

Formal specification involves writing precise, machine-checkable descriptions of systems using languages like TLA+ (Temporal Logic of Actions), Alloy, Event-B, or Z. The model checker (TLC for TLA+, Alloy Analyzer for Alloy) automatically verifies whether the specification satisfies desired properties — invariants, liveness, safety, absence of deadlocks. This is a clean RLVR domain: write a spec, run the model checker, get a binary result.

This is a novel RLVR domain with no published training results. However, the verification infrastructure is mature (TLA+ has been used at AWS, Microsoft, etc.), and the task is of high practical value — formal specifications prevent real bugs. The challenge is dataset scarcity: formal specs are rare compared to general code.

## Verification Mechanism

**Primary method: Model checker execution.**

```python
import subprocess
import tempfile

def verify_tlaplus_spec(spec: str, properties: list[str], timeout: int = 120) -> float:
    """
    Verify a TLA+ specification using the TLC model checker.
    
    Args:
        spec: The TLA+ module source code
        properties: List of property names to check (invariants, temporal properties)
        timeout: Max seconds for model checking
    
    Returns:
        1.0 if all properties verified, 0.0 if any violated or error
    """
    with tempfile.TemporaryDirectory() as tmpdir:
        # Write the spec
        spec_path = f"{tmpdir}/Spec.tla"
        with open(spec_path, 'w') as f:
            f.write(spec)
        
        # Write the TLC config
        cfg_path = f"{tmpdir}/Spec.cfg"
        with open(cfg_path, 'w') as f:
            f.write("SPECIFICATION Spec\n")
            for prop in properties:
                if prop.startswith("Inv_"):
                    f.write(f"INVARIANT {prop}\n")
                else:
                    f.write(f"PROPERTY {prop}\n")
        
        # Run TLC
        result = subprocess.run(
            ['java', '-jar', 'tla2tools.jar', '-config', cfg_path, spec_path,
             '-workers', 'auto', '-deadlock'],
            capture_output=True, text=True, timeout=timeout,
            cwd=tmpdir
        )
        
        # Parse TLC output
        if 'Model checking completed. No error has been found.' in result.stdout:
            return 1.0
        elif 'Error:' in result.stdout or 'Invariant' in result.stdout:
            return 0.0  # Property violated
        elif 'Deadlock reached' in result.stdout:
            return 0.0
        else:
            return 0.0  # Parse error or other issue


def verify_alloy_spec(spec: str, assertions: list[str], timeout: int = 60) -> float:
    """
    Verify an Alloy specification using the Alloy Analyzer.
    
    Args:
        spec: The Alloy model source code
        assertions: Assertion names to check
        timeout: Max seconds
    
    Returns:
        1.0 if no counterexamples found for any assertion, 0.0 otherwise
    """
    with tempfile.NamedTemporaryFile(suffix='.als', mode='w', delete=False) as f:
        f.write(spec)
        f.flush()
        
        # Run Alloy CLI (alloy-cli or similar)
        for assertion in assertions:
            result = subprocess.run(
                ['java', '-jar', 'alloy.jar', '--check', assertion, f.name],
                capture_output=True, text=True, timeout=timeout
            )
            if 'Counterexample found' in result.stdout:
                return 0.0  # Assertion violated
            if 'No counterexample found' not in result.stdout:
                return 0.0  # Error or unexpected output
    
    return 1.0


def verify_spec_task(task_type: str, model_output: str, requirements: dict) -> float:
    """
    High-level verification dispatcher for formal spec tasks.
    
    task_type options:
    - "write_spec": Model writes a spec that satisfies given properties
    - "find_bug": Model identifies a bug in a given spec (counterexample)
    - "fix_spec": Model fixes a buggy spec so properties hold
    - "add_property": Model adds a new invariant that holds
    """
    if task_type == "write_spec":
        spec = extract_spec(model_output)
        return verify_tlaplus_spec(spec, requirements["properties"])
    
    elif task_type == "find_bug":
        counterexample = extract_counterexample(model_output)
        return verify_counterexample(requirements["spec"], counterexample)
    
    elif task_type == "fix_spec":
        fixed_spec = extract_spec(model_output)
        # Must satisfy ALL original properties
        score = verify_tlaplus_spec(fixed_spec, requirements["properties"])
        # And must be syntactically different from the buggy original
        if score == 1.0 and fixed_spec.strip() == requirements["buggy_spec"].strip():
            return 0.0  # Didn't actually fix anything
        return score
    
    elif task_type == "add_property":
        new_property = extract_property(model_output)
        # Property must hold on the spec
        spec_with_property = inject_property(requirements["spec"], new_property)
        result = verify_tlaplus_spec(spec_with_property, [new_property])
        if result == 0.0:
            return 0.0  # Property doesn't hold
        # Property must be non-trivial (not just TRUE or existing property)
        if is_trivial_property(new_property, requirements["spec"]):
            return 0.0
        return 1.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **TLA+ Examples** | ~300 | [github.com/tlaplus/Examples](https://github.com/tlaplus/Examples) | Official TLA+ examples |
| **TLA+ Community Modules** | ~100 | [github.com/tlaplus/CommunityModules](https://github.com/tlaplus/CommunityModules) | Reusable TLA+ modules |
| **AWS TLA+ specs** | ~10 | [github.com/awslabs/tla-specifications](https://github.com/awslabs) | DynamoDB, S3, etc. |
| **Alloy models** | ~500 | [alloytools.org/models](https://alloytools.org/) | Various Alloy examples |
| **Event-B models** | ~200 | [deploy-eprints.ecs.soton.ac.uk](https://deploy-eprints.ecs.soton.ac.uk/) | From the DEPLOY project |
| **GitHub TLA+ repos** | ~2K specs | GitHub search `extension:tla` | Community TLA+ specifications |
| **GitHub Alloy repos** | ~1K specs | GitHub search `extension:als` | Community Alloy models |
| **Specifying Systems (Lamport)** | ~50 specs | Lamport's book examples | Textbook quality |
| **Hillel Wayne's examples** | ~100 | [learntla.com](https://learntla.com/) | Tutorial examples |
| **Formal Methods in Practice (various)** | ~500 | Various papers | Industrial case studies |

### Synthetic Task Generation

```python
def generate_mutual_exclusion_task(n_processes=2, bug_type=None):
    """
    Generate a mutual exclusion specification task.
    
    The correct spec must ensure:
    - Safety: no two processes in critical section simultaneously
    - Liveness: every requesting process eventually enters critical section
    - No deadlock
    """
    if bug_type is None:
        # Task: write a correct spec from scratch
        return {
            "task": "write_spec",
            "description": f"Write a TLA+ specification for mutual exclusion with {n_processes} processes.",
            "properties": ["MutualExclusion", "NoDeadlock", "Liveness"],
            "hints": f"Use {n_processes} processes with states: idle, waiting, critical."
        }
    else:
        # Task: find/fix a bug in a given spec
        buggy_spec = inject_bug(CORRECT_MUTEX_SPEC, bug_type)
        return {
            "task": "fix_spec",
            "buggy_spec": buggy_spec,
            "properties": ["MutualExclusion", "NoDeadlock"],
            "bug_type": bug_type  # e.g., "missing_guard", "wrong_transition"
        }

def generate_distributed_system_task(system_type="leader_election"):
    """Generate tasks based on classic distributed system specifications."""
    templates = {
        "leader_election": {
            "description": "Specify a leader election protocol for a ring of N nodes.",
            "properties": ["ExactlyOneLeader", "Termination"]
        },
        "two_phase_commit": {
            "description": "Specify the two-phase commit protocol.",
            "properties": ["ConsistentDecision", "NoBlockingAfterRecovery"]
        },
        "raft_consensus": {
            "description": "Specify the Raft consensus protocol (simplified).",
            "properties": ["ElectionSafety", "LogMatching", "LeaderCompleteness"]
        }
    }
    return templates[system_type]
```

## Task Format

**Input**: System description + desired properties.

```
Task: Write a TLA+ specification for a simple producer-consumer system with a 
bounded buffer of size 3. 

The specification must satisfy:
1. INVARIANT BufferBounded: The buffer never exceeds 3 items.
2. INVARIANT TypeOK: All variables have the correct types.
3. PROPERTY Liveness: If the producer wants to produce, it eventually does.

Provide the complete TLA+ module.
```

**Output**: Complete TLA+ module.

```tla
---- MODULE ProducerConsumer ----
EXTENDS Integers, Sequences

CONSTANT MaxBufSize
ASSUME MaxBufSize = 3

VARIABLES buffer, waitingProd, waitingCons

vars == <<buffer, waitingProd, waitingCons>>

Init == /\ buffer = <<>>
        /\ waitingProd = FALSE
        /\ waitingCons = FALSE

Produce == /\ Len(buffer) < MaxBufSize
           /\ buffer' = Append(buffer, "item")
           /\ UNCHANGED <<waitingProd, waitingCons>>

Consume == /\ Len(buffer) > 0
           /\ buffer' = Tail(buffer)
           /\ UNCHANGED <<waitingProd, waitingCons>>

Next == Produce \/ Consume

Spec == Init /\ [][Next]_vars /\ WF_vars(Produce) /\ WF_vars(Consume)

BufferBounded == Len(buffer) <= MaxBufSize
TypeOK == /\ buffer \in Seq({"item"})
          /\ Len(buffer) <= MaxBufSize
====
```

## Difficulty Curriculum

| Level | Task | Spec Size | Model Check Time | Scale |
|-------|------|-----------|-----------------|-------|
| Easy | Simple state machines (traffic light, counter) | 20-40 lines | <1s | Unlimited |
| Medium | Mutual exclusion, producer-consumer | 40-80 lines | 1-10s | ~500 |
| Hard | Distributed protocols (2PC, Paxos simplified) | 80-200 lines | 10s-5min | ~200 |
| Very Hard | Full Raft/Paxos, complex industrial specs | 200+ lines | Minutes-hours | ~50 |
| Superhuman | Novel protocol design with novel properties | Unbounded | May not terminate | Open |

## Limitations & Risks

1. **Dataset scarcity**: Formal specifications are rare compared to general code. There may only be a few thousand TLA+ specs in existence. This limits the pretraining signal and RL data.
2. **State space explosion**: Model checking may not terminate in reasonable time for complex specs. The verifier has a time limit, but correct specs might time out, giving false negatives.
3. **Syntactic fragility**: TLA+ syntax is unusual and error-prone. Models may produce syntactically invalid specs. A syntax-check pre-filter is essential.
4. **Partial credit is hard**: A spec that satisfies 2 of 3 properties gets 0 reward (all-or-nothing). Graduated reward based on number of properties satisfied could help, but the model checker typically reports only the first violation.
5. **Limited practical impact without natural language understanding**: The real value of LLMs for formal specs is translating *informal* requirements into *formal* specifications. This requires pairing formal verification with natural language understanding, which is harder to evaluate automatically.
6. **No published RLVR results**: This is an entirely novel RLVR domain. Expected to work based on analogy with code generation + test execution, but unproven.

## Connections

- **math-formal-proofs.md**: Lean4/Coq type-checking is analogous to model checking — both are "write code, tool checks it" paradigms.
- **logic-first-order.md**: TLA+ is based on temporal logic, which extends FOL with temporal operators.
- **logic-propositional.md**: Bounded model checking reduces to SAT.
- **automated-reasoning.md**: Formal specification is a key application of automated reasoning.
