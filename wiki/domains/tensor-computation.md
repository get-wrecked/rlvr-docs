---
domain: Tensor Computation & Einsum
category: math-computation
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Tensor Computation & Einsum

## Overview
Compute tensor operations: contractions, decompositions, einsum expressions, Kronecker products, tensor networks. Important for ML (attention mechanisms, convolutions) and physics (general relativity, quantum mechanics). Verification: numerical computation.

## Verification Mechanism
```python
import numpy as np

def verify(task_type: str, input_tensors: dict, answer: Any) -> float:
    if task_type == "einsum":
        # Verify einsum expression produces correct result
        expression = answer["expression"]
        expected = answer["result"]
        computed = np.einsum(expression, *[input_tensors[k] for k in answer["operands"]])
        return 1.0 if np.allclose(computed, expected, rtol=1e-5) else 0.0
    
    elif task_type == "decomposition":
        # Verify tensor decomposition reconstructs original
        reconstructed = reconstruct_from_decomposition(answer)
        original = input_tensors["target"]
        error = np.linalg.norm(reconstructed - original) / np.linalg.norm(original)
        return max(0, 1 - error / answer.get("target_error", 0.01))
    
    elif task_type == "contraction_order":
        # Verify contraction order gives correct result and is efficient
        result = execute_contraction(input_tensors, answer["order"])
        expected = input_tensors["expected_result"]
        if not np.allclose(result, expected, rtol=1e-5):
            return 0.0
        # Bonus for FLOP-optimal ordering
        flops = count_flops(input_tensors, answer["order"])
        optimal_flops = optimal_contraction_flops(input_tensors)
        return min(1.0, optimal_flops / flops)
    
    elif task_type == "shape_inference":
        # Given einsum string, predict output shape
        expression = input_tensors["expression"]
        input_shapes = input_tensors["shapes"]
        correct_shape = compute_einsum_shape(expression, input_shapes)
        return 1.0 if tuple(answer) == correct_shape else 0.0
```

## Dataset Sources
- **ML framework tutorials**: PyTorch, JAX einsum examples.
- **Tensor network benchmarks**: Quantum computing tensor network problems.
- **Opt_einsum benchmarks**: Optimal contraction ordering benchmarks.
- **Procedural generation**: Generate random tensor shapes + operations.
- **Physics computation**: GR tensor computations with known results.
- **ML architecture analysis**: Compute attention patterns, convolution shapes.

## Task Format
- **Input**: "Compute the matrix multiplication of A (3x4) and B (4x5) using einsum notation"
- **Output**: "np.einsum('ij,jk->ik', A, B)" + result

## Difficulty Curriculum
- Level 1: Matrix multiplication, trace, transpose
- Level 3: Batch operations, outer products
- Level 5: Multi-tensor contractions, attention mechanisms
- Level 7: Optimal contraction ordering for tensor networks
- Level 9: Tensor decomposition (Tucker, CP, Tensor Train)

## Limitations & Risks
- Numerical precision matters for large tensors. Use appropriate tolerances.
- Contraction ordering optimization is NP-hard. Accept near-optimal orderings.

## Connections
- [[linear-algebra-computation]] — special case (2D tensors)
- [[math-numerical]] — numerical computation
- [[physics-simulation]] — tensor physics
- [[quantum-computing]] — quantum tensor networks
