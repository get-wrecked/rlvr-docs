---
domain: Data Visualization Generation
category: data-science
verification_type: execution
dataset_scale: 100K+ (from Kaggle, public datasets)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Data Visualization Generation

## Overview
Given a dataset and a visualization request ("show the distribution of ages by gender"), generate code that produces the correct chart. Verification: execute the code, check the output image matches the specification structurally.

## Verification Mechanism
```python
def verify(data: pd.DataFrame, request: str, code: str, reference_spec: dict) -> float:
    # Execute the visualization code
    try:
        fig = execute_viz_code(code, data)
    except Exception:
        return 0.0
    
    score = 0.0
    checks = 0
    
    # Check chart type
    if "chart_type" in reference_spec:
        checks += 1
        detected_type = detect_chart_type(fig)
        if detected_type == reference_spec["chart_type"]:
            score += 1.0
    
    # Check axes data
    if "x_column" in reference_spec:
        checks += 1
        x_data = extract_x_data(fig)
        if data_matches(x_data, data[reference_spec["x_column"]]):
            score += 1.0
    
    if "y_column" in reference_spec:
        checks += 1
        y_data = extract_y_data(fig)
        if data_matches(y_data, data[reference_spec["y_column"]]):
            score += 1.0
    
    # Check required elements (title, legend, labels)
    for element in reference_spec.get("required_elements", []):
        checks += 1
        if has_element(fig, element):
            score += 1.0
    
    # Check data integrity (all data points present)
    checks += 1
    if verify_data_integrity(fig, data, reference_spec):
        score += 1.0
    
    return score / checks if checks > 0 else 0.0
```

Alternative simpler verification: extract the data from the rendered plot (using matplotlib's internal data structures) and compare to expected.

## Dataset Sources
- **Kaggle datasets**: 50K+ public datasets with associated visualizations.
- **matplotlib/seaborn gallery**: Example visualizations with code.
- **Plotly examples**: Interactive visualization examples.
- **Observable/D3 examples**: Web-based visualization examples.
- **VizML dataset**: 100K+ visualization-data pairs from Plotly.
- **Procedural generation**: Take any tabular dataset + random visualization specs.

## Task Format
- **Input**: CSV/DataFrame + "Create a grouped bar chart showing average salary by department and gender"
- **Output**: Python code (matplotlib/seaborn/plotly) that produces the chart

## Difficulty Curriculum
- Level 1: Basic bar/line/scatter plots
- Level 3: Grouped/stacked charts, histograms, box plots
- Level 5: Multi-panel figures, custom color schemes, annotations
- Level 7: Complex statistical visualizations, geographic maps
- Level 9: Interactive dashboards, animated visualizations

## Limitations & Risks
- Visual comparison is approximate. Focus on structural/data verification rather than aesthetic matching.
- matplotlib's internal data structures can be inspected programmatically, making verification more reliable than pixel comparison.

## Connections
- [[data-wrangling]] — data preparation for visualization
- [[chart-understanding]] — reverse direction (chart → data)
- [[code-generation]] — visualization code is code
