---
domain: Analog Circuit Design
category: hardware-engineering
verification_type: simulation
dataset_scale: 10K+ (from circuit benchmarks)
difficulty_range: medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Analog Circuit Design

## Overview
Design analog circuits (amplifiers, filters, oscillators, ADCs/DACs, PLLs) meeting specified performance requirements. Verification: SPICE simulation confirms specs (gain, bandwidth, noise, linearity, power consumption).

## Verification Mechanism
```python
import PySpice

def verify(specs: dict, netlist: str) -> float:
    # Parse and simulate the circuit
    try:
        circuit = parse_spice_netlist(netlist)
        simulator = circuit.simulator()
    except:
        return 0.0
    
    score = 0.0
    checks = 0
    
    # DC operating point
    analysis = simulator.operating_point()
    
    # AC analysis (gain and bandwidth)
    if "gain_db" in specs or "bandwidth" in specs:
        ac = simulator.ac(start_frequency=1, stop_frequency=1e9, 
                         number_of_points=1000, variation='dec')
        gain_curve = 20 * np.log10(np.abs(ac.output))
        
        if "gain_db" in specs:
            checks += 1
            midband_gain = np.max(gain_curve)
            if abs(midband_gain - specs["gain_db"]) < 3:  # within 3dB
                score += 1
        
        if "bandwidth" in specs:
            checks += 1
            bw = compute_3db_bandwidth(ac)
            if bw >= specs["bandwidth"] * 0.8:
                score += 1
    
    # Power consumption
    if "max_power_mw" in specs:
        checks += 1
        power = compute_power(analysis)
        if power <= specs["max_power_mw"]:
            score += 1
    
    # Transient analysis
    if "settling_time" in specs:
        checks += 1
        tran = simulator.transient(step_time=1e-9, end_time=1e-3)
        settling = compute_settling_time(tran.output, specs["final_value"])
        if settling <= specs["settling_time"]:
            score += 1
    
    return score / checks if checks > 0 else 0.0
```

## Dataset Sources
- **ISCAS benchmark circuits**: Standard circuit benchmarks.
- **Analog circuit design textbooks**: Razavi, Gray & Meyer — example circuits.
- **SPICE model libraries**: Component models.
- **Analog design automation benchmarks**: MAGICAL, OpenDB.
- **Electronics StackExchange**: Circuit design Q&A.

## Task Format
- **Input**: "Design a non-inverting op-amp circuit with gain = 10, bandwidth > 100kHz, using LM741 op-amp and standard resistor values"
- **Output**: SPICE netlist

## Difficulty Curriculum
- Level 1: Resistor dividers, RC filters
- Level 3: Op-amp circuits, transistor biasing
- Level 5: Multi-stage amplifiers, active filters
- Level 7: PLLs, oscillators, DAC/ADC
- Level 9: Full mixed-signal system design

## Limitations & Risks
- SPICE simulation is computationally expensive for complex circuits.
- SPICE models must be accurate for meaningful verification.
- Real analog design has component tolerances, temperature effects — simulation is idealized.

## Connections
- [[circuit-design]] — general circuit design
- [[chip-design-rtl]] — digital counterpart
- [[signal-processing]] — filter design
- [[electrical-engineering]] — EE fundamentals
