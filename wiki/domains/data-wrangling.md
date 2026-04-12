---
domain: Data Wrangling
category: Code & Software
verification_type: execution-based (compare output DataFrame/table to reference)
dataset_scale: 1K–20K transformation tasks
difficulty_range: single-column operations → complex multi-step reshape/merge/aggregate pipelines
modality: text-to-code (pandas, R, SQL-like transformations)
status: growing
---

## Overview

Data wrangling tasks the model with writing code (typically pandas, R/dplyr, or PySpark) that transforms an input dataset into a specified output format. Verification is execution-based: run the generated transformation code on the input data and compare the resulting DataFrame to a reference output, cell by cell.

This domain is a strong RLVR candidate because (a) DataFrame comparison is deterministic and fast, (b) data scientists write these transformations daily — the practical value is clear, (c) the task naturally admits partial credit (fraction of rows/columns correct), and (d) existing benchmarks provide curated problems with input-output pairs.

## Verification Mechanism

```
def verify_data_wrangling(generated_code, input_data, expected_output,
                          tolerance=1e-6):
    # 1. Execute generated code with input data
    try:
        exec_env = {"pd": pandas, "np": numpy, "df": input_data.copy()}
        exec(generated_code, exec_env)
        result_df = exec_env.get("result", exec_env.get("output", None))
    except Exception as e:
        return 0.0  # runtime error

    if result_df is None:
        return 0.0  # no output produced

    # 2. Compare DataFrames
    # Check shape
    if result_df.shape != expected_output.shape:
        return 0.0  # wrong dimensions

    # Check column names (order-sensitive or not, depending on task)
    if set(result_df.columns) != set(expected_output.columns):
        return 0.0
    # Reorder columns to match expected
    result_df = result_df[expected_output.columns]

    # 3. Cell-by-cell comparison with type awareness
    matches = 0
    total = result_df.shape[0] * result_df.shape[1]

    for col in expected_output.columns:
        for idx in range(len(expected_output)):
            expected_val = expected_output[col].iloc[idx]
            actual_val = result_df[col].iloc[idx]

            if pd.isna(expected_val) and pd.isna(actual_val):
                matches += 1
            elif isinstance(expected_val, float):
                if abs(expected_val - actual_val) < tolerance:
                    matches += 1
            elif expected_val == actual_val:
                matches += 1

    accuracy = matches / total

    # 4. Reward: strict (all match) or partial
    strict_reward = 1.0 if accuracy == 1.0 else 0.0
    return strict_reward  # or accuracy for partial credit
```

Efficient comparison using pandas built-ins:

```
def fast_verify(result_df, expected_df, tolerance=1e-6):
    """Optimized comparison using pandas.testing."""
    try:
        pd.testing.assert_frame_equal(
            result_df.reset_index(drop=True),
            expected_df.reset_index(drop=True),
            check_dtype=False,    # allow int vs float if values match
            check_exact=False,
            atol=tolerance,
            check_names=True,
            check_like=True       # ignore column/row order
        )
        return 1.0
    except AssertionError:
        return 0.0
```

Key considerations:

- **Row ordering**: Unless the task specifies a sort order, row ordering should be ignored. Sort both DataFrames by all columns before comparison, or use set-based comparison.
- **Type flexibility**: `int` vs. `float` for whole numbers, `str` vs. `object` — minor type differences that don't affect values should be tolerated.
- **NaN handling**: NaN != NaN in Python, so explicit NaN-matching logic is needed.
- **Large DataFrames**: For performance tasks with large data, execution time limits prevent runaway computations.
- **Side effects**: The generated code should not modify the input DataFrame. Pass a `.copy()`.

## Dataset Sources

