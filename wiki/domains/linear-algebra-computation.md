---
domain: Linear Algebra Computation
category: math-computation
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Linear Algebra Computation

## Overview
Solve linear algebra problems: matrix operations, eigenvalues/eigenvectors, SVD, linear systems, determinants, rank, null space, matrix decompositions. Verification: compute the result numerically and compare.

## Verification Mechanism
```python
import numpy as np

def verify(task_type: str, matrix: np.array, answer: Any) -> float:
    if task_type == "determinant":
        correct = np.linalg.det(matrix)
        return 1.0 if np.isclose(answer, correct, rtol=1e-6) else 0.0
    
    elif task_type == "eigenvalues":
        correct = sorted(np.linalg.eigvals(matrix))
        answer_sorted = sorted(answer)
        return 1.0 if np.allclose(answer_sorted, correct, rtol=1e-4) else 0.0
    
    elif task_type == "inverse":
        # Check A * A_inv = I
        product = matrix @ np.array(answer)
        return 1.0 if np.allclose(product, np.eye(matrix.shape[0]), atol=1e-6) else 0.0
    
    elif task_type == "solve_system":
        # Check Ax = b
        A, b = matrix
        x = np.array(answer)
        return 1.0 if np.allclose(A @ x, b, atol=1e-6) else 0.0
    
    elif task_type == "rank":
        correct = np.linalg.matrix_rank(matrix)
        return 1.0 if answer == correct else 0.0
    
    elif task_type == "svd":
        U, S, Vt = answer
        reconstructed = U @ np.diag(S) @ Vt
        return 1.0 if np.allclose(reconstructed, matrix, atol=1e-6) else 0.0
    
    elif task_type == "null_space":
        # Check all vectors in null space satisfy Ax = 0
        for v in answer:
            if not np.allclose(matrix @ np.array(v), 0, atol=1e-6):
                return 0.0
        # Check dimensionality
        expected_dim = matrix.shape[1] - np.linalg.matrix_rank(matrix)
        if len(answer) != expected_dim:
            return 0.5  # Right space, wrong number of basis vectors
        return 1.0
```

## Dataset Sources
- **Procedural generation**: Generate random matrices of controlled size and properties. Unlimited.
- **Linear algebra textbooks**: Strang, Axler — problem sets with solutions.
- **MIT OCW 18.06**: Gilbert Strang's famous course with problem sets.
- **LAPACK test matrices**: Test matrices with known properties.
- **SuiteSparse Matrix Collection**: 2900+ sparse matrices from real applications.

## Task Format
- **Input**: "Find the eigenvalues of the matrix [[2, 1], [1, 2]]"
- **Output**: [3, 1]

## Difficulty Curriculum
- Level 1: 2x2 matrices, basic operations
- Level 3: 3x3-5x5, eigenvalue problems, SVD
- Level 5: Symbolic linear algebra (parametric matrices)
- Level 7: Large sparse systems, iterative methods
- Level 9: Theoretical proofs about linear operators

## Limitations & Risks
- Numerical precision matters. Use appropriate tolerances for floating-point comparison.
- Some problems have multiple valid answers (e.g., eigenvector direction). Verify properties rather than exact values.
- For very large matrices, computation is straightforward but the agent needs to output the solution format efficiently.

## Connections
- [[math-numerical]] — numerical computation
- [[abstract-algebra]] — linear algebra over abstract fields
- [[data-science-eda]] — PCA and dimensionality reduction use SVD
