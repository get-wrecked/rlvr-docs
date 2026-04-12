---
domain: Database Query Optimization
category: code-optimization
verification_type: execution
dataset_scale: 10K+ (from query workloads)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Database Query Optimization

## Overview
Given a SQL query and database schema (with statistics), rewrite the query or suggest an execution plan that produces the same results faster. Verification: both queries must return identical results, and the optimized version must be measurably faster.

## Verification Mechanism
```python
def verify(original_query: str, optimized_query: str, db: Database) -> float:
    # Check correctness: same results
    original_result = db.execute(original_query)
    optimized_result = db.execute(optimized_query)
    
    if not results_equal(original_result, optimized_result):
        return 0.0  # Wrong results = total failure
    
    # Measure performance
    original_time = db.benchmark(original_query, runs=5)
    optimized_time = db.benchmark(optimized_query, runs=5)
    
    if optimized_time >= original_time:
        return 0.1  # Correct but no improvement
    
    speedup = original_time / optimized_time
    return min(1.0, speedup / 10)  # Cap at 10x improvement
```

## Dataset Sources
- **TPC-H**: Standard OLAP benchmark with 22 queries. Scale factors from 1GB to 100TB.
- **TPC-DS**: 99 queries, more complex.
- **JOB (Join Order Benchmark)**: Real-world queries from IMDB data.
- **Stack Overflow data dump**: Real query workloads.
- **Calcite test suite**: Apache Calcite's query optimization tests.
- **SlowQueryLog**: Extract slow queries from MySQL/PostgreSQL logs (many available on GitHub).

## Task Format
- **Input**: SQL query + schema + table statistics (row counts, index info)
- **Output**: Optimized SQL query (or execution plan hints)

## Difficulty Curriculum
- Level 1: Simple predicate pushdown, unnecessary DISTINCT removal
- Level 3: Join reordering, subquery unnesting
- Level 5: Complex rewrites (correlated subquery → join, window functions)
- Level 7: Multi-query optimization, materialized view selection
- Level 9: Workload-level optimization (index selection + query rewriting)

## Limitations & Risks
- Performance measurement is noisy (caching, system load). Need careful benchmarking setup.
- Optimization is database-engine-specific (PostgreSQL vs MySQL vs etc.).
- Some "optimizations" may only be faster for specific data distributions.

## Connections
- [[sql-generation]] — generating SQL from scratch
- [[code-optimization]] — general code optimization
- [[combinatorics-optimization]] — query plan selection is combinatorial
