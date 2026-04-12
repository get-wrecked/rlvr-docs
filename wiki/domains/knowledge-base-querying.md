---
domain: Knowledge Base Querying (SPARQL/Cypher)
category: data-querying
verification_type: execution
dataset_scale: 100K+ (Wikidata, Freebase)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Knowledge Base Querying (SPARQL/Cypher)

## Overview
Generate graph database queries (SPARQL for RDF/Wikidata, Cypher for Neo4j) from natural language questions. Verification: execute the query against the database and compare result sets to the reference answer.

## Verification Mechanism
```python
def verify(question: str, generated_query: str, 
           reference_answer: set, endpoint: str) -> float:
    try:
        # Execute the generated query
        results = execute_sparql(generated_query, endpoint)
        result_set = normalize_results(results)
    except QueryError:
        return 0.0
    
    # Compare result sets
    reference_set = normalize_results(reference_answer)
    
    if result_set == reference_set:
        return 1.0
    
    # Partial credit for partial overlap
    if len(reference_set) > 0:
        precision = len(result_set & reference_set) / max(len(result_set), 1)
        recall = len(result_set & reference_set) / len(reference_set)
        f1 = 2 * precision * recall / max(precision + recall, 1e-10)
        return f1
    
    return 0.0
```

## Dataset Sources
- **LC-QuAD 2.0**: 30K SPARQL queries over Wikidata and DBpedia.
- **KQA Pro**: 120K questions with SPARQL/KoPL queries over Wikidata.
- **GrailQA**: 64K questions over Freebase.
- **WebQuestionsSP**: 4K questions with SPARQL.
- **SimpleQuestions**: 100K simple questions over Freebase.
- **QALD (Question Answering over Linked Data)**: Long-running challenge series.
- **Wikidata**: 100B+ triples — the largest freely queryable knowledge base.

## Task Format
- **Input**: "Who directed the movie that won Best Picture in 2020?" + schema hints
- **Output**: SPARQL query against Wikidata

## Difficulty Curriculum
- Level 1: Single-triple queries ("Who is the president of France?")
- Level 3: Multi-hop queries (2-3 joins)
- Level 5: Aggregation, filtering, OPTIONAL patterns
- Level 7: Complex path queries, subqueries, federation
- Level 9: Multi-constraint reasoning over large subgraphs

## Limitations & Risks
- Wikidata is constantly updated — answers change over time. Pin to a specific dump version.
- SPARQL endpoint availability and performance vary.
- Schema knowledge is critical — the agent needs to know Wikidata property IDs. Consider providing schema hints.

## Connections
- [[sql-generation]] — relational querying (closely related)
- [[semantic-parsing]] — parsing NL to formal query
- [[knowledge-graph-completion]] — the KG that's being queried
- [[question-answering-closed]] — answering the same questions in a different way
