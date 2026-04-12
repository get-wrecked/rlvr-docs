---
domain: Distributed Systems Design & Verification
category: systems
verification_type: simulation
dataset_scale: 10K+ (from DS courses + model checking)
difficulty_range: hard/superhuman
modality: text
status: needs_validation
---

# Distributed Systems Design & Verification

## Overview
Design distributed protocols/algorithms (consensus, leader election, distributed locking, replication) and verify their correctness properties (safety, liveness, fault tolerance). Verification: model check the protocol using TLA+/Alloy, or test it in a fault-injection simulation.

## Verification Mechanism
```python
def verify(spec_type: str, protocol: str, properties: list[str]) -> float:
    if spec_type == "tla_plus":
        # Model-check the TLA+ specification
        result = run_tlc(protocol, properties, timeout=300)
        if result.all_properties_hold:
            return 1.0
        return result.properties_passed / result.total_properties
    
    elif spec_type == "code":
        # Fault-injection testing
        passed = 0
        for scenario in fault_scenarios:
            # Run protocol with injected faults
            result = run_simulation(
                protocol, 
                nodes=scenario["nodes"],
                faults=scenario["faults"],  # node crashes, network partitions, delays
                timeout=60
            )
            # Check invariants
            if all(check_property(result, prop) for prop in properties):
                passed += 1
        return passed / len(fault_scenarios)
```

## Dataset Sources
- **TLA+ examples**: Lamport's examples (Paxos, Raft, etc.).
- **DistAlgo**: Distributed algorithm specification language with examples.
- **Jepsen test results**: Known distributed system failures and fixes.
- **DSLDI workshop papers**: Distributed systems papers with formal specs.
- **MIT 6.824**: Distributed systems course labs (MapReduce, Raft, etc.).
- **etcd/CockroachDB/FoundationDB TLA+ specs**: Real-world protocol specifications.

## Task Format
- **Input**: "Design a consensus protocol for 5 nodes that tolerates 2 failures. Specify in TLA+."
- **Output**: TLA+ specification

Or:
- **Input**: "Implement the Raft consensus algorithm in Go." + test harness
- **Output**: Go code that passes the test harness under fault injection

## Difficulty Curriculum
- Level 3: Leader election in a ring
- Level 5: Two-phase commit, distributed locking
- Level 7: Paxos/Raft implementation
- Level 9: Byzantine fault tolerance, novel consensus mechanisms

## Limitations & Risks
- Model checking has state space explosion — limit to small models.
- Fault-injection testing is probabilistic, not exhaustive.
- TLA+ model checking is the strongest verification but requires formal specification skills.
- This is a high-value domain — correct distributed protocols are critical infrastructure.

## Connections
- [[logic-formal-specification]] — TLA+ specifications
- [[code-generation]] — protocol implementation
- [[planning-classical]] — protocol design as planning
- [[multi-agent-coordination]] — multi-node coordination
