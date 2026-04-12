---
domain: Database Migrations
category: Code & Software
verification_type: execution-based (apply migration + run validation queries + schema comparison)
dataset_scale: 500–10K migration tasks
difficulty_range: add column → cross-database schema redesign with data transformation
modality: text-to-SQL/DDL (migration scripts)
status: early-stage
---

## Overview

Database migration generation tasks the model with producing schema migration scripts (DDL statements, migration framework code) that transform a database schema from one state to another while preserving existing data integrity. Verification is execution-based: apply the migration to a test database, then validate that the schema matches the target state and that existing data was correctly preserved or transformed.

This domain is suitable for RLVR because (a) migrations can be applied to test databases and verified automatically, (b) schema comparison is deterministic, and (c) the task is practically valuable — migrations are a common source of production incidents when done incorrectly.

## Verification Mechanism

```
def verify_migration(migration_script, source_schema, target_schema,
                     test_data, validation_queries):
    # 1. Set up test database with source schema and test data
    db = create_test_database()
    apply_schema(db, source_schema)
    load_data(db, test_data)

    # Record pre-migration data for preservation checks
    pre_migration_data = {}
    for table in get_tables(db):
        pre_migration_data[table] = query(db, f"SELECT * FROM {table}")

    # 2. Apply the migration
    try:
        execute_migration(db, migration_script, timeout=120)
    except MigrationError as e:
        return 0.0  # migration failed to apply

    # 3. Verify target schema matches expected
    actual_schema = introspect_schema(db)
    schema_match = compare_schemas(actual_schema, target_schema)

    if not schema_match.tables_match:
        return 0.0  # wrong tables
    if not schema_match.columns_match:
        return 0.0  # wrong columns / types
    if not schema_match.constraints_match:
        return 0.0  # missing or wrong constraints / indexes

    # 4. Verify data preservation / transformation
    data_checks_passed = True
    for vq in validation_queries:
        try:
            result = query(db, vq.query)
            if not matches_expected(result, vq.expected):
                data_checks_passed = False
                break
        except QueryError:
            data_checks_passed = False
            break

    if not data_checks_passed:
        return 0.0

    # 5. Verify data row counts (no data loss)
    for table, pre_data in pre_migration_data.items():
        if table in get_tables(db):  # table still exists
            post_count = query(db, f"SELECT COUNT(*) FROM {table}")[0][0]
            pre_count = len(pre_data)
            if post_count < pre_count:
                return 0.0  # data loss detected

    # 6. Check reversibility (optional)
    if hasattr(migration_script, 'down'):
        try:
            execute_migration(db, migration_script.down)
            reverted_schema = introspect_schema(db)
            if compare_schemas(reverted_schema, source_schema).exact_match:
                reversibility_bonus = 0.1
            else:
                reversibility_bonus = 0.0
        except:
            reversibility_bonus = 0.0
    else:
        reversibility_bonus = 0.0

    return 1.0 + reversibility_bonus  # base reward + bonus for reversibility


def compare_schemas(actual, expected):
    """Detailed schema comparison."""
    result = SchemaComparison()

    # Compare tables
    actual_tables = set(actual.tables.keys())
    expected_tables = set(expected.tables.keys())
    result.tables_match = (actual_tables == expected_tables)

    # Compare columns per table
    result.columns_match = True
    for table in expected_tables:
        if table not in actual.tables:
            result.columns_match = False
            continue
        for col in expected.tables[table].columns:
            actual_col = actual.tables[table].get_column(col.name)
            if actual_col is None:
                result.columns_match = False
            elif not types_compatible(actual_col.type, col.type):
                result.columns_match = False
            elif actual_col.nullable != col.nullable:
                result.columns_match = False

    # Compare constraints (PK, FK, UNIQUE, CHECK)
    result.constraints_match = compare_constraints(actual, expected)

    # Compare indexes
    result.indexes_match = compare_indexes(actual, expected)

    return result
```

Key considerations:

