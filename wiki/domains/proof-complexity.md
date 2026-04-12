---
domain: Computational Complexity & Reductions
category: theoretical-cs
verification_type: execution
dataset_scale: 10K+ (from CS theory courses)
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Computational Complexity & Reductions

## Overview
Prove complexity-theoretic properties: construct polynomial-time reductions between problems, verify NP-completeness proofs, determine the complexity class of a problem, build efficient algorithms for problems with known polynomial solutions. Verification: check that reductions are correct (polynomial time, correctness of mapping).

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: dict) -> float:
    if task_type == "reduction":
        # Verify polynomial reduction from problem A to problem B
        reduction_fn = answer["reduction_function"]
        
        # Check correctness: yes-instances map to yes-instances
        for yes_instance in problem["A_yes_instances"]:
            reduced = apply_reduction(reduction_fn, yes_instance)
            if not solve_B(reduced):
                return 0.0  # Reduction fails on yes-instance
        
        # Check: no-instances map to no-instances
        for no_instance in problem["A_no_instances"]:
            reduced = apply_reduction(reduction_fn, no_instance)
            if solve_B(reduced):
                return 0.0  # Reduction fails on no-instance
        
        # Check polynomial time
        for instance in problem["A_yes_instances"] + problem["A_no_instances"]:
            time = measure_reduction_time(reduction_fn, instance)
            input_size = len(instance)
            if time > input_size ** answer.get("poly_degree", 3) * 1000:
                return 0.0  # Not polynomial
        
        return 1.0
    
    elif task_type == "algorithm_complexity":
        # Verify that proposed algorithm is correct AND runs in claimed time
        algorithm = answer["algorithm"]
        
        # Correctness
        for test in problem["test_cases"]:
            output = execute_with_timing(algorithm, test["input"])
            if output["result"] != test["expected"]:
                return 0.0
        
        # Time complexity verification (empirical)
        times = []
        for size in [100, 200, 400, 800, 1600]:
            input_data = generate_random_input(problem["problem_type"], size)
            result = execute_with_timing(algorithm, input_data)
            times.append((size, result["time"]))
        
        # Check growth rate matches claimed complexity
        actual_growth = estimate_growth_rate(times)
        claimed_growth = parse_big_o(answer["complexity"])
        return 1.0 if actual_growth <= claimed_growth * 1.5 else 0.5
```

## Dataset Sources
- **CLRS exercises**: Algorithm design and complexity problems.
- **Karp's 21 NP-complete problems**: With reductions.
- **Garey & Johnson**: NP-completeness compendium.
- **Computational complexity textbooks**: Arora & Barak, Sipser.
- **Algorithm competitions**: Problems with known optimal complexity.
- **Procedural generation**: Generate problems from known NP-complete problems via reduction.

## Task Format
- **Input**: "Show that the Vertex Cover problem is NP-complete by reducing from 3-SAT"
- **Output**: Reduction function + proof of correctness

## Difficulty Curriculum
- Level 3: Simple reductions between classic problems
- Level 5: Novel reductions requiring gadgets
- Level 7: Approximation hardness, PCP-related reductions
- Level 9: Novel complexity results

## Limitations & Risks
- Testing reductions on instances is a good heuristic but doesn't prove correctness for all inputs.
- Formal reduction proofs would need Lean4/Coq for full verification.
- Empirical complexity testing is approximate.

## Connections
- [[logic-propositional]] — SAT as the canonical NP-complete problem
- [[graph-theory]] — many NP-complete problems are graph problems
- [[combinatorics-optimization]] — optimization versions of hard problems
