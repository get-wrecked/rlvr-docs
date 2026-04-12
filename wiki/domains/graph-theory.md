---
domain: Graph Theory
category: Math
verification_type: execution
dataset_scale: ~50K existing + unlimited procedural
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

## Overview

Graph theory problems ask the model to compute graph properties (chromatic number, clique number, independence number, diameter, connectivity, planarity), find specific substructures (Hamiltonian paths, Eulerian circuits, matchings), or determine graph-theoretic quantities (number of spanning trees, Tutte polynomial evaluations). Verification is clean: compute the property independently using graph algorithms.

This domain bridges mathematics and computer science. Many graph problems appear in math competitions, algorithm courses, and combinatorial optimization. The verification infrastructure is excellent — NetworkX, SageMath, and dedicated graph libraries can check virtually any graph property. Datasets range from textbook exercises to the DIMACS challenge problems.

Graph theory is partially validated in RLVR through its overlap with combinatorial optimization (see `combinatorics-optimization.md`) and competition math. Dedicated RLVR on graph-theoretic reasoning is underexplored but has strong potential.

## Verification Mechanism

**Primary method: Compute the graph property independently using graph algorithms.**

```python
import networkx as nx
import numpy as np

def verify_graph_property(graph_data: dict, property_name: str, 
                          model_answer, tolerance=0) -> float:
    """
    Verify a graph theory answer by computing the property.
    
    Args:
        graph_data: {"nodes": [...], "edges": [...]} or adjacency matrix
        property_name: Which property to verify
        model_answer: The model's claimed answer
    """
    G = build_graph(graph_data)
    
    if property_name == "chromatic_number":
        # NP-hard in general, but computable for small graphs
        gold = compute_chromatic_number(G)  # exact for small graphs
        return 1.0 if int(model_answer) == gold else 0.0
    
    elif property_name == "clique_number":
        gold = nx.graph_clique_number(G)
        return 1.0 if int(model_answer) == gold else 0.0
    
    elif property_name == "independence_number":
        # alpha(G) = omega(complement(G))
        complement = nx.complement(G)
        gold = nx.graph_clique_number(complement)
        return 1.0 if int(model_answer) == gold else 0.0
    
    elif property_name == "is_planar":
        gold = nx.check_planarity(G)[0]
        return 1.0 if model_answer == gold else 0.0
    
    elif property_name == "is_bipartite":
        gold = nx.is_bipartite(G)
        return 1.0 if model_answer == gold else 0.0
    
    elif property_name == "diameter":
        if nx.is_connected(G):
            gold = nx.diameter(G)
            return 1.0 if int(model_answer) == gold else 0.0
        else:
            return 1.0 if model_answer == "infinite" or model_answer == float('inf') else 0.0
    
    elif property_name == "num_connected_components":
        gold = nx.number_connected_components(G)
        return 1.0 if int(model_answer) == gold else 0.0
    
    elif property_name == "num_spanning_trees":
        # Kirchhoff's theorem: det of any cofactor of Laplacian
        L = nx.laplacian_matrix(G).toarray()
        gold = int(round(np.linalg.det(L[1:, 1:])))
        return 1.0 if int(model_answer) == gold else 0.0
    
    elif property_name == "edge_connectivity":
        gold = nx.edge_connectivity(G)
        return 1.0 if int(model_answer) == gold else 0.0
    
    elif property_name == "vertex_connectivity":
        gold = nx.node_connectivity(G)
        return 1.0 if int(model_answer) == gold else 0.0
    
    elif property_name == "is_eulerian":
        gold = nx.is_eulerian(G)
        return 1.0 if model_answer == gold else 0.0
    
    elif property_name == "is_hamiltonian":
        # NP-complete, only feasible for small graphs
        gold = check_hamiltonian(G)  # brute-force for small graphs
        return 1.0 if model_answer == gold else 0.0
    
    elif property_name == "matching_number":
        matching = nx.max_weight_matching(G)
        gold = len(matching)
        return 1.0 if int(model_answer) == gold else 0.0
    
    return 0.0


def verify_graph_construction(problem: dict, model_graph: dict) -> float:
    """
    Verify that a model-constructed graph satisfies required properties.
    
    E.g., "Construct a 3-regular graph on 6 vertices"
    """
    G = build_graph(model_graph)
    
    for prop, value in problem["required_properties"].items():
        if prop == "n_vertices":
            if G.number_of_nodes() != value:
                return 0.0
        elif prop == "n_edges":
            if G.number_of_edges() != value:
                return 0.0
        elif prop == "is_regular" and value is True:
            degrees = [d for _, d in G.degree()]
            if len(set(degrees)) != 1:
                return 0.0
        elif prop == "regularity_degree":
            degrees = [d for _, d in G.degree()]
            if not all(d == value for d in degrees):
                return 0.0
        elif prop == "is_connected":
            if nx.is_connected(G) != value:
                return 0.0
        elif prop == "is_planar":
            if nx.check_planarity(G)[0] != value:
                return 0.0
        elif prop == "chromatic_number":
            if compute_chromatic_number(G) != value:
                return 0.0
        elif prop == "girth":
            if compute_girth(G) != value:
                return 0.0
    
    return 1.0


def verify_path_or_cycle(G: nx.Graph, model_path: list, 
                          path_type: str = "path") -> float:
    """
    Verify a Hamiltonian path/cycle or Eulerian path/circuit.
    """
    if path_type == "hamiltonian_path":
        # Check: visits every vertex exactly once
        if set(model_path) != set(G.nodes()):
            return 0.0
        if len(model_path) != G.number_of_nodes():
            return 0.0
        # Check: consecutive vertices are adjacent
        for i in range(len(model_path) - 1):
            if not G.has_edge(model_path[i], model_path[i + 1]):
                return 0.0
        return 1.0
    
    elif path_type == "hamiltonian_cycle":
        # Same as path, plus last connects to first
        result = verify_path_or_cycle(G, model_path, "hamiltonian_path")
        if result == 0.0:
            return 0.0
        if not G.has_edge(model_path[-1], model_path[0]):
            return 0.0
        return 1.0
    
    elif path_type == "eulerian_circuit":
        # Check: traverses every edge exactly once, returns to start
        if model_path[0] != model_path[-1]:
            return 0.0
        edges_traversed = set()
        for i in range(len(model_path) - 1):
            edge = frozenset([model_path[i], model_path[i + 1]])
            if edge in edges_traversed:
                return 0.0  # Edge repeated
            if not G.has_edge(model_path[i], model_path[i + 1]):
                return 0.0  # Edge doesn't exist
            edges_traversed.add(edge)
        if len(edges_traversed) != G.number_of_edges():
            return 0.0  # Didn't cover all edges
        return 1.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **DIMACS Graph Challenges** | ~500 instances | [dimacs.rutgers.edu](http://dimacs.rutgers.edu/) | Coloring, cliques, shortest paths |
| **NetworkRepository** | 5,000+ graphs | [networkrepository.com](https://networkrepository.com/) | Real-world graphs with known properties |
| **Stanford SNAP** | 100+ datasets | [snap.stanford.edu](https://snap.stanford.edu/) | Social/information/biological networks |
| **House of Graphs** | ~31,000 graphs | [houseofgraphs.org](https://houseofgraphs.org/) | Interesting/notable graphs |
| **MATH** (graph theory subset) | ~200 | hendrycks/math | Competition-level graph problems |
| **Graph Theory textbook problems** | ~5K | Bondy & Murty, West, Diestel | Standard exercises |
| **TSPLIB** | ~110 | See combinatorics-optimization.md | TSP on complete graphs |
| **Petersen Graph catalog** | Named graphs | Various | Classic graphs with known properties |
| **SageMath graph database** | ~1,000 named graphs | SageMath `graphs.` namespace | Directly accessible in SageMath |
| **Competition problems** | ~1K | AMC/AIME/Olympiad | Graph theory in competitions |

### Procedural Generation

```python
def generate_graph_theory_problem(difficulty="medium"):
    """Generate a graph theory problem with verified answer."""
    
    if difficulty == "easy":
        # Small named graph, basic property
        n = random.randint(4, 8)
        graph_type = random.choice(["cycle", "complete", "path", "star", "wheel"])
        
        if graph_type == "cycle":
            G = nx.cycle_graph(n)
            name = f"C_{n}"
        elif graph_type == "complete":
            G = nx.complete_graph(n)
            name = f"K_{n}"
        elif graph_type == "path":
            G = nx.path_graph(n)
            name = f"P_{n}"
        elif graph_type == "star":
            G = nx.star_graph(n - 1)
            name = f"S_{n}"
        elif graph_type == "wheel":
            G = nx.wheel_graph(n)
            name = f"W_{n}"
        
        prop = random.choice([
            "num_edges", "diameter", "is_bipartite", "chromatic_number",
            "is_eulerian", "num_connected_components", "max_degree"
        ])
        
        answer = compute_property(G, prop)
        return {
            "problem": f"What is the {prop.replace('_', ' ')} of the graph {name}?",
            "answer": answer,
            "graph": nx.node_link_data(G)
        }
    
    elif difficulty == "medium":
        # Random graph, moderate property
        n = random.randint(8, 15)
        p = random.uniform(0.2, 0.6)
        G = nx.erdos_renyi_graph(n, p, seed=random.randint(0, 10000))
        
        prop = random.choice([
            "chromatic_number", "clique_number", "independence_number",
            "edge_connectivity", "vertex_connectivity", "matching_number",
            "num_spanning_trees"
        ])
        
        answer = compute_property(G, prop)
        edges = list(G.edges())
        
        return {
            "problem": f"Consider the graph on {n} vertices with edges: {edges}. Find the {prop.replace('_', ' ')}.",
            "answer": answer
        }
    
    elif difficulty == "hard":
        # Larger graphs, harder properties
        n = random.randint(15, 25)
        G = nx.erdos_renyi_graph(n, 0.3)
        
        prop = random.choice([
            "is_hamiltonian", "treewidth", "genus"
        ])
        
        answer = compute_property(G, prop)
        return {
            "problem": f"For the given graph on {n} vertices, determine the {prop.replace('_', ' ')}.",
            "answer": answer
        }
