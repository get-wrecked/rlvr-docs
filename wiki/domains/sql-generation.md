---
domain: SQL Generation
category: Code & Software
verification_type: execution-based (query result set comparison)
dataset_scale: 5K–200K question-query pairs
difficulty_range: single-table SELECT → multi-table nested subqueries with window functions
modality: text-to-SQL
status: mature
---

## Overview

SQL generation (text-to-SQL) tasks the model with translating a natural-language question into a SQL query that, when executed against a given database, returns the correct result set. Verification is fully automatic: execute the generated query and the reference query against the same database, then compare result sets.

This is one of the most mature RLVR domains. The verification is deterministic, datasets are large and well-curated, and difficulty scales naturally with SQL complexity. The domain has seen sustained research attention since WikiSQL (2017) through Spider, BIRD, and beyond.

## Verification Mechanism

```
def verify_sql(question, generated_sql, reference_sql, database):
    # 1. Execute generated query
    try:
        gen_result = execute_sql(
            database=database,
            query=generated_sql,
            timeout=30  # seconds
        )
    except SQLSyntaxError as e:
        return 0.0  # invalid SQL
    except SQLTimeoutError:
        return 0.0  # query too slow (likely cartesian join)
    except SQLExecutionError:
        return 0.0  # runtime error (e.g., column doesn't exist)

    # 2. Execute reference query
    ref_result = execute_sql(database=database, query=reference_sql, timeout=30)

    # 3. Compare result sets
    #    Method A: Exact set match (order-independent)
    gen_rows = set(tuple(row) for row in gen_result.rows)
    ref_rows = set(tuple(row) for row in ref_result.rows)
    if gen_rows == ref_rows:
        return 1.0

    # Method B: Execution accuracy with tolerance for numerics
    if result_match_with_tolerance(gen_result, ref_result, rtol=1e-6):
        return 1.0

    return 0.0
```

More sophisticated comparison for partial credit:

```
def verify_sql_detailed(generated_sql, reference_sql, database, test_databases):
    """
    Run against multiple database instances to catch queries that
    happen to produce the right answer on one DB but are logically wrong.
    """
    scores = []
    for db in test_databases:  # same schema, different data
        gen = execute_sql(db, generated_sql)
        ref = execute_sql(db, reference_sql)
        scores.append(1.0 if result_match(gen, ref) else 0.0)
    return min(scores)  # must be correct on ALL database instances
```

Key considerations:

- **Execution match vs. exact match**: Older metrics compared SQL strings; modern practice executes both queries and compares outputs. Two syntactically different queries can be semantically identical.
- **Multiple valid queries**: The same question often has many correct SQL translations. Execution-based comparison handles this naturally.
- **Multiple database instances**: Running queries against multiple database instances with different data (same schema) catches "accidentally correct" queries. BIRD and the test-suite approach in Spider use this.
- **ORDER BY handling**: If the question implies ordering, compare ordered lists; otherwise, compare sets.

## Dataset Sources

| Dataset | Size | Databases | Difficulty | Notes |
|---------|------|-----------|-----------|-------|
| **WikiSQL** | 80,654 pairs | 26,521 tables | Easy (single table) | First large text-to-SQL dataset; simple queries only |
| **Spider** | 10,181 pairs | 200 DBs (138 domains) | Easy → Extra Hard | Multi-table, complex joins, nested queries; gold standard |
| **Spider 2.0** | 632 examples | Enterprise DBs | Hard → Extra Hard | Real enterprise data warehouses (BigQuery, dbt, Snowflake) |
| **BIRD** | 12,751 pairs | 95 DBs (37 domains) | Medium → Hard | Emphasizes real-world messy data with external knowledge |
| **SQA (Sequential QA)** | 6,066 sequences | HTML tables | Easy–Medium | Multi-turn sequential questions on tables |
| **SEDE** | 12,023 pairs | StackExchange DB | Medium–Hard | Real queries from StackExchange Data Explorer |
| **KaggleDBQA** | 272 pairs | 8 real Kaggle DBs | Medium | Real-world databases with domain-specific terminology |
| **CSpider** | 10,181 pairs | 200 DBs | Easy → Extra Hard | Chinese version of Spider |
| **DuSQL** | 23,797 pairs | 200 DBs | Mixed | Chinese text-to-SQL |
| **CHASE** | 17,940 pairs | 280 DBs | Mixed | Cross-database, context-dependent |

For RL training, Spider and BIRD together provide ~23K diverse examples across hundreds of database schemas, covering the full difficulty spectrum.

## Task Format

**Input prompt**:
```
Database schema:
  students(id, name, major, gpa, enrollment_year)
  courses(id, title, department, credits)
  enrollments(student_id, course_id, grade, semester)

Question: What is the average GPA of students majoring in
Computer Science who have taken more than 5 courses?
```

**Expected output**:
```sql
SELECT AVG(s.gpa)
FROM students s
WHERE s.major = 'Computer Science'
  AND (SELECT COUNT(*) FROM enrollments e
       WHERE e.student_id = s.id) > 5;
```

**Verification**: Execute both the generated and reference query against the database; compare result sets.

## Difficulty Curriculum

| Level | SQL Features | Spider Difficulty | Example |
|-------|-------------|------------------|---------|
| 1 | Single table, WHERE, basic aggregation | Easy | "How many students have GPA > 3.5?" |
| 2 | JOIN two tables, GROUP BY | Medium | "Count courses per department" |
| 3 | Nested subqueries, HAVING | Hard | "Departments where avg grade > B" |
| 4 | Correlated subqueries, UNION, EXCEPT | Extra Hard | "Students in all CS courses but no math courses" |
| 5 | Window functions, CTEs, complex joins | Beyond Spider | "Running average of sales by quarter, ranked by region" |
| 6 | Cross-database, external knowledge | BIRD-style | Requires understanding domain conventions and dirty data |

Spider's built-in difficulty labels (Easy/Medium/Hard/Extra Hard) provide a ready-made curriculum. BIRD adds the dimension of real-world data messiness.

## Limitations & Risks

- **Schema ambiguity**: Natural-language questions can be ambiguous about which columns/tables to use. Multiple interpretations may be valid, but only one reference query exists. Multi-database-instance testing helps but doesn't fully solve this.
- **Data-dependent correctness**: A query might return the correct result on the test database by coincidence (e.g., a missing WHERE clause doesn't matter if the data happens to filter naturally). Multiple database instances mitigate this.
- **NULL handling**: SQL NULL semantics are subtle (NULL != NULL, three-valued logic). Test databases should include NULLs to exercise this.
- **Dialect differences**: SQLite, PostgreSQL, MySQL, and BigQuery have syntax differences. Most benchmarks use SQLite; real-world applicability requires dialect awareness.
- **Execution cost**: Running thousands of SQL queries is fast for simple databases but can be slow for large schemas or complex queries. Pre-loaded in-memory databases (SQLite) keep verification fast.

## Connections

- Shares the "execute and compare" verification paradigm with **Code Generation**.
- **Data Wrangling** is the programmatic cousin: pandas/R transformations verified against reference DataFrames.
- **Database Migrations** operate on schema-level SQL rather than query-level SQL.
- Natural-language understanding aspects connect to other NL→formal-language domains like **Regex Synthesis** and **Shell Commands**.
- **API Usage** shares the "generate a structured call, verify the response" pattern.
