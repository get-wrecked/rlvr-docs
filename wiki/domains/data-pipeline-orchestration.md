---
domain: Data Pipeline Orchestration
category: Code & Software
verification_type: execution
dataset_scale: ~10K+ Airflow DAGs on GitHub + dbt examples
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Data Pipeline Orchestration

## Overview
Generate data pipeline code (Airflow DAGs, dbt models, Spark jobs, Prefect flows). Verification via DAG validation, execution against test data, and dependency ordering checks.

## Verification Mechanism
1. DAG parses without errors (Airflow's `python -c "import dag"`)
2. Pipeline runs against test data, output matches expected
3. Dependency ordering is correct (topological sort verification)
4. Idempotency check (running twice produces same result)

## Dataset Sources
- Astronomer Airflow examples
- dbt example projects (jaffle_shop)
- Spark examples
- Public Airflow DAGs from GitHub
- Prefect flow examples

## Task Format
**Input**: Data sources, transformations, sinks, scheduling requirements
**Output**: Airflow DAG Python file, dbt model SQL, or Spark job code

## Difficulty Curriculum
1. Simple extract-transform-load (ETL)
2. Branching and conditional execution
3. Dynamic task generation
4. Cross-DAG dependencies and sensors
5. Complex data quality checks and SLA monitoring

## Connections
[[workflow-automation]], [[data-wrangling]], [[sql-generation]]
