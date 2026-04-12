---
domain: Social Network Analysis
category: data-science
verification_type: execution
dataset_scale: 1M+ (from social network datasets)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Social Network Analysis

## Overview
Analyze network/graph datasets: compute centrality measures, detect communities, predict links, identify influencers, measure network properties (small-world, scale-free). Verification: compute metrics using NetworkX/igraph and compare.

## Verification Mechanism
```python
import networkx as nx

def verify(task_type: str, graph: nx.Graph, answer: Any) -> float:
    if task_type == "centrality":
        correct = nx.betweenness_centrality(graph)
        if isinstance(answer, dict):
            errors = [abs(answer.get(n, 0) - correct[n]) for n in correct]
            return 1.0 if max(errors) < 0.01 else max(0, 1 - max(errors))
        else:
            # Top-k most central
            top_k = sorted(correct, key=correct.get, reverse=True)[:answer["k"]]
            return set_f1(set(answer["nodes"]), set(top_k))
    
    elif task_type == "community_detection":
        # Normalized Mutual Information between predicted and ground truth
        from sklearn.metrics import normalized_mutual_info_score
        nmi = normalized_mutual_info_score(answer["communities"], ground_truth["communities"])
        return nmi
    
    elif task_type == "network_property":
        if answer["property"] == "clustering_coefficient":
            correct = nx.average_clustering(graph)
            return 1.0 if abs(answer["value"] - correct) < 0.01 else 0.0
        elif answer["property"] == "diameter":
            correct = nx.diameter(graph)
            return 1.0 if answer["value"] == correct else 0.0
        elif answer["property"] == "density":
            correct = nx.density(graph)
            return 1.0 if abs(answer["value"] - correct) < 0.001 else 0.0
```

## Dataset Sources
- **SNAP (Stanford Network Analysis Platform)**: 100+ real-world networks.
- **KONECT**: Thousands of network datasets.
- **Network Repository**: 5K+ network datasets.
- **Facebook/Twitter research data**: Academic social network datasets.
- **Pokec**: 1.6M-node Slovak social network.
- **DBLP**: Academic collaboration network.
- **Wikipedia edit network**: Editor interaction network.

## Task Format
- **Input**: Network edge list + "Find the top 5 nodes by betweenness centrality"
- **Output**: [node_A, node_B, node_C, node_D, node_E]

## Difficulty Curriculum
- Level 1: Basic graph metrics (degree distribution, density)
- Level 3: Centrality measures, shortest paths
- Level 5: Community detection, link prediction
- Level 7: Temporal network analysis, influence propagation
- Level 9: Network formation models, epidemic spreading on networks

## Connections
- [[graph-theory]] — graph theory foundations
- [[graph-algorithm-execution]] — graph algorithms
- [[data-science-eda]] — data analysis
