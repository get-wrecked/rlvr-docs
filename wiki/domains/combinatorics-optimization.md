---
domain: Combinatorial Optimization
category: Math / Operations Research
verification_type: execution
dataset_scale: ~100K instances (unlimited via procedural generation)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

## Overview

Combinatorial optimization asks for the best solution from a finite (but exponentially large) set: shortest tour (TSP), best bin packing, optimal scheduling, minimum vertex cover, maximum independent set, graph coloring, etc. Verification is clean and unambiguous: compute the objective value of the proposed solution, check feasibility constraints, and optionally compare to known bounds or optimal values.

This domain is well-suited to RLVR because:
1. **Feasibility is easy to check**: Does the tour visit every city exactly once? Do the items fit in the bins?
2. **Quality is easy to measure**: Total distance, number of bins, makespan, etc.
3. **Rich benchmark libraries exist**: TSPLIB, OR-Library, CVRPLIB, etc. have been curated over decades.
4. **Graduated reward is natural**: Instead of binary (correct/incorrect), reward = quality of solution relative to known optimum.

Some RLVR work exists (e.g., RL for combinatorial optimization with pointer networks, attention-based methods), but these are typically specialized architectures, not LLM-based RLVR.

## Verification Mechanism

**Primary method: Compute objective value + check feasibility constraints.**

```python
import math

def verify_tsp(cities: list[tuple], tour: list[int], known_optimal: float = None) -> float:
    """
    Verify a Traveling Salesman Problem solution.
    
    Args:
        cities: List of (x, y) coordinates
        tour: Ordered list of city indices (0-indexed), representing the tour
        known_optimal: Optimal tour length (if known)
    
    Returns:
        Score in [0, 1] based on solution quality
    """
    n = len(cities)
    
    # Check feasibility
    if len(tour) != n:
        return 0.0
    if set(tour) != set(range(n)):
        return 0.0  # Must visit each city exactly once
    
    # Compute tour length
    total_distance = 0.0
    for i in range(n):
        c1 = cities[tour[i]]
        c2 = cities[tour[(i + 1) % n]]
        total_distance += math.dist(c1, c2)
    
    # Score based on quality
    if known_optimal is not None:
        # Ratio to optimal (1.0 = optimal, decays as solution gets worse)
        ratio = known_optimal / total_distance  # <= 1.0
        return max(0.0, ratio)  # 0 if somehow negative (shouldn't happen)
    else:
        # Without known optimal, just return feasibility (binary)
        return 1.0  # Feasible solution found


def verify_bin_packing(items: list[float], bin_capacity: float, 
                       assignment: list[int], known_optimal: int = None) -> float:
    """
    Verify a bin packing solution.
    
    Args:
        items: List of item sizes
        bin_capacity: Capacity of each bin
        assignment: assignment[i] = bin index for item i
        known_optimal: Optimal number of bins (if known)
    """
    # Check feasibility: no bin exceeds capacity
    bins = {}
    for i, bin_idx in enumerate(assignment):
        if bin_idx not in bins:
            bins[bin_idx] = 0.0
        bins[bin_idx] += items[i]
        if bins[bin_idx] > bin_capacity + 1e-9:
            return 0.0  # Bin overflow
    
    n_bins_used = len(bins)
    
    if known_optimal is not None:
        return known_optimal / n_bins_used  # 1.0 if optimal
    else:
        # Lower bound: sum(items) / capacity
        lb = math.ceil(sum(items) / bin_capacity)
        return lb / n_bins_used


def verify_graph_coloring(adj_matrix: list[list[int]], coloring: list[int],
                          known_chromatic: int = None) -> float:
    """
    Verify a graph coloring solution.
    
    Args:
        adj_matrix: Adjacency matrix (0/1)
        coloring: coloring[i] = color of vertex i
        known_chromatic: Chromatic number (if known)
    """
    n = len(adj_matrix)
    
    # Check feasibility: no two adjacent vertices share a color
    for i in range(n):
        for j in range(i + 1, n):
            if adj_matrix[i][j] == 1 and coloring[i] == coloring[j]:
                return 0.0  # Adjacent vertices with same color
    
    n_colors = len(set(coloring))
    
    if known_chromatic is not None:
        return known_chromatic / n_colors
    else:
        return 1.0  # Valid coloring


def verify_scheduling(jobs: list[dict], schedule: list[dict],
                      known_optimal_makespan: float = None) -> float:
    """
    Verify a job-shop scheduling solution.
    
    Args:
        jobs: List of {"operations": [(machine, duration), ...]}
        schedule: List of {"job": j, "op": o, "start": t, "machine": m}
        known_optimal_makespan: Optimal makespan if known
    """
    # Check feasibility
    # 1. Each operation scheduled exactly once
    # 2. Operation precedence within jobs respected
    # 3. No machine conflicts (two ops on same machine at same time)
    
    if not check_schedule_feasibility(jobs, schedule):
        return 0.0
    
    # Compute makespan
    makespan = max(s["start"] + get_duration(jobs, s["job"], s["op"]) for s in schedule)
    
    if known_optimal_makespan is not None:
        return known_optimal_makespan / makespan
    else:
        return 1.0


def verify_knapsack(items: list[dict], capacity: float, 
                    selection: list[int], known_optimal: float = None) -> float:
    """
    Verify a 0/1 knapsack solution.
    
    Args:
        items: List of {"weight": w, "value": v}
        capacity: Knapsack capacity
        selection: List of 0/1 indicating which items to include
        known_optimal: Optimal value if known
    """
    total_weight = sum(items[i]["weight"] for i in range(len(items)) if selection[i] == 1)
    total_value = sum(items[i]["value"] for i in range(len(items)) if selection[i] == 1)
    
    if total_weight > capacity + 1e-9:
        return 0.0  # Over capacity
    
    if known_optimal is not None:
        return total_value / known_optimal
    else:
        return 1.0 if total_value > 0 else 0.0
```

