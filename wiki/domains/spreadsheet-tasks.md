---
domain: spreadsheet-tasks
category: agent/productivity
verification_type: diff | constraint
dataset_scale: >1K tasks (benchmarks); generatable from templates
difficulty_range: easy/medium/hard
modality: text | multimodal
status: strong_hypothesis
---

# Spreadsheet Tasks

## Overview

Spreadsheet tasks require an agent to manipulate spreadsheets: writing formulas, creating pivot tables, formatting cells, building charts, cleaning data, and performing complex data analysis. This is a high-value RLVR domain because spreadsheets are ubiquitous in business and data work, verification is concrete (compare the output spreadsheet state to a reference), and tasks span a wide difficulty range from simple SUM formulas to complex multi-sheet financial models. The domain tests procedural knowledge, mathematical reasoning, and the ability to translate natural-language requests into precise operations.

## Verification Mechanism

1. **Cell value comparison** (diff-based): Compare specific cell values in the output spreadsheet to expected values. Handle numeric tolerance for floating-point results. Primary verification mechanism. Implementation: read cells via openpyxl/xlsxwriter and compare.
2. **Formula comparison** (exact match): Check that specific cells contain the correct formulas (not just the correct values). Important for tasks where the formula structure matters (e.g., "use VLOOKUP, not INDEX/MATCH"). Normalize formula strings for comparison.
3. **Structural comparison** (diff-based): Compare spreadsheet structure: number of sheets, column headers, row counts, data types. For pivot table tasks, verify the pivot structure (row/column fields, value aggregations).
4. **Chart verification** (constraint-based): Verify that a chart exists with the correct type (bar, line, pie), data range, title, and axis labels. Queried through spreadsheet API.
5. **Formatting verification** (constraint-based): Check cell formatting: bold, color, borders, number format, conditional formatting rules. Queried through spreadsheet API.
6. **Full sheet diff** (diff-based): Export both reference and output spreadsheets to structured format (CSV, JSON), then diff cell-by-cell. Most comprehensive but may be too strict for tasks with multiple valid approaches.
7. **Execution-based verification**: For tasks involving macros or VBA, run the macro and check the resulting state.

## Dataset Sources

- **SheetCopilot**: https://github.com/AlibabaResearch/DAMO-ConvAI/tree/main/sheetcopilot — 221 spreadsheet tasks across 6 categories (data manipulation, formatting, charts, formulas, pivot tables, macros). Each task has a setup spreadsheet, instruction, and reference output. The primary benchmark.
- **Spreadsheet-Bench**: Benchmark for evaluating spreadsheet manipulation capabilities.
- **NL2Formula**: Natural language to spreadsheet formula translation. Formulas can be verified by execution.
- **TURL (Table Understanding and Reasoning with Language)**: Related table manipulation tasks.
- **Template-based generation**: Spreadsheet tasks are highly amenable to procedural generation:
  - Generate random data tables.
  - Apply random transformations (sort, filter, aggregate).
  - Save the result as the reference output.
  - Generate natural language instructions for the transformation.
  - Unlimited tasks can be created this way.
- **Excel practice exercises**: Thousands of practice exercises from Excel training websites (Excel Easy, Chandoo, ExcelJet). Can be scraped and formalized with verification.
- **Kaggle datasets**: Any tabular dataset can be turned into spreadsheet tasks (clean this data, compute summary statistics, create a pivot table).

## Task Format

**Formula writing**:
- Input: Spreadsheet state + instruction (e.g., "In cell E2, write a formula to calculate the total revenue (price * quantity) for each row").
- Output: Formula string (e.g., "=C2*D2") or action sequence to input it.
- Verification: Formula in cell matches expected, OR computed value matches expected.

**Data manipulation**:
- Input: Spreadsheet + instruction (e.g., "Sort the data by date descending, then remove all rows where status is 'cancelled'").
- Output: Modified spreadsheet (or action sequence).
- Verification: Resulting spreadsheet matches reference (cell-by-cell diff).

**Pivot table creation**:
- Input: Data table + instruction (e.g., "Create a pivot table showing total sales by region and product category").
- Output: Pivot table in a new sheet.
- Verification: Check pivot table structure (rows, columns, values) and aggregated numbers.

**Chart creation**:
- Input: Data + instruction (e.g., "Create a bar chart showing monthly revenue with title 'Q3 Revenue'").
- Output: Chart added to spreadsheet.
- Verification: Chart exists with correct type, data range, title.

**Multi-step workflow**:
- Input: Raw data + complex instruction (e.g., "Clean the dataset: remove duplicates, standardize date format, compute a 'revenue' column, create a summary table by month, and format it professionally").
- Output: Completed spreadsheet.
- Verification: Multiple checks (column exists, values correct, formatting applied).

## Difficulty Curriculum

1. **Basic formulas** (easy): SUM, AVERAGE, COUNT, MIN, MAX on a single column.
2. **Cell references** (easy): Formulas referencing other cells, basic arithmetic.
3. **Conditional functions** (easy-medium): IF, COUNTIF, SUMIF. Simple conditions.
4. **Lookup functions** (medium): VLOOKUP, HLOOKUP, INDEX/MATCH. Cross-referencing data.
5. **Data cleaning** (medium): Remove duplicates, handle missing values, standardize formats.
6. **Pivot tables** (medium-hard): Create and configure pivot tables from data.
7. **Advanced formulas** (hard): Nested functions, array formulas, QUERY. Multi-step calculations.
8. **Charts with formatting** (hard): Create publication-quality charts with proper formatting.
9. **Financial modeling** (very hard): Multi-sheet financial models with projections, scenarios, and cross-references.
10. **VBA / macros** (very hard): Write automation scripts for complex repetitive tasks.

## Limitations & Risks

- **Multiple valid approaches**: Many spreadsheet tasks can be accomplished in multiple ways (e.g., VLOOKUP vs INDEX/MATCH, manual vs formula-based). Strict formula comparison may reject valid alternatives. Value-based comparison is more robust but misses formula quality.
- **Spreadsheet software fragility**: Google Sheets, Excel (desktop), Excel (online), and LibreOffice Calc have different formula syntaxes, features, and behaviors. Tasks and verification must target a specific platform.
- **Action-based vs state-based tasks**: Some tasks naturally produce measurable state (cell values), others are about process (formatting, chart creation) that is harder to verify.
- **Limited benchmark scale**: SheetCopilot has only 221 tasks. Template-based generation can scale quantity but may lack the diversity and nuance of real-world spreadsheet tasks.
- **Context window for large spreadsheets**: Real-world spreadsheets can have thousands of rows and dozens of columns. Fitting this into an LLM context window is challenging.
- **Verification of "correctness" vs "quality"**: A correct formula is not the same as a well-designed spreadsheet. RLVR can verify correctness but not elegance, maintainability, or user-friendliness.

## Connections

- [[computer-use]] — Spreadsheet tasks are a core computer use scenario
- [[web-navigation]] — Web-based spreadsheet interaction (Google Sheets) is a form of web navigation
- [[competitive-programming-interactive]] — Both involve precise computational implementations with automated verification
- [[document-ocr-extraction]] — Extracting data from documents often feeds into spreadsheet tasks
- [[file-system-tasks]] — Both involve producing specific digital artifacts verified by state comparison
