---
domain: Graph Algorithm Execution
category: algorithms
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Graph Algorithm Execution

## Overview
Given a graph and an algorithm, manually execute the algorithm and report the result: shortest paths, MST weight, topological order, connected components, max flow, bipartite matching, coloring number. This tests algorithmic reasoning — the agent must simulate the algorithm mentally rather than just calling a library.

## Verification Mechanism
```python
import networkx as nx

def verify(graph: nx.Graph, task: str, answer: Any) -> float:
    if task == "shortest_path":
        correct = nx.shortest_path_length(graph, answer["source"], answer["target"], weight="weight")
        return 1.0 if answer["length"] == correct else 0.0
    
    elif task == "mst_weight":
        correct = sum(d["weight"] for u, v, d in nx.minimum_spanning_tree(graph).edges(data=True))
        return 1.0 if abs(answer - correct) < 1e-6 else 0.0
    
    elif task == "topological_sort":
        # Check if it's a valid topological ordering
        order = answer
        position = {node: i for i, node in enumerate(order)}
        for u, v in graph.edges():
            if position[u] >= position[v]:
                return 0.0
        return 1.0
    
    elif task == "max_flow":
        correct = nx.maximum_flow_value(graph, answer["source"], answer["sink"])
        return 1.0 if abs(answer["value"] - correct) < 1e-6 else 0.0
    
    elif task == "chromatic_number":
        # Verify coloring is valid
        coloring = answer["coloring"]
        for u, v in graph.edges():
            if coloring[u] == coloring[v]:
                return 0.0
        # Check number of colors matches claim
        num_colors = len(set(coloring.values()))
        return 1.0 if num_colors == answer["chromatic_number"] else 0.5
    
    elif task == "connected_components":
        correct = nx.number_connected_components(graph)
        return 1.0 if answer == correct else 0.0
    
    elif task == "is_bipartite":
        correct = nx.is_bipartite(graph)
        return 1.0 if answer == correct else 0.0
    
    elif task == "cycle_detection":
        has_cycle = not nx.is_directed_acyclic_graph(graph)
        if answer["has_cycle"] != has_cycle:
            return 0.0
        if has_cycle and "cycle" in answer:
            # Verify the reported cycle is valid
            cycle = answer["cycle"]
            for i in range(len(cycle)):
                if not graph.has_edge(cycle[i], cycle[(i+1) % len(cycle)]):
                    return 0.5  # Correct detection, wrong cycle
        return 1.0
```

## Dataset Sources
- **Procedural generation**: Random graphs (Erdos-Renyi, Barabasi-Albert, planted partition). Unlimited.
- **Graph algorithm textbooks**: CLRS, Kleinberg & Tardos — problem sets with solutions.
- **Graph benchmarks**: SNAP datasets, KONECT, Network Repository.
- **Competition problems**: Graph-heavy competitive programming problems.

## Task Format
- **Input**: "Graph with edges: A-B(3), A-C(1), B-C(1), B-D(4), C-D(2). Find the shortest path from A to D."
- **Output**: "A→C→D, length = 3"

## Difficulty Curriculum
- Level 1: BFS/DFS on small graphs (5-10 nodes)
- Level 3: Dijkstra, Kruskal on medium graphs (20-50 nodes)
- Level 5: Max flow, bipartite matching (50-100 nodes)
- Level 7: Large graph analysis (1000+ nodes, requires efficient algorithms)
- Level 9: NP-hard graph problems (coloring, clique) on graphs near the hardness threshold

## Limitations & Risks
- For NP-hard problems, we verify feasibility and quality (number of colors used), not optimality.
- Multiple valid answers may exist (many valid topological sorts). Accept any valid one.
- Graph algorithms are well-understood — verification is straightforward via NetworkX.

## Connections
- [[graph-theory]] — theoretical properties
- [[combinatorics-optimization]] — graph optimization
- [[planning-classical]] — graph search as planning