| Dataset | Size | Focus | Notes |
|---------|------|-------|-------|
| **DS-1000** | 1,000 problems | 7 Python data-science libraries | pandas, numpy, scipy, sklearn, matplotlib, pytorch, tensorflow |
| **Arcade** | 1,082 tasks | pandas transformations | Multi-step data manipulation with natural-language descriptions |
| **Juice** | 1,981 tasks | pandas | Mined from Jupyter notebooks on GitHub |
| **NumpyEval** | 101 problems | numpy | Targeted numpy operations |
| **PandasEval** | 101 problems | pandas | Targeted pandas operations |
| **RTFM** | 271 tasks | pandas | Read-the-docs style tasks requiring API knowledge |
| **Spider for DataFrames** | Adaptable | pandas/SQL dual | Spider NL-to-SQL problems re-cast as NL-to-pandas |
| **Kaggle notebooks** | 100K+ | Mixed | Real data-science notebooks; mine input/output pairs |
| **CoNaLa** | 2,879 pairs | Python one-liners | NL → Python, many involve data manipulation |

**Synthetic data generation**:
1. Generate random DataFrames with various data types.
2. Apply random sequences of pandas operations (filter, groupby, merge, pivot, etc.).
3. Record the input DataFrame, the operation sequence (as NL description), and the output DataFrame.
4. Scale to 100K+ tasks automatically.

## Task Format

**Input prompt**:
```
Given a DataFrame `df` with columns: ['name', 'department',
'salary', 'hire_date']

Task: Calculate the average salary per department, but only
include employees hired after 2020-01-01. Sort by average
salary descending. The result should have columns
['department', 'avg_salary'].

Store the result in a variable called `result`.
```

**Input data** (provided as CSV or DataFrame):
```
name,department,salary,hire_date
Alice,Engineering,120000,2021-03-15
Bob,Engineering,110000,2019-06-01
Carol,Marketing,95000,2022-01-10
Dave,Marketing,88000,2023-04-20
Eve,Engineering,130000,2021-11-01
```

**Expected output**:
```
department,avg_salary
Engineering,125000.0
Marketing,91500.0
```

**Verification**: Execute the code, compare `result` DataFrame to expected.

## Difficulty Curriculum

| Level | Operations | Example |
|-------|-----------|---------|
| 1 | Single operation (filter, select, sort) | Filter rows where salary > 100K |
| 2 | Two operations (group + aggregate) | Average salary by department |
| 3 | Multi-step (filter + group + sort) | Top 3 departments by headcount post-2020 |
| 4 | Reshaping (pivot, melt, stack) | Pivot monthly sales by region |
| 5 | Multi-DataFrame (merge, concat, join) | Join employees with departments and compute metrics |
| 6 | Complex pipeline | Window functions, custom aggregations, multi-level groupby with reshaping |

## Limitations & Risks

- **Multiple valid implementations**: `df.groupby('dept')['salary'].mean()` and `df.pivot_table(values='salary', index='dept', aggfunc='mean')` produce identical results. Output-based comparison handles this.
- **Floating-point precision**: Aggregations can produce slightly different floats depending on operation order. Tolerance-based comparison is essential.
- **Library version sensitivity**: pandas API changes between versions (e.g., `append` deprecated in pandas 2.0). Pin library versions in the sandbox.
- **Performance vs. correctness**: A correct but extremely slow solution (e.g., row-by-row iteration instead of vectorized operations) should be penalized only if the task explicitly requires efficiency. Otherwise, correctness is sufficient.
- **Real-world messiness**: Benchmark data is clean, but real-world data has missing values, inconsistent types, encoding issues. Tasks should include messy-data variants.
- **Visualization tasks**: DS-1000 includes matplotlib tasks where "correct output" is a plot. Verifying plot correctness is significantly harder than DataFrame comparison. These tasks may need separate treatment.

## Connections

- **SQL Generation** is the declarative counterpart: both transform tabular data from natural-language descriptions, verified by output comparison.
- **Code Generation** is the superset: data wrangling is code generation specialized to data transformation libraries.
- Closely related to **Regex Synthesis** for string-column transformations.
- **Web Scraping** produces raw data that needs wrangling as the next step.
- **Code Optimization** applies: optimizing slow pandas pipelines (vectorization, avoiding `iterrows()`).
