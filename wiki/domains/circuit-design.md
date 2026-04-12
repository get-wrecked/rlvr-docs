---
domain: Circuit Design
category: Engineering
verification_type: SPICE simulation confirms electrical specifications met
dataset_scale: Small-to-Medium (1K-20K; high-quality benchmarks are scarce)
difficulty_range: Simple resistor networks to mixed-signal IC design
modality: Text-in, structured-out (netlists, component values)
status: Promising but constrained — SPICE verification is solid, dataset availability is the bottleneck
---

## Overview

Circuit design tasks ask the model to design or analyze digital and analog circuits: choose component values, determine circuit topology, predict output waveforms, or meet electrical specifications (gain, bandwidth, power, noise). The RLVR opportunity is that circuits can be definitively verified by simulation — SPICE (Simulation Program with Integrated Circuit Emphasis) has been the gold standard for circuit simulation since the 1970s and produces highly reliable results for well-defined circuits.

This domain covers resistor networks (voltage dividers, Wheatstone bridges), amplifier design (common-emitter, op-amp configurations), filter circuits (RC, RLC, active filters), digital logic (gate-level design, timing analysis), and power electronics (converter topologies, regulation).

## Verification Mechanism

**Primary approach:** Convert the model's circuit design to a SPICE netlist, simulate, and check whether output characteristics meet the specified requirements.

- **DC operating point verification:** Simulate the circuit at DC and check node voltages, branch currents against specifications.
- **AC analysis verification:** Run AC sweep and verify gain, bandwidth, phase margin, etc. match requirements within tolerance.
- **Transient analysis verification:** Simulate time-domain response and check rise time, overshoot, settling time, output waveform.
- **Digital logic verification:** For combinational logic, exhaustively check truth table. For sequential logic, run testbench stimulus and compare outputs to expected sequence.
- **Spec compliance check:** The design problem specifies requirements (e.g., "voltage gain > 20 dB, bandwidth > 1 MHz, power < 10 mW"). Simulation extracts these metrics; verification checks all specs are met simultaneously.

**Verification reliability: VERY HIGH for DC and digital analysis.** Kirchhoff's laws are exact. A resistor network simulation is deterministic to floating-point precision.

**Verification reliability: HIGH for small-signal AC and transient analysis.** SPICE simulation of linear and mildly nonlinear circuits is well-validated. Convergence issues are rare for well-designed circuits.

**Verification reliability: MODERATE for large or complex circuits.** Simulation convergence can fail for poorly specified circuits. The model might produce a valid design that SPICE cannot simulate due to numerical issues — this creates a false-negative risk.

## Dataset Sources

- **ISCAS benchmark circuits:** Standard benchmark suite for digital circuits (ISCAS-85 combinational, ISCAS-89 sequential). Well-studied but focused on testing/verification rather than design.
- **Textbook problems:** Sedra/Smith, Razavi, Horowitz/Hill provide thousands of circuit analysis and design problems with known solutions.
- **Open-source circuit libraries:** CircuitVerse, OpenCircuits provide community-designed circuits.
- **Synthetic generation:** Parameterize circuit templates (e.g., "Design an inverting amplifier with gain = -X using an op-amp with R_f and R_in"). Vary parameters to generate instances.
- **IEEE benchmark circuits:** Industry benchmarks for specific applications (ADCs, PLLs, power converters).

**Realistic scale:** 5K-20K problems with SPICE-verified solutions. Synthetic generation from parameterized templates is the most scalable approach. The bottleneck is the diversity of circuit topologies, not the number of parameter variations.

## Task Format

**Input:** Circuit design specification in natural language.

Example 1 (analysis):
```
A voltage divider consists of R1 = 10 kOhm and R2 = 5 kOhm connected
in series across a 12V supply. What is the voltage across R2?
```

Example 2 (design):
```
Design a non-inverting op-amp amplifier with a voltage gain of exactly 11.
Specify the feedback resistor Rf and input resistor R1 values.
Assume an ideal op-amp.
```

Example 3 (spec compliance):
```
Design a second-order active low-pass Butterworth filter with a cutoff
frequency of 1 kHz and a passband gain of 0 dB. Specify all component values.
```

**Output:** Component values, netlist, or numerical analysis results.

## Difficulty Curriculum

1. **Level 1 — Resistor networks:** Series/parallel combinations, voltage dividers, Wheatstone bridges. Exact analytical solutions.
2. **Level 2 — Single-transistor/op-amp circuits:** Common-emitter amplifiers, inverting/non-inverting op-amp configurations. Standard design formulas.
3. **Level 3 — Multi-stage and feedback circuits:** Cascaded amplifiers, feedback networks, active filters. Require understanding of loading effects and frequency response.
4. **Level 4 — Mixed-signal design:** ADC/DAC architectures, PLLs, oscillators. Require co-design of analog and digital sections.
5. **Level 5 — Full IC design:** Multi-transistor analog blocks with constraints on area, power, noise, linearity simultaneously. This approaches real design complexity.

## Limitations & Risks

- **Netlist format fragility:** The model must produce syntactically valid SPICE netlists. Even small formatting errors cause simulation failure. A preprocessing step that validates netlist syntax before simulation is essential.
- **Multiple valid designs:** Most design specifications can be met by many different topologies and component values. Verification must check spec compliance, not match a reference design.
- **SPICE convergence:** Complex or poorly-designed circuits may cause SPICE convergence failures. The verification system must distinguish between "design is bad" and "simulation failed to converge" — the latter is not the model's fault in all cases.
- **Component models:** SPICE simulation depends on device models (transistor models, op-amp models). Different models give different results. The verification must use consistent models and communicate them in the problem statement.
- **Parasitics and real-world effects:** Textbook problems ignore parasitic capacitance, resistance, and inductance. Real circuits behave differently. This is a fundamental gap between simulation and reality, but for RLVR purposes, the simulation is the ground truth.
- **Limited dataset diversity:** The space of useful circuits is vast, but existing benchmarks cover a narrow slice. Heavy investment in dataset curation is needed.

## Connections

- Closely related to **electrical-engineering.md** (which covers broader EE problems including power systems)
- **signal-processing.md** overlaps for filter design tasks
- **control-systems.md** shares feedback analysis concepts
- **quantum-computing.md** provides a quantum analogue (quantum circuit design)
- Verification approach (simulation-based) is shared with **physics-simulation.md** and **fluid-dynamics.md**
