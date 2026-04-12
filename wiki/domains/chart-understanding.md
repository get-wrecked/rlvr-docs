---
domain: chart-understanding
category: vision/chart
verification_type: exact_match
dataset_scale: >200K question-answer pairs
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Chart Understanding

## Overview

Chart understanding tasks require an agent to answer questions about data visualizations: bar charts, line graphs, pie charts, scatter plots, heatmaps, area charts, and more. This is a high-value RLVR domain because (1) charts are ubiquitous in business, science, and education, (2) questions about charts typically have unambiguous factual answers, and (3) the domain tests a distinctive combination of visual parsing (reading axes, legends, data points) and quantitative reasoning (computing differences, trends, comparisons). Unlike general VQA, chart questions are almost always verifiable because they ask about concrete data.

## Verification Mechanism

1. **Exact answer match** (exact match): Most chart QA answers are short strings — numbers, labels, or yes/no. Verification: normalized string comparison. Handle number formats (1000 vs 1,000 vs 1K), percentage formats (50% vs 0.5), and label variations.
2. **Numeric tolerance match** (exact match with tolerance): For questions requiring reading values from axes (inherently approximate), accept answers within a percentage tolerance. E.g., if the correct value is 47 and the agent says 46 or 48, accept within 5% tolerance.
3. **Multiple choice** (exact match): Some benchmarks (FigureQA) use binary yes/no or multiple choice format. Completely unambiguous verification.
4. **Data table extraction** (diff-based): For tasks requiring full data extraction from charts, compare extracted data table to ground truth. Cell-by-cell numeric comparison with tolerance.

## Dataset Sources

- **ChartQA**: https://github.com/vis-nlp/ChartQA — 32K questions on 21K chart images. Mix of human-written and machine-generated questions. Covers bar, line, pie charts. Standard benchmark. Two subsets: "human" (harder, requires reasoning) and "augmented" (simpler, template-generated).
- **PlotQA**: https://github.com/NiteshMethworku/PlotQA — 8.1M QA pairs on 224K synthetic chart images. Programmatically generated from real-world data. Three question types: structural, data retrieval, reasoning. Massive scale.
- **FigureQA**: https://www.microsoft.com/en-us/research/project/figureqa-dataset/ — 1.3M binary (yes/no) questions on 100K synthetic figures. Focuses on structural understanding.
- **DVQA**: https://github.com/kushalkafle/DVQA_dataset — 3.5M QA pairs on 300K synthetic bar charts. Tests OCR + reasoning.
- **ChartBench**: Large-scale chart understanding benchmark with diverse chart types.
- **Chart-to-Text**: https://github.com/vis-nlp/Chart-to-Text — 44K chart summaries. Paired chart images with descriptive text.
- **UniChart**: https://github.com/vis-nlp/UniChart — Unified chart understanding benchmark across multiple tasks.
- **OpenCQA**: Open-ended chart QA requiring explanatory answers (harder to verify — partial RLVR applicability).
- **Statista / Our World in Data**: Real-world chart sources for generating new QA pairs. Charts with known underlying data tables.

## Task Format

**Factual chart QA**:
- Input: Chart image + question (e.g., "What was the highest value in 2020?" over a line graph).
- Output: Short answer (e.g., "4.5 million" or "4500000").
- Verification: Exact match after normalization, or numeric tolerance.

**Structural chart QA**:
- Input: Chart image + structural question (e.g., "How many bars are in the chart?", "What color represents 'Sales'?").
- Output: Answer (e.g., "5", "blue").
- Verification: Exact match.

**Comparison / reasoning chart QA**:
- Input: Chart image + reasoning question (e.g., "In which year did revenue first exceed expenses?").
- Output: Answer (e.g., "2018").
- Verification: Exact match.

**Binary (yes/no) chart QA**:
- Input: Chart image + binary question (e.g., "Is the red line always above the blue line?").
- Output: "yes" or "no".
- Verification: Exact match. FigureQA format.

**Data table extraction**:
- Input: Chart image.
- Output: Underlying data as a table (JSON/CSV).
- Verification: Cell-by-cell comparison with numeric tolerance.

## Difficulty Curriculum

1. **Title/label reading** (trivial): "What is the title of this chart?" Pure OCR.
2. **Single value lookup** (easy): "What is the value for category X?" Read one bar/point.
3. **Comparison questions** (easy-medium): "Which category has the highest value?" Compare visible bars/points.
4. **Trend questions** (medium): "Is the trend increasing or decreasing?" Requires understanding line direction.
5. **Multi-step computation** (medium-hard): "What is the difference between the highest and lowest values?" Requires reading + arithmetic.
6. **Complex reasoning** (hard): "What percentage of total sales does category X represent?" Requires reading multiple values + computation.
7. **Aggregation across categories** (hard): "What is the average value across all years?" Requires reading many values.
8. **Multi-chart reasoning** (very hard): Compare data across multiple charts. Rare in benchmarks but highly practical.
9. **Noisy/complex real-world charts** (very hard): Charts with multiple axes, overlapping elements, poor formatting, 3D effects.

## Limitations & Risks

- **Reading precision**: Humans disagree on exact values read from charts (is that bar at 47 or 48?). This introduces noise in both ground truth and evaluation.
- **Synthetic vs real charts**: PlotQA and FigureQA use synthetic charts that are cleaner and more uniform than real-world charts. Models trained on synthetic charts may struggle with messy real-world visualizations.
- **Chart diversity**: Most benchmarks focus on bar, line, and pie charts. Scatter plots, heatmaps, box plots, Sankey diagrams, and other visualization types are underrepresented.
- **OCR dependency**: Many chart questions require reading axis labels, legends, and data annotations. If the model's OCR is weak, it will fail regardless of reasoning ability.
- **Number format normalization**: Correctly matching "1.5M", "1,500,000", "1500000", and "1.5 million" is surprisingly tricky. Normalization functions must handle many edge cases.
- **Reasoning vs perception**: It is hard to disentangle whether errors are due to incorrect visual reading or incorrect reasoning. Both failure modes exist.

## Connections

- [[visual-question-answering]] — Chart QA is a specialized form of VQA with stronger verifiability
- [[math-word-problems-visual]] — Chart questions involving computation overlap with visual math
- [[document-ocr-extraction]] — Charts in documents require similar parsing capabilities
- [[spatial-reasoning]] — Understanding chart layout requires spatial understanding
