---
domain: Data Science EDA & Statistical Analysis
category: data-science
verification_type: execution
dataset_scale: 50K+ (Kaggle, UCI, government data)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Data Science EDA & Statistical Analysis

## Overview
Given a dataset and analytical questions, produce code that performs correct statistical analysis and returns the right numerical answers. Questions like: "What is the correlation between age and income?", "Is there a statistically significant difference between groups A and B?", "What is the 95th percentile of response times?"

## Verification Mechanism
```python
def verify(dataset: pd.DataFrame, question: str, code: str, 
           expected_answer: Any, tolerance: float = 0.001) -> float:
    # Execute the analysis code
    try:
        result = execute_in_sandbox(code, {"df": dataset}, timeout=60)
    except Exception:
        return 0.0
    
    # Type-specific comparison
    if isinstance(expected_answer, (int, float)):
        return 1.0 if abs(result - expected_answer) <= tolerance * abs(expected_answer) else 0.0
    elif isinstance(expected_answer, bool):
        return 1.0 if result == expected_answer else 0.0
    elif isinstance(expected_answer, str):
        return 1.0 if result.strip().lower() == expected_answer.strip().lower() else 0.0
    elif isinstance(expected_answer, pd.DataFrame):
        return 1.0 if dataframes_match(result, expected_answer, tolerance) else 0.0
    
    return 0.0
```

## Dataset Sources
- **Kaggle**: 50K+ public datasets with notebooks (reference analyses).
- **UCI Machine Learning Repository**: 600+ datasets with known analyses.
- **FiveThirtyEight data**: Journalistic datasets with published analyses.
- **US Government Open Data (data.gov)**: Thousands of datasets.
- **World Bank Open Data**: Global development indicators.
- **Google BigQuery public datasets**: Large-scale datasets.
- **DS-1000**: Data science coding benchmark.
- **Arcade**: Data analysis benchmark.

## Task Format
- **Input**: DataFrame + "Compute the Pearson correlation between columns 'temperature' and 'ice_cream_sales'. Is it statistically significant at p < 0.05?"
- **Output**: Python code that computes the answer (e.g., "r = 0.87, p = 0.001, significant: True")

## Difficulty Curriculum
- Level 1: Descriptive statistics (mean, median, mode, percentiles)
- Level 3: Correlation, basic hypothesis tests (t-test, chi-squared)
- Level 5: Regression analysis, ANOVA, effect sizes
- Level 7: Time series analysis, multivariate analysis
- Level 9: Causal inference, Bayesian analysis, complex ML pipeline evaluation

## Limitations & Risks
- Numerical precision: floating point comparison needs appropriate tolerance.
- Statistical questions sometimes have multiple valid approaches (different tests). Focus on the numerical answer rather than the specific method.
- Need to pre-compute expected answers for each dataset-question pair.

## Connections
- [[data-wrangling]] — data preparation
- [[data-visualization]] — visual analysis
- [[probability-statistics]] — theoretical foundations
- [[sql-generation]] — data querying
