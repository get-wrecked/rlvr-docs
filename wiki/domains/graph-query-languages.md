---
domain: Graph Database Queries (Cypher/Gremlin)
category: data-querying
verification_type: execution
dataset_scale: 50K+ (from graph DB benchmarks)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Graph Database Queries (Cypher/Gremlin)

## Overview
Generate graph database queries (Cypher for Neo4j, Gremlin for TinkerPop/JanusGraph) from natural language. Distinct from SPARQL (RDF/semantic web) — property graph model with different query semantics. Verification: execute query, compare result set.

## Verification Mechanism
```python
def verify(question: str, query: str, db: GraphDB, reference_answer: Any) -> float:
    try:
        result = db.execute(query)
    except QueryError:
        return 0.0
    
    result_set = normalize_graph_results(result)
    expected_set = normalize_graph_results(reference_answer)
    
    if result_set == expected_set:
        return 1.0
    
    # Partial credit
    if len(expected_set) > 0:
        precision = len(result_set & expected_set) / max(len(result_set), 1)
        recall = len(result_set & expected_set) / len(expected_set)
        return 2 * precision * recall / max(precision + recall, 1e-10)
    
    return 0.0
```

## Dataset Sources
- **Neo4j Movie Database**: Standard tutorial graph.
- **LDBC Social Network Benchmark (SNB)**: Industry-standard graph DB benchmark.
- **Northwind graph dataset**: Business data modeled as graph.
- **Panama Papers graph**: Financial network data.
- **Procedural generation**: Generate random property graphs + natural language questions.
- **Graph database courses**: Neo4j certification exercises.

## Task Format
- **Input**: "Using this movie database, find all actors who acted in a movie directed by Steven Spielberg and also acted in a movie with Tom Hanks" + schema description
- **Output**: Cypher query: `MATCH (a:Actor)-[:ACTED_IN]->(m:Movie)<-[:DIRECTED]-(d:Director {name:'Steven Spielberg'}), (a)-[:ACTED_IN]->(m2:Movie)<-[:ACTED_IN]-(t:Actor {name:'Tom Hanks'}) RETURN DISTINCT a.name`

## Difficulty Curriculum
- Level 1: Single-hop pattern matching (MATCH ... RETURN)
- Level 3: Multi-hop patterns, filtering, aggregation
- Level 5: Variable-length paths, path finding, WITH clauses
- Level 7: Complex graph algorithms (shortest path, community detection via query)
- Level 9: Graph data science (PageRank, Node2Vec via GDS library)

## Connections
- [[sql-generation]] — relational querying
- [[knowledge-base-querying]] — SPARQL/semantic web querying
- [[graph-algorithm-execution]] — graph algorithms
