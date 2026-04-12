---
domain: RTL / Chip Design (Verilog/VHDL)
category: hardware-engineering
verification_type: simulation
dataset_scale: 100K+ (from open-source hardware repos)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# RTL / Chip Design (Verilog/VHDL)

## Overview
Generate Register Transfer Level (RTL) hardware descriptions in Verilog or VHDL that implement specified digital circuits. Verification via hardware simulation: the design must pass a testbench (analogous to unit tests for software).

## Verification Mechanism
```python
def verify(spec: str, verilog_code: str, testbench: str) -> float:
    # Compile the design
    compile_result = run_iverilog(verilog_code, testbench)
    if not compile_result.success:
        return 0.0
    
    # Run simulation
    sim_result = run_vvp(compile_result.binary, timeout=60)
    
    # Parse testbench results
    tests_passed = parse_test_results(sim_result.output)
    total_tests = parse_total_tests(sim_result.output)
    
    if total_tests == 0:
        return 0.0
    
    return tests_passed / total_tests
```

Can also verify:
- **Synthesis**: Does the design synthesize (using Yosys)? Indicates it's actually implementable.
- **Formal verification**: Use SymbiYosys for formal property checking.
- **Resource utilization**: Check gate count, critical path delay.

## Dataset Sources
- **VerilogEval**: Benchmark for Verilog generation (analogous to HumanEval).
- **RTLLM**: RTL generation benchmark.
- **HDLBits**: Educational Verilog exercises (170+ problems with test benches). hdlbits.01xz.net
- **OpenCores**: Open-source hardware designs.
- **RISC-V cores**: Open-source processor implementations (PicoRV32, VexRiscv).
- **LibreCores**: Open-source hardware library.
- **Chisel/SpinalHDL examples**: Hardware construction language examples (can be converted to Verilog).
- **University course materials**: Digital design courses with Verilog exercises.

## Task Format
- **Input**: "Implement a 4-bit synchronous counter with active-low reset and carry output"
- **Output**: Verilog module code

## Difficulty Curriculum
- Level 1: Combinational logic (multiplexers, adders, encoders)
- Level 3: Sequential logic (counters, registers, FSMs)
- Level 5: Memory controllers, UART, SPI interfaces
- Level 7: Pipeline processors, cache controllers
- Level 9: Full processor cores, complex SoC components
- Level 10: Novel architectures, hardware ML accelerators

## Limitations & Risks
- Testbench coverage may be incomplete (design passes tests but has bugs). Mitigate with formal verification.
- Simulation is slower than software test execution. Iverilog is fast enough for most designs.
- Synthesis tools (Yosys) provide additional verification but are slower.
- Hardware design has stricter correctness requirements than software — a single bug can be catastrophic (no patching after fabrication).

## Connections
- [[circuit-design]] — analog circuit design
- [[code-generation]] — RTL generation is "code" generation for hardware
- [[compiler-tasks]] — hardware synthesis is like compilation
- [[logic-propositional]] — hardware design is applied Boolean logic
