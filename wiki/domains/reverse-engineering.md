---
domain: Reverse Engineering (Binary Analysis)
category: security
verification_type: exact_match
dataset_scale: 50K+ (from CTF archives, crackmes)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Reverse Engineering (Binary Analysis)

## Overview
Given a compiled binary (or bytecode), determine its behavior, extract hidden data, decompile it, or identify its algorithm. Verification: the extracted information matches the known ground truth (original source, hidden flag, algorithm name).

## Verification Mechanism
```python
def verify(task_type: str, binary: bytes, answer: str, ground_truth: str) -> float:
    if task_type == "extract_flag":
        return 1.0 if answer.strip() == ground_truth.strip() else 0.0
    
    elif task_type == "identify_algorithm":
        return 1.0 if answer.lower() == ground_truth.lower() else 0.0
    
    elif task_type == "decompile":
        # Check if decompiled code is functionally equivalent
        orig_binary = compile(ground_truth)  # compile original source
        for test in test_inputs:
            orig_out = run(orig_binary, test)
            decomp_out = run(compile(answer), test)
            if orig_out != decomp_out:
                return 0.0
        return 1.0
    
    elif task_type == "find_vulnerability":
        # Check if identified vulnerability matches known CVE/bug
        return 1.0 if vulnerability_matches(answer, ground_truth) else 0.0
```

## Dataset Sources
- **CrackMe archives**: crackmes.one — thousands of reverse engineering challenges.
- **CTF reversing challenges**: From CTFtime.org archives.
- **AnghaBench**: 1M compilable C functions — compile, then reverse.
- **Decompilation benchmarks**: ExeBench, various decompiler evaluation sets.
- **Malware analysis datasets**: VirusTotal academic dataset, SOREL-20M.
- **Procedural generation**: Compile random C/C++ programs, strip symbols, create RE tasks.

## Task Format
- **Input**: Binary file (or disassembly) + "What does this program do?" or "Find the password"
- **Output**: Algorithm description, extracted data, or decompiled source

## Difficulty Curriculum
- Level 1: Simple unstripped binaries, identify basic operations
- Level 3: Stripped binaries, simple control flow analysis
- Level 5: Obfuscated code, anti-debugging techniques
- Level 7: Complex algorithms (cryptographic implementations)
- Level 9: Packed/encrypted binaries, custom VMs

## Limitations & Risks
- Binary analysis often requires tool use (disassemblers, debuggers) — this is inherently an agentic task.
- Functional equivalence testing for decompilation is strong but not complete (same output on test inputs ≠ identical program).
- Flag extraction (CTF-style) has perfect verification.

## Connections
- [[cybersecurity-ctf]] — RE is a major CTF category
- [[compiler-tasks]] — reverse of compilation
- [[code-repair]] — understanding code to fix it
