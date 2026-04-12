---
domain: Quantum Computing
category: Engineering/Science
verification_type: Quantum circuit simulation (statevector/unitary comparison)
dataset_scale: Small-to-Medium (5K-30K; limited by circuit simulation cost for large circuits)
difficulty_range: Single-qubit gates to multi-qubit algorithm design and optimization
modality: Text-in, structured-out (gate sequences, circuit descriptions)
status: Strong for small circuits — verification is exact; scalability is the constraint
---

## Overview

Quantum computing tasks ask the model to design, analyze, or optimize quantum circuits: construct circuits implementing specific unitary operations, optimize gate counts for known algorithms, predict measurement outcome probabilities, and solve quantum information problems. The RLVR advantage is that quantum circuits can be simulated exactly on classical computers (for small numbers of qubits), giving perfect verification. The catch is that classical simulation of quantum circuits is exponentially expensive in the number of qubits, limiting verification to circuits with ~25-30 qubits at most.

This domain covers quantum gate construction (decompose a unitary into elementary gates), circuit optimization (reduce gate count or depth while preserving functionality), algorithm implementation (implement Grover, QFT, VQE circuits), error correction (design/verify error correcting codes), and quantum state preparation.

## Verification Mechanism

**Primary approach:** Simulate the model's circuit and compare the output state or unitary matrix to the target.

- **Statevector comparison:** Simulate the circuit on the specified input state. Compare the output statevector to the target statevector. Two states are equivalent if they differ by at most a global phase and their fidelity |<psi|phi>|^2 is within tolerance of 1.
- **Unitary comparison:** Compute the unitary matrix implemented by the circuit. Compare to the target unitary (up to global phase). Process fidelity provides a scalar metric.
- **Measurement probability verification:** For circuits with measurement, compute the output probability distribution. Compare to the target distribution using total variation distance or KL divergence.
- **Gate count/depth verification:** Count the number of gates and circuit depth. Verify these meet optimization constraints (e.g., "implement QFT on 4 qubits using at most 12 CNOT gates").
- **Error correction verification:** Verify that the proposed code corrects the specified errors by checking the Knill-Laflamme conditions or by simulating error injection and correction.

**Verification reliability: EXACT for small circuits (up to ~25 qubits).** Statevector simulation gives exact results. Unitary comparison is mathematically rigorous. This is one of the strongest verification mechanisms in this wiki.

**Verification reliability: HIGH for gate count and depth verification.** Counting gates is trivial. Verifying that the circuit still implements the correct operation after optimization requires simulation (exact for small circuits).

**Verification reliability: MODERATE for large circuits (>25 qubits).** Classical simulation becomes intractable. Approximate simulation methods (tensor network, stabilizer simulation for Clifford circuits) work for specific circuit classes but not generally.

## Dataset Sources

- **Qiskit textbook problems:** IBM's Qiskit Textbook provides structured exercises covering quantum gates, algorithms, and applications.
- **Quantum algorithm zoo:** Catalog of known quantum algorithms. Each can be turned into a "implement algorithm X for input size N" problem.
- **Circuit optimization benchmarks:** MQT Bench (Munich Quantum Toolkit), RevLib provide circuit benchmarks for optimization.
- **QASMBench:** Benchmark suite of quantum circuits at various sizes and complexities.
- **Synthetic generation:** Generate random target unitaries (for small qubit counts) and ask the model to decompose them into elementary gates. Highly scalable within the qubit limit.
- **Error correction problems:** Construct problems from known quantum error-correcting codes (Steane code, surface code, Shor code).

**Realistic scale:** 10K-30K problems for circuits up to ~10 qubits (where simulation is fast). ~5K problems for 10-20 qubits (simulation takes seconds to minutes). Beyond 25 qubits, verification becomes impractical for general circuits.

## Task Format

**Input:** Target operation or algorithm specification, gate set constraints, and optimization objectives.

Example 1 (gate synthesis):
```
Construct a quantum circuit using only {H, CNOT, T, T†} gates that
implements the Toffoli gate (CCNOT) on 3 qubits. Minimize the
number of T/T† gates.
```

Example 2 (algorithm implementation):
```
Implement the Quantum Fourier Transform (QFT) on 4 qubits using
H, controlled-phase, and SWAP gates. Give the circuit as a
sequence of gates.
```

Example 3 (state preparation):
```
Design a circuit that prepares the state |psi> = (1/sqrt(3))|00> +
(1/sqrt(3))|01> + (1/sqrt(3))|10> from the initial state |00>,
using only H, CNOT, and Ry gates.
```

Example 4 (optimization):
```
The following circuit implements a 2-qubit operation:
H q[0]; CNOT q[0],q[1]; H q[0]; CNOT q[0],q[1]; H q[0];
Simplify this circuit to use fewer gates while implementing the
same unitary operation.
```

**Output:** Gate sequence or circuit description (QASM format or structured list).

## Difficulty Curriculum

1. **Level 1 — Single-qubit operations:** Express rotations using elementary gates. Verify by 2x2 matrix multiplication.
2. **Level 2 — Two-qubit circuits:** CNOT decomposition, Bell state preparation, simple entanglement circuits. 4x4 unitary verification.
3. **Level 3 — Standard algorithms:** Implement QFT, Grover iteration, quantum teleportation for small qubit counts. Well-known constructions.
4. **Level 4 — Circuit optimization:** Reduce gate count or depth of given circuits while preserving functionality. Requires understanding gate identities and commutation relations.
5. **Level 5 — Novel circuit design:** Design circuits for less standard tasks (variational ansatze, error correction encoding circuits, quantum simulation circuits). More open-ended; verification checks correctness, not optimality.

## Limitations & Risks

- **Exponential simulation cost is the hard ceiling.** Beyond ~25-30 qubits, we cannot verify general circuits. This limits the domain to small-scale quantum computing problems. For Clifford circuits or circuits with special structure, efficient simulation exists for larger sizes.
- **Global phase ambiguity:** Quantum states are equivalent up to global phase. The verification must account for this (compare |<psi|phi>|^2, not raw amplitudes).
- **Gate set specification:** The available gate set must be clearly specified. Different gate sets have different decomposition costs. H+CNOT+T is universal; H+CNOT alone generates only Clifford group.
- **Numerical precision:** For small circuits, statevector simulation is exact. For larger circuits, floating-point accumulation can introduce errors. Tolerance must account for this.
- **Circuit format parsing:** The model must output circuits in a parseable format (QASM, gate lists, etc.). Robust parsing is essential.
- **Practical relevance gap:** Problems solvable with <=25 qubits are not the circuits that will run on future quantum computers. The skills learned on small circuits may not transfer to the 100-1000 qubit regime.
- **Measurement non-determinism:** If the circuit includes mid-circuit measurement and conditional operations, verification must account for the probabilistic nature of measurement (run many shots and compare distributions).

## Connections

- **circuit-design.md** is the classical analogue (classical circuit design vs. quantum circuit design)
- **physics-simulation.md** connects via quantum physics (the underlying theory)
- **engineering-optimization.md** relates for circuit optimization (minimize gate count/depth is an optimization problem)
- **signal-processing.md** connects through the quantum Fourier transform
- The exponential simulation barrier is unique to this domain and fundamentally limits scalability in a way other domains do not face
