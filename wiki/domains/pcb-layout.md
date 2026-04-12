---
domain: PCB Layout & Physical EDA
category: hardware-engineering
verification_type: constraint
dataset_scale: 10K+ (from EDA benchmarks)
difficulty_range: medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# PCB Layout & Physical EDA

## Overview
Place and route electronic components on a printed circuit board or integrated circuit, satisfying design rules (clearances, trace widths, layer assignments) while optimizing area, wirelength, and signal integrity. Verification: DRC (Design Rule Check) passes, netlist connectivity verified.

## Verification Mechanism
```python
def verify(netlist: dict, layout: dict, design_rules: dict) -> float:
    score = 0.0
    checks = 0
    
    # DRC: all design rules satisfied
    checks += 1
    drc_violations = run_drc(layout, design_rules)
    if len(drc_violations) == 0:
        score += 1
    
    # Connectivity: all nets connected correctly
    checks += 1
    connectivity_errors = check_connectivity(netlist, layout)
    if len(connectivity_errors) == 0:
        score += 1
    
    # No short circuits
    checks += 1
    shorts = detect_shorts(layout)
    if len(shorts) == 0:
        score += 1
    
    # Optimization metrics
    if "target_area" in design_rules:
        checks += 1
        actual_area = compute_bounding_area(layout)
        if actual_area <= design_rules["target_area"]:
            score += 1
    
    if "target_wirelength" in design_rules:
        checks += 1
        wl = compute_total_wirelength(layout)
        if wl <= design_rules["target_wirelength"] * 1.1:
            score += 1
    
    return score / checks
```

## Dataset Sources
- **ISPD Placement Benchmarks**: Annual EDA placement competition instances.
- **ICCAD CAD Contest**: Routing and placement benchmarks.
- **FreePDK45/FreePDK15**: Open-source process design kits.
- **OpenROAD benchmarks**: Open-source EDA flow benchmarks.
- **KiCad open hardware**: Open-source PCB designs.
- **ISCAS benchmarks**: Standard circuit benchmarks for physical design.

## Task Format
- **Input**: Circuit netlist + board outline + component library + design rules
- **Output**: Component placements (x, y, rotation) + trace routes (layer, path)

## Difficulty Curriculum
- Level 1: Place 5-10 components on single-layer board
- Level 3: Two-layer routing with vias
- Level 5: Multi-layer board with ground planes, differential pairs
- Level 7: High-density interconnect, impedance matching
- Level 9: ASIC placement and routing at advanced nodes

## Limitations & Risks
- DRC is well-defined and automatable — strong verification.
- Signal integrity analysis (EMI, crosstalk) requires simulation on top of DRC.
- Physical design is computationally expensive to verify fully.

## Connections
- [[chip-design-rtl]] — logical design that feeds physical layout
- [[analog-circuit-design]] — circuit that needs layout
- [[combinatorics-optimization]] — placement/routing as optimization
- [[engineering-optimization]] — physical design optimization