- **Data preservation**: The migration must not lose data. Row-count checks and value-preservation queries are essential.
- **Data transformation**: Some migrations require transforming data (e.g., splitting a full_name column into first_name and last_name). Validation queries must check that transformations are correct.
- **Idempotency**: Running the migration twice should not cause errors (or should be guarded by `IF NOT EXISTS` checks).
- **Transaction safety**: Migrations should be wrapped in transactions where possible. The verification environment should check for partial application states.
- **Database dialect**: Migrations differ across PostgreSQL, MySQL, SQLite, etc. The task must specify the target database.

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **Rails migration history** | 100K+ migrations | GitHub Ruby/Rails repos | ActiveRecord migrations are well-structured |
| **Django migration history** | 50K+ migrations | GitHub Python/Django repos | Django auto-generates migration files |
| **Alembic migrations** | 20K+ | GitHub Python/SQLAlchemy repos | Python ORM migration framework |
| **Flyway/Liquibase scripts** | 10K+ | GitHub Java repos | Java ecosystem migration tools |
| **Schema evolution datasets** | Academic | Research | Tracked schema changes over project lifetimes |
| **Prisma migrations** | Growing | GitHub TypeScript/Node repos | Modern ORM with migration support |
| **SchemaPile** | 200K+ schemas | GitHub | Large collection of database schemas (pairs can be derived) |

**Synthetic data generation**:
1. Generate random source schemas (3–20 tables with relationships).
2. Apply random schema transformations (add column, rename table, add index, change type, split table, merge tables).
3. The source schema + transformation description = task; the target schema + migration script = answer.
4. Populate test databases with random data and generate validation queries.

## Task Format

**Input prompt**:
```
Current database schema (PostgreSQL):

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR(200) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    total DECIMAL(10,2),
    status VARCHAR(20)
);

Write a migration to:
1. Split users.full_name into first_name and last_name columns
2. Add an 'address' table with id, user_id (FK), street,
   city, state, zip
3. Add an index on orders(user_id, status)

Preserve all existing data. For name splitting, use the first
space as the delimiter.
```

**Expected output**: SQL migration script with ALTER TABLE, CREATE TABLE, UPDATE, CREATE INDEX statements.

**Verification**: Apply migration, check schema matches target, verify name splitting was done correctly, verify no data loss.

## Difficulty Curriculum

| Level | Migration Type | Example |
|-------|---------------|---------|
| 1 | Add column (nullable) | `ALTER TABLE users ADD COLUMN phone VARCHAR(20)` |
| 2 | Add table with foreign keys | New `addresses` table referencing `users` |
| 3 | Rename / change column type | Rename `full_name` to `display_name`, change VARCHAR length |
| 4 | Data transformation | Split column, merge tables, compute derived values |
| 5 | Schema normalization | Decompose a denormalized table into 3NF |
| 6 | Complex restructuring | Merge two databases, change primary keys, update all FKs |

## Limitations & Risks

- **Data loss**: A bad migration can irreversibly destroy data. Verification must explicitly check for data preservation, but test data may not cover all edge cases.
- **Performance on large tables**: A migration that works on a test database with 100 rows might lock a production table with 100M rows for hours. Verification can't easily test for production-scale performance.
- **Database-specific syntax**: DDL syntax varies significantly across PostgreSQL, MySQL, SQLite, SQL Server. Tasks must target a specific database.
- **Down migrations**: Not all migrations are reversible (e.g., dropping a column loses data). Verification of down migrations is valuable but not always possible.
- **Concurrency**: Migrations that take long-running locks can cause downtime. Online DDL techniques (e.g., `ALTER TABLE ... ALGORITHM=INPLACE` in MySQL) are important in practice but hard to verify in test environments.
- **ORM abstraction**: Real-world migrations often use ORM frameworks (Alembic, ActiveRecord) rather than raw SQL. The task format must specify whether raw SQL or ORM-style code is expected.

## Connections

- **SQL Generation** is the query-level counterpart: writing SELECT queries vs. writing DDL/DML migration scripts.
- **Code Repair** applies: fixing a broken migration that partially applied.
- **Infrastructure-as-Code** parallels: both manage declarative state changes (schema state vs. infrastructure state).
- **Data Wrangling** overlaps for the data-transformation aspect of migrations (ETL-style operations).
- **Build Configuration** shares the "change system state declaratively and verify" pattern.
