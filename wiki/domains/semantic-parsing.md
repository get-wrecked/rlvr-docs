---
domain: semantic-parsing
category: language
verified: true
verification_type: exact
scalability: high
maturity: high
tags: [semantic-parsing, sql, amr, lambda-calculus, formal-language]
---

# Semantic Parsing

## Overview

Semantic parsing maps natural language utterances to formal representations: SQL queries, SPARQL queries, AMR (Abstract Meaning Representation), lambda calculus expressions, or domain-specific logical forms. The model must understand the meaning of the utterance and express it in a formal language that can be executed or evaluated.

This domain has strong RLVR properties. The formal output can be verified in two complementary ways: exact match against a gold logical form, or execution against a database/knowledge base and comparison of results. Execution-based verification is especially powerful because it tolerates syntactically different but semantically equivalent programs.

## Verification Mechanism

**Verification type: Exact**

Two verification strategies:

### 1. Exact match on logical form
```python
def verify_exact(predicted_sql: str, gold_sql: str) -> int:
    return 1 if normalize_sql(predicted_sql) == normalize_sql(gold_sql) else 0
```
Normalization includes: canonicalize table/column aliases, standardize keyword casing, normalize whitespace, sort commutative clauses (AND, OR).

**Limitation**: Different SQL queries can produce the same result. Exact match penalizes valid alternatives.

### 2. Execution-based verification (denotation match)
```python
def verify_execution(predicted_sql: str, gold_sql: str, database) -> int:
    try:
        pred_result = execute(predicted_sql, database)
        gold_result = execute(gold_sql, database)
        return 1 if pred_result == gold_result else 0
    except Exception:
        return 0  # Syntax error or runtime error
```

Execution-based verification is superior because it:
- Accepts semantically equivalent but syntactically different queries
- Catches queries that parse but return wrong results
- Naturally handles the many-ways-to-express-one-meaning problem

**Caution**: Execution match can produce false positives (different queries that happen to give the same result on a specific database instance). Using multiple database instances reduces this risk.

### 3. AMR-specific verification
For AMR parsing, Smatch (Semantic Match) score computes overlap between predicted and gold AMR graphs:
```
Smatch = F1 over (variable, relation, variable) triples in AMR graphs
```
Smatch is exact in the graph-matching sense (no approximation), though the graph alignment is solved approximately via hill climbing.

## Dataset Sources

| Dataset | Formal Language | Size | Source | Notes |
|---------|----------------|------|--------|-------|
| Spider | SQL | 10K questions, 200 DBs | Cross-database text-to-SQL | Multiple databases with complex schemas |
| WikiSQL | SQL | 81K questions | Wikipedia tables | Single-table, simpler queries |
| SParC | SQL (conversational) | 4.3K dialogues | Spider databases | Multi-turn text-to-SQL |
| CoSQL | SQL (conversational) | 3K dialogues | Spider databases | Collaborative SQL |
| GrailQA | SPARQL | 64K questions | Freebase | Knowledge base question answering |
| KQA Pro | KoPL | 120K questions | Wikidata | Compositional KBQA with programs |
| AMR 3.0 | AMR | 59K sentences | Multi-genre | Abstract Meaning Representation |
| COGS | Logical form | 24K examples | Synthetic | Compositional generalization |
| GeoQuery | Lambda calculus | 880 questions | US geography | Classic semantic parsing benchmark |
| SCAN | Action sequences | 20K examples | Synthetic | Compositional instruction following |
| Overnight | Lambda DCS | 26K examples | 8 domains | Domain-specific logical forms |

**Scalability**: For text-to-SQL, any database with data can generate questions via SQL template reversal (write SQL, generate NL question). Wikidata and Freebase support large-scale SPARQL question generation. Synthetic compositional datasets (COGS, SCAN) can be generated at arbitrary scale.

## Task Format

**Input**: A natural language question or instruction, plus a schema (for SQL) or ontology (for SPARQL/KBQA).

**Output**: A formal representation (SQL query, SPARQL query, AMR graph, logical form).

**Prompt template example** (text-to-SQL):
```
Given the following database schema, write a SQL query to answer the question.

Schema:
CREATE TABLE employees (id INT, name TEXT, department TEXT, salary INT);
CREATE TABLE departments (id INT, name TEXT, location TEXT);

Question: What is the average salary of employees in the Engineering department?

SQL:
```

For KBQA:
```
Convert the following question to a SPARQL query over Wikidata.

Question: Who directed the film that won the Academy Award for Best Picture in 2020?

SPARQL:
```

## Difficulty Curriculum

1. **Level 1 — Simple SELECT**: Single-table queries with WHERE clauses. "What is the population of France?"
2. **Level 2 — Aggregation**: COUNT, SUM, AVG, MAX, MIN with GROUP BY. "How many employees are in each department?"
3. **Level 3 — JOIN operations**: Multi-table queries requiring JOIN. "What departments are in New York?"
4. **Level 4 — Nested queries**: Subqueries and correlated subqueries. "Who earns more than the average salary?"
5. **Level 5 — Complex composition**: Multiple JOINs, nested aggregation, HAVING, UNION. Spider's "extra hard" category.
6. **Level 6 — Cross-database generalization**: Parse queries for databases not seen during training (Spider's cross-database setting).
7. **Level 7 — Conversational semantic parsing**: Multi-turn dialogue where queries build on previous context (SParC, CoSQL). Requires co-reference resolution and query composition.

## Limitations & Risks

- **Schema complexity**: Large database schemas may not fit in the model's context window. Schema linking (identifying relevant tables/columns) is a hard subproblem.
- **Execution environment**: Execution-based verification requires a running database, which adds infrastructure complexity.
- **False positive execution match**: Two different queries may produce the same result on a specific database instance by coincidence. Multiple test instances mitigate but don't eliminate this.
- **Domain specificity**: SQL parsing skills may not transfer to SPARQL, AMR, or other formal languages. Each requires domain-specific training.
- **Natural language ambiguity**: The same natural language question may map to multiple valid logical forms (different interpretations). Gold annotations capture one interpretation.
- **Compositionality**: Models often fail on novel compositions of known operations (the COGS/SCAN problem). RLVR training may improve compositional generalization, but this remains an open challenge.

## Connections

- **[table-understanding](table-understanding.md)**: Text-to-SQL is the primary approach for complex table QA.
- **[text-to-code-nl](text-to-code-nl.md)**: SQL generation is a form of code generation; semantic parsing and code generation share techniques.
- **[structured-output-generation](structured-output-generation.md)**: Formal representations are structured outputs.
- **[knowledge-graph-completion](knowledge-graph-completion.md)**: KBQA (semantic parsing over KGs) directly queries knowledge graphs.
- **[question-answering-closed](question-answering-closed.md)**: Semantic parsing is often the mechanism for answering knowledge base questions.
- **[reading-comprehension](reading-comprehension.md)**: Multi-hop question decomposition relates to compositional semantic parsing.
