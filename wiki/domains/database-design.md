---
domain: Database Schema Design & Normalization
category: data-engineering
verification_type: constraint
dataset_scale: 50K+ (from database courses + real schemas)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Database Schema Design & Normalization

## Overview
Design relational database schemas from requirements, normalize schemas to specific normal forms (1NF, 2NF, 3NF, BCNF), identify functional dependencies. Verification: check normal form properties algorithmically, verify functional dependencies.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: dict) -> float:
    if task_type == "normalize":
        schema = answer["schema"]
        target_nf = problem["target_normal_form"]
        
        # Check if schema is in target normal form
        fds = problem["functional_dependencies"]
        
        if target_nf == "3NF":
            violations = check_3nf(schema, fds)
            return 1.0 if len(violations) == 0 else max(0, 1 - len(violations)/10)
        elif target_nf == "BCNF":
            violations = check_bcnf(schema, fds)
            return 1.0 if len(violations) == 0 else max(0, 1 - len(violations)/10)
        
    elif task_type == "design":
        # Check that schema supports all required queries
        score = 0
        for query in problem["required_queries"]:
            if can_express_query(answer["schema"], query):
                score += 1
        
        # Check referential integrity constraints
        for fk in answer.get("foreign_keys", []):
            if not valid_foreign_key(answer["schema"], fk):
                return 0.0
        
        return score / len(problem["required_queries"])
    
    elif task_type == "identify_fds":
        pred_fds = set(tuple(fd) for fd in answer["fds"])
        gold_fds = set(tuple(fd) for fd in problem["gold_fds"])
        return set_f1(pred_fds, gold_fds)

def check_bcnf(schema: dict, fds: list) -> list:
    """Check if schema is in BCNF. Return violations."""
    violations = []
    for table_name, columns in schema.items():
        keys = compute_candidate_keys(columns, fds)
        for fd in fds:
            lhs, rhs = fd
            if set(lhs).issubset(set(columns)) and set(rhs).issubset(set(columns)):
                if not any(set(lhs).issuperset(key) for key in keys):
                    violations.append((table_name, fd))
    return violations
```

## Dataset Sources
- **Database textbook exercises**: Elmasri & Navathe, Ramakrishnan & Gehrke.
- **SQL competitions**: Schema design challenges.
- **Public database schemas**: MySQL/PostgreSQL sample databases (Sakila, Northwind, Chinook).
- **Database design courses**: CMU 15-445, Stanford CS145.
- **Procedural generation**: Generate entity-relationship specs, compute normal forms.

## Task Format
- **Input**: "A university needs to track students (ID, name), courses (code, title, credits), and enrollments (student, course, semester, grade). Design a 3NF schema."
- **Output**: Schema definition (tables, columns, keys, FKs)

## Difficulty Curriculum
- Level 1: Single-table normalization (1NF → 2NF)
- Level 3: Multi-table schema design, basic normalization
- Level 5: BCNF decomposition, lossless join property
- Level 7: Complex schemas with temporal data, inheritance
- Level 9: Distributed database design, sharding strategies

## Connections
- [[sql-generation]] — querying the designed database
- [[database-migrations]] — evolving schemas
- [[structured-output-generation]] — schema as structured output
