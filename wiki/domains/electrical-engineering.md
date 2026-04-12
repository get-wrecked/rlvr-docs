---
domain: Electrical Engineering
category: Engineering
verification_type: Circuit simulation (SPICE, LTspice), analytical computation, standards lookup
dataset_scale: Medium (10K-50K from textbooks and synthetic generation)
difficulty_range: Ohm's law to power system stability analysis
modality: Text-in, text-out (numerical values, component specifications)
status: Strong — most EE problems have deterministic, computable answers
---

## Overview

Electrical engineering tasks cover the broad spectrum of EE beyond circuit design (which has its own page): circuit analysis (mesh/nodal analysis, Thevenin/Norton equivalents), power systems (load flow, fault analysis, protection coordination), electromagnetics (field calculations, transmission line analysis), semiconductor physics (diode/transistor characteristics), and electrical machines (motor/generator performance). The RLVR advantage is that nearly all textbook EE problems have exact analytical or simulation-verifiable answers.

This domain complements **circuit-design.md** (which focuses on design synthesis) by covering analysis and broader EE topics. The distinction: circuit design asks "build a circuit that does X," while this domain asks "given this circuit/system, compute Y."

## Verification Mechanism

**Primary approach:** Solve the problem using established analytical methods or circuit simulation and compare to the model's answer.

- **Circuit analysis verification:** Set up the circuit in SPICE or solve using mesh/nodal analysis (linear algebra for resistive circuits, Laplace methods for dynamic circuits). Compare voltages, currents, power values.
- **Power systems verification:** Run load flow analysis (Newton-Raphson solver), fault analysis (symmetrical components), or stability analysis (swing equation simulation). Compare to model's answers.
- **Electromagnetics verification:** Compute field quantities from Maxwell's equations (analytical for simple geometries, numerical for complex ones). Compare E, B, S fields and derived quantities.
- **Transmission line verification:** Use transmission line equations (Smith chart calculations, ABCD parameters) to verify impedance, reflection coefficient, VSWR.
- **Semiconductor verification:** Use Shockley diode equation, MOSFET models to compute operating points. Compare to model outputs.

**Verification reliability: VERY HIGH for linear circuit analysis.** Kirchhoff's laws + Ohm's law = system of linear equations with an exact solution.

**Verification reliability: HIGH for power systems load flow and fault analysis.** Algorithms are deterministic and well-validated. Standard IEEE test systems provide benchmarks.

**Verification reliability: HIGH for EM field calculations in simple geometries.** Coulomb's law, Biot-Savart law, and Faraday's law give exact answers for canonical geometries (infinite wire, parallel plates, solenoid).

**Verification reliability: MODERATE for complex EM geometries.** Numerical EM solvers (FDTD, FEM) are approximate and mesh-dependent. But for standard textbook geometries, analytical solutions exist.

## Dataset Sources

- **Textbook problems:** Hayt/Buck (EM), Irwin/Nelms (circuit analysis), Glover/Sarma (power systems), Sedra/Smith (electronics). Thousands of problems per textbook.
- **IEEE test systems:** IEEE 14-bus, 30-bus, 118-bus systems for power flow analysis. Standard benchmarks with known solutions.
- **FE/PE exam problems:** Fundamentals of Engineering and Professional Engineering exam problems provide well-calibrated, medium-difficulty problems.
- **Synthetic generation:** Parameterize circuits (vary R, L, C values), power systems (vary loads, generation), EM geometries (vary dimensions, charges). Highly scalable.
- **OpenStax textbook problems:** Free, openly licensed textbook problems for introductory EE.

**Realistic scale:** 20K-50K problems achievable. Textbooks provide ~5K-10K. Synthetic generation provides unlimited instances of parameterized problems.

## Task Format

**Input:** EE problem with circuit/system description and specific question.

Example 1 (circuit analysis):
```
In the following circuit, find the current through R3 using mesh analysis:
- V1 = 12V (source in mesh 1)
- R1 = 4 Ohm, R2 = 6 Ohm, R3 = 3 Ohm
- R1 and R2 are in mesh 1; R2 and R3 are in mesh 2
[Circuit topology description]
```

Example 2 (power systems):
```
For the IEEE 14-bus system with the following load and generation data,
perform a load flow analysis and report the voltage magnitude and angle
at Bus 5.
```

Example 3 (electromagnetics):
```
Two parallel infinite wires separated by distance d = 0.5 m carry
currents I1 = 10 A and I2 = 5 A in opposite directions. What is the
magnetic field magnitude (in T) at the midpoint between the wires?
```

**Output:** Numerical answers with units.

## Difficulty Curriculum

1. **Level 1 — Single-concept problems:** Ohm's law, series/parallel resistance, capacitor charge/discharge. Direct formula application.
2. **Level 2 — Multi-concept analysis:** Mesh/nodal analysis of 3-5 node circuits, Thevenin equivalents, AC phasor analysis.
3. **Level 3 — Dynamic and frequency analysis:** Transient analysis of RLC circuits, frequency response, resonance. Laplace transform methods.
4. **Level 4 — Power systems and machines:** Load flow, fault analysis, motor/generator performance calculations. Require domain-specific methods.
5. **Level 5 — Advanced topics:** Transmission line matching, electromagnetic wave propagation, power system stability, advanced semiconductor analysis.

## Limitations & Risks

- **Circuit topology communication:** Describing circuit topology in natural language is error-prone. The model and verifier must share an unambiguous representation. Netlists or structured descriptions help.
- **Multiple valid solution methods:** The same circuit can be solved by mesh analysis, nodal analysis, superposition, or Thevenin transformation. Answers should match regardless of method, but intermediate steps differ.
- **Unit consistency:** EE uses many unit systems (SI, CGS for magnetics; per-unit for power systems). The problem must specify units clearly, and the verifier must handle unit conversions.
- **Idealization assumptions:** Textbook problems assume ideal wires (zero resistance), ideal sources, ideal op-amps. The verification must share these assumptions.
- **Power systems scale:** Real power systems have thousands of buses. Textbook problems are much smaller. The gap between textbook-scale and real-world-scale problems is significant.
- **Frequency-domain vs. time-domain:** Some problems have equivalent formulations in frequency and time domains. The answer format must be specified (e.g., peak value vs. RMS, magnitude vs. complex).

## Connections

- **circuit-design.md** is the design counterpart (this page covers analysis)
- **signal-processing.md** shares AC analysis, frequency response, and filter concepts
- **control-systems.md** overlaps for motor control and power electronics regulation
- **physics-simulation.md** provides the electromagnetic theory underlying EE
- **engineering-optimization.md** connects for power system optimal dispatch and economic operation
