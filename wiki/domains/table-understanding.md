---
domain: table-understanding
category: information-processing
verified: true
verification_type: exact
scalability: high
maturity: high
tags: [table-qa, table-reasoning, wikitablequestions, structured-data]
---

# Table Understanding

## Overview

Table understanding encompasses tasks that require reasoning over tabular data: answering questions about tables, performing table-based fact verification, generating descriptions from tables, and table-to-text generation. The model must interpret rows, columns, headers, cell values, and relationships between cells.

This domain has clean verification (exact match on extracted answers) and large-scale data from Wikipedia tables and spreadsheets. It tests structured reasoning skills that complement unstructured text comprehension.

## Verification Mechanism

**Verification type: Exact**

For table QA, the verification function compares the model's answer against gold answer(s):

```python
def verify(prediction: str, gold_answers: list[str]) -> int:
    pred_norm = normalize(prediction)
    for gold in gold_answers:
        if pred_norm == normalize(gold):
            return 1
    return 0
```

**Normalization for table answers**:
- Lowercase and strip whitespace
- Remove articles and punctuation
- For numerical answers: parse to float and compare with tolerance (e.g., |pred - gold| < 0.01)
- For date answers: parse to canonical form
- For list answers: compare as sets (order-insensitive)

For table-based fact verification (TabFact), verification is label comparison (ENTAILED/REFUTED), same as text classification.

For tasks requiring SQL or program generation as intermediate steps, the generated program can be executed against the table to produce an answer, which is then verified against gold.

## Dataset Sources

| Dataset | Task | Size | Source | Notes |
|---------|------|------|--------|-------|
| WikiTableQuestions (WTQ) | Table QA | 22K questions | Wikipedia tables | Compositional questions requiring multi-step reasoning |
| SQA (Sequential QA) | Conversational table QA | 6K sequences | Wikipedia tables | Follow-up questions in context |
| HybridQA | Hybrid table + text QA | 70K questions | Wikipedia | Requires reasoning over both table and linked passages |
| TabFact | Table fact verification | 118K claims | Wikipedia tables | Binary entailment classification |
| ToTTo | Table-to-text | 120K examples | Wikipedia tables | Generate description from highlighted cells |
| TAPAS datasets | Various table tasks | Varies | Wikipedia tables | Google's table parsing benchmarks |
| Spider (table component) | Text-to-SQL | 10K questions | 200 databases | Complex cross-table reasoning |
| FeTaQA | Free-form table QA | 10K questions | Wikipedia tables | Long-form answers |
| SciGen | Scientific table description | 1.3K tables | Scientific papers | Describe patterns in data tables |
| FinQA | Financial table QA | 8.3K questions | Financial reports | Numerical reasoning over financial data |

**Scalability**: Wikipedia contains over 3 million tables. Any table with structured headers and content can generate QA pairs through template-based question generation, SQL query reversal, or claim generation. Financial reports, government data, and spreadsheets provide additional scale.

## Task Format

**Input**: A table (serialized as markdown, CSV, or HTML) and a question or claim.

**Output**: An answer (entity, number, date, yes/no) or a label.

**Prompt template example**:
```
Answer the question based on the table below.

| Country | Capital | Population |
|---------|---------|------------|
| France  | Paris   | 67M        |
| Germany | Berlin  | 83M        |
| Italy   | Rome    | 60M        |

Question: Which country has the largest population?

Answer:
```

For multi-step:
```
Using the table below, answer the question. Show your reasoning.

{table}

Question: {question}

Answer:
```

## Difficulty Curriculum

1. **Level 1 — Cell lookup**: "What is the capital of France?" Direct lookup in a single cell.
2. **Level 2 — Row/column filtering**: "Which countries have population over 70M?" Filter rows by a condition.
3. **Level 3 — Aggregation**: "What is the average population?" Requires computation over a column.
4. **Level 4 — Comparison and sorting**: "Which country has the second-highest GDP?" Requires sorting and ordinal selection.
5. **Level 5 — Multi-column reasoning**: "Which country has the highest GDP per capita?" Requires combining values from multiple columns.
6. **Level 6 — Multi-step compositional**: "What is the total population of countries whose capital starts with B?" Chained operations.
7. **Level 7 — Hybrid table + text reasoning**: HybridQA-style questions requiring information from both the table and linked text passages.

## Limitations & Risks

- **Table serialization**: LLMs process tables as linearized text (markdown, CSV). This loses spatial relationships. Wide tables may exceed context windows. The serialization format affects model performance.
- **Numerical reasoning weakness**: LLMs are notoriously weak at precise arithmetic. Table QA often requires exact numerical computation (sums, averages, percentages), which LLMs perform unreliably.
- **Large tables**: Tables with hundreds of rows stress context windows and attention. The model may miss relevant rows in large tables.
- **Ambiguous questions**: "What is the highest value?" — highest in which column? Table QA datasets sometimes have ambiguous questions.
- **Answer format diversity**: Answers can be numbers, strings, dates, or lists. Normalization must handle all types, which adds complexity to the verifier.

## Connections

- **[fact-verification](fact-verification.md)**: TabFact combines table understanding with fact verification.
- **[document-parsing](document-parsing.md)**: Extracting tables from documents is a prerequisite for table QA.
- **[semantic-parsing](semantic-parsing.md)**: Table QA is often solved by generating SQL (semantic parsing) and executing it.
- **[question-answering-extractive](question-answering-extractive.md)**: Table QA is extractive QA over structured data.
- **[structured-output-generation](structured-output-generation.md)**: Producing structured outputs from tables (JSON, SQL) is a form of structured output generation.
- **[information-extraction](information-extraction.md)**: Extracting information from tables is a form of IE.