```

## Task Format

**Input**: Graph description (edge list, adjacency matrix, or named graph) + question.

```
Problem: Consider the Petersen graph. What is its chromatic number?

Expected answer: 3
```

```
Problem: A graph G has 6 vertices and edges: 
{(0,1), (0,2), (1,2), (1,3), (2,4), (3,4), (3,5), (4,5)}.
Find the number of spanning trees of G.

Expected answer: 75
```

```
Problem: Does the following graph have a Hamiltonian cycle?
Vertices: {0, 1, 2, 3, 4}
Edges: {(0,1), (0,3), (1,2), (1,4), (2,3), (3,4)}

Expected answer: Yes. One Hamiltonian cycle: 0-1-2-3-4-0
(Verification: check all consecutive pairs are edges and all vertices visited)
```

**Output**: The answer, optionally with the specific substructure (path, coloring, etc.).

## Difficulty Curriculum

| Level | Type | Graph Size | Scale |
|-------|------|-----------|-------|
| Easy | Named graphs (K_n, C_n, P_n), basic properties | 4-8 vertices | Unlimited |
| Medium | Random graphs, chromatic/clique/independence number | 8-15 vertices | Unlimited |
| Hard | Hamiltonicity, larger random/structured graphs | 15-30 vertices | Unlimited |
| Very Hard | Extremal graph theory, Ramsey numbers | 30+ vertices or theoretical | ~1K |
| Superhuman | Open conjectures (Hadwiger, etc.), large DIMACS instances | Benchmark-scale | ~100 |

## Limitations & Risks

1. **NP-hard properties**: Many interesting graph properties (chromatic number, Hamiltonicity, clique number) are NP-hard to compute. The verifier itself is expensive for large graphs. This limits problem size.
2. **Graph representation in text**: Representing a graph in text (edge list, adjacency matrix) is verbose and error-prone. A 20-vertex graph with 50 edges requires substantial text just for the input. Models struggle to track connectivity mentally.
3. **Named graph memorization**: Models may memorize properties of named graphs (Petersen: chromatic number 3, not Hamiltonian) from training data rather than computing them. Random graphs mitigate this.
4. **Spatial reasoning limitations**: Graph theory requires spatial/structural reasoning that LLMs are not naturally good at. Performance may plateau quickly.
5. **Output complexity**: Some answers (a specific Hamiltonian path, a coloring) are complex to express in text. The extraction and verification pipeline must handle this.

## Connections

- **combinatorics-optimization.md**: Graph coloring, TSP, maximum matching are both graph theory and combinatorial optimization.
- **logic-propositional.md**: Graph coloring and many graph problems reduce to SAT.
- **logic-puzzles.md**: Some puzzles (e.g., map coloring) are graph theory problems.
- **math-competition.md**: Graph theory appears frequently in competition combinatorics.
- **automated-reasoning.md**: Graph property verification is a specific form of automated reasoning.