## Dataset Sources

| Dataset | Size | Source | Problem Type |
|---------|------|--------|-------------|
| **TSPLIB** | ~110 instances | [comopt.ifi.uni-heidelberg.de/software/TSPLIB95](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) | TSP, ATSP, with known optima |
| **CVRPLIB** | ~300 instances | [vrp.atd-lab.inf.puc-rio.br](http://vrp.atd-lab.inf.puc-rio.br/) | Capacitated vehicle routing |
| **OR-Library** (Beasley) | ~2,000 instances | [people.brunel.ac.uk/~mastjjb/jeb/info.html](http://people.brunel.ac.uk/~mastjjb/jeb/info.html) | Bin packing, knapsack, set covering, scheduling, etc. |
| **Concorde TSP solutions** | ~110 | [math.uwaterloo.ca/tsp](http://www.math.uwaterloo.ca/tsp/) | Optimal TSP solutions for TSPLIB |
| **DIMACS graph coloring** | ~125 instances | [mat.tepper.cmu.edu/COLOR](https://mat.tepper.cmu.edu/COLOR/) | Graph coloring benchmarks |
| **Job-Shop benchmarks** (Taillard) | ~200 | Various | Scheduling benchmarks with known bounds |
| **MIPLIB** | ~1,500 | [miplib.zib.de](https://miplib.zib.de/) | Mixed-integer programming instances |
| **Kaggle Santa's Workshop** | ~5 competitions | Kaggle | Combinatorial optimization competitions |
| **Google OR-Tools examples** | ~100 | [developers.google.com/optimization](https://developers.google.com/optimization) | Various CO problems |
| **Procedural generation** | Unlimited | Custom | Random TSP, bin packing, etc. |

### Procedural Generation with Difficulty Control

```python
def generate_tsp_instance(n_cities: int, distribution: str = "uniform"):
    """Generate a TSP instance with controlled size and structure."""
    if distribution == "uniform":
        cities = [(random.uniform(0, 1000), random.uniform(0, 1000)) for _ in range(n_cities)]
    elif distribution == "clustered":
        n_clusters = random.randint(3, n_cities // 5)
        centers = [(random.uniform(0, 1000), random.uniform(0, 1000)) for _ in range(n_clusters)]
        cities = []
        for _ in range(n_cities):
            center = random.choice(centers)
            cities.append((center[0] + random.gauss(0, 50), center[1] + random.gauss(0, 50)))
    
    # Compute optimal (for small instances) or best-known bound
    if n_cities <= 20:
        optimal = solve_tsp_exact(cities)  # Exact solver
    else:
        optimal = solve_tsp_lkh(cities)  # LKH heuristic (near-optimal)
    
    return {"cities": cities, "known_optimal": optimal}

def generate_knapsack_instance(n_items: int, correlation: str = "uncorrelated"):
    """Generate a knapsack instance."""
    if correlation == "uncorrelated":
        items = [{"weight": random.randint(1, 100), "value": random.randint(1, 100)} 
                 for _ in range(n_items)]
    elif correlation == "weakly_correlated":
        items = []
        for _ in range(n_items):
            w = random.randint(1, 100)
            v = max(1, w + random.randint(-10, 10))
            items.append({"weight": w, "value": v})
    
    capacity = sum(i["weight"] for i in items) // 2
    
    # Solve optimally with DP (for small instances)
    if n_items <= 1000:
        optimal = solve_knapsack_dp(items, capacity)
    else:
        optimal = None
    
    return {"items": items, "capacity": capacity, "known_optimal": optimal}
```

## Task Format

**Input**: Problem description with instance data.

```
Problem: Find the shortest tour visiting all cities exactly once and returning 
to the start.

Cities (x, y):
0: (0, 0)
1: (10, 20)
2: (30, 10)
3: (25, 35)
4: (5, 15)

Known optimal tour length: 87.32

Provide your tour as an ordered list of city indices.
```

**Output**:
```
Tour: [0, 4, 1, 3, 2]
Tour length: 89.15
```

## Difficulty Curriculum

| Level | Problem | Size | Approx. Gap to Optimal | Scale |
|-------|---------|------|----------------------|-------|
| Easy | TSP (5-10 cities), Knapsack (10 items) | Tiny | Exact solve possible | Unlimited |
| Medium | TSP (20-50), Bin Packing (50 items) | Small | <5% gap achievable | Unlimited |
| Hard | TSP (100-500), Graph Coloring (100 nodes) | Medium | 1-10% gap | ~10K |
| Very Hard | TSP (1000+), Job-Shop (100 jobs) | Large | State-of-art heuristics needed | ~1K |
| Superhuman | TSPLIB hard instances, open MIPLIB | Benchmark | Unknown optimal | ~100 |

**Graduated reward**: Unlike binary domains, optimization allows continuous reward based on solution quality. This provides a much richer learning signal.

## Limitations & Risks

1. **LLMs are fundamentally bad at optimization**: LLMs process text sequentially and lack the ability to perform the systematic search that optimization requires. They may learn heuristics (nearest neighbor, greedy) but are unlikely to match dedicated solvers (LKH, Gurobi, CPLEX).
2. **Output format complexity**: Specifying a full TSP tour or bin assignment in text is verbose and error-prone. Structured output formats help but add complexity.
3. **Continuous vs. binary reward**: While graduated reward is an advantage, it requires careful reward shaping. A model might learn to always output the nearest-neighbor heuristic (which gives a decent score) without learning to improve.
4. **Solver dominance**: For well-studied problems (TSP, bin packing), decades of algorithm engineering have produced near-optimal solvers. The LLM's comparative advantage is unclear.
5. **Instance representation**: How to represent a graph or city list in text is a design choice that significantly affects model performance. Adjacency lists, edge lists, coordinate lists, and natural language all yield different results.

## Connections

- **graph-theory.md**: Many combinatorial optimization problems are defined on graphs.
- **logic-propositional.md**: Many optimization problems reduce to SAT/MaxSAT.
- **logic-puzzles.md**: Puzzles like Sudoku are constraint satisfaction problems — optimization without an objective function.
- **math-competition.md**: Competition combinatorics problems sometimes involve optimization.
- **number-theory-computation.md**: Some number-theoretic problems (shortest addition chain, etc.) are combinatorial optimization.
