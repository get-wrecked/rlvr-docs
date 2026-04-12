---
domain: ML Pipeline / AutoML
category: machine-learning
verification_type: execution
dataset_scale: 50K+ (from Kaggle + OpenML)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# ML Pipeline / AutoML

## Overview
Given a dataset and prediction task, design and implement an ML pipeline (feature engineering, model selection, hyperparameter tuning, ensemble) that maximizes a specified metric. Verification: train on train split, evaluate on held-out test split. The metric is objectively computable.

## Verification Mechanism
```python
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score, f1_score, mean_squared_error

def verify(dataset: pd.DataFrame, target_col: str, metric: str,
           pipeline_code: str, test_data: pd.DataFrame) -> float:
    try:
        # Execute the pipeline code
        model = execute_pipeline(pipeline_code, dataset, target_col, timeout=300)
    except Exception:
        return 0.0
    
    # Evaluate on held-out test data
    X_test = test_data.drop(columns=[target_col])
    y_test = test_data[target_col]
    predictions = model.predict(X_test)
    
    if metric == "accuracy":
        score = accuracy_score(y_test, predictions)
    elif metric == "f1":
        score = f1_score(y_test, predictions, average="weighted")
    elif metric == "rmse":
        rmse = mean_squared_error(y_test, predictions, squared=False)
        # Normalize: lower is better, convert to 0-1
        baseline_rmse = y_test.std()
        score = max(0, 1 - rmse / baseline_rmse)
    elif metric == "auc":
        score = roc_auc_score(y_test, model.predict_proba(X_test)[:, 1])
    
    return score
```

## Dataset Sources
- **OpenML**: 20K+ datasets with known baselines.
- **Kaggle competitions**: Thousands of ML competitions with leaderboards.
- **AutoML benchmarks**: AutoMLBenchmark (AMLB).
- **Penn ML Benchmarks**: Curated classification datasets.
- **UCI ML Repository**: 600+ datasets.
- **Scikit-learn built-in datasets**: Standard benchmarks.

## Task Format
- **Input**: Dataset (CSV) + "Build a model to predict [target] maximizing [metric]. Training budget: 5 minutes."
- **Output**: Python code implementing the full pipeline

## Difficulty Curriculum
- Level 1: Clean tabular data, standard classification
- Level 3: Missing values, categorical features, feature engineering
- Level 5: Imbalanced classes, high-dimensional data, ensemble methods
- Level 7: Multi-objective optimization (accuracy + fairness + latency)
- Level 9: Novel architectures for unusual data types

## Limitations & Risks
- Overfitting to test set via information leakage. Must strictly separate train/test.
- Reproducibility requires pinning random seeds and library versions.
- Training time limits are important — unbounded training trivially wins.
- Metric gaming: optimize metric without genuine predictive power. Use proper cross-validation.

## Connections
- [[code-generation]] — generating ML code
- [[data-wrangling]] — data preprocessing
- [[data-science-eda]] — understanding the data first
- [[combinatorics-optimization]] — hyperparameter search
