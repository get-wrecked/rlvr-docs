---
domain: Code Parallelization
category: Code & Software
verification_type: execution
dataset_scale: ~1K+ from PolyBench + Rodinia + NAS benchmarks
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Code Parallelization

## Overview
Transform sequential code into parallel code (OpenMP, CUDA, MPI, async). Triple verification: functional equivalence, measured speedup, and race-freedom.

## Verification Mechanism
1. Parallel version produces same output as sequential (functional equivalence)
2. Parallel version is faster (speedup > 1.0 on multi-core/GPU)
3. No data races (ThreadSanitizer clean)
4. Statistical benchmarking (multiple runs, confidence intervals)

## Dataset Sources
- PolyBench/C (parallelization benchmarks)
- Rodinia benchmark suite
- NAS Parallel Benchmarks
- PARSEC benchmark
- OpenMP examples, CUDA samples

## Task Format
**Input**: Sequential code + target parallelism model (OpenMP/CUDA/MPI)
**Output**: Parallel version

## Difficulty Curriculum
1. Embarrassingly parallel loops (OpenMP `#pragma omp parallel for`)
2. Reduction operations
3. GPU kernel writing (CUDA/OpenCL)
4. Pipeline parallelism
5. Distributed computation (MPI)
6. Mixed CPU+GPU heterogeneous parallelism

## Connections
[[code-optimization]], [[concurrency-puzzles]], [[memory-management]]
