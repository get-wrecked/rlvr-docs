---
domain: Spreadsheet Formula Generation
category: data-productivity
verification_type: execution
dataset_scale: 1M+ (from Excel/Sheets forums)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Spreadsheet Formula Generation

## Overview
Given a spreadsheet with data and a natural language description of desired computation, generate the correct Excel/Google Sheets formula. Verification: evaluate the formula against the data and compare to expected output.

## Verification Mechanism
```python
import openpyxl
from formulas import ExcelParser

def verify(spreadsheet_data: dict, cell_target: str, 
           nl_description: str, formula: str, expected_value: Any) -> float:
    # Parse and evaluate the formula
    try:
        parser = ExcelParser()
        result = parser.evaluate(formula, context=spreadsheet_data)
    except Exception:
        return 0.0
    
    # Compare to expected value
    if isinstance(expected_value, (int, float)):
        return 1.0 if abs(result - expected_value) < 1e-6 else 0.0
    else:
        return 1.0 if str(result) == str(expected_value) else 0.0
```

Also verify: formula syntax is valid (parseable by Excel engine).

## Dataset Sources
- **ENRON spreadsheet corpus**: 15K+ real-world spreadsheets.
- **Excel forums (MrExcel, ExcelJet)**: Millions of Q&A with formula solutions.
- **Google Sheets help forums**: Q&A with formula solutions.
- **SheetCopilot benchmark**: Spreadsheet task benchmark.
- **Spreadsheet Corpus (FUSE)**: Large corpus of spreadsheets.
- **Procedural generation**: Create random data tables + formula specifications.

## Task Format
- **Input**: Spreadsheet data (as table) + "Calculate the running total of column B, showing the cumulative sum in column C"
- **Output**: `=SUM(B$1:B1)` (to be dragged down)

## Difficulty Curriculum
- Level 1: Basic SUM, AVERAGE, COUNT, IF
- Level 3: VLOOKUP, INDEX/MATCH, nested IF
- Level 5: Array formulas, SUMPRODUCT, conditional formatting formulas
- Level 7: Dynamic arrays (FILTER, UNIQUE, SORT, LAMBDA)
- Level 9: Complex financial modeling formulas, recursive LAMBDA

## Limitations & Risks
- Excel formula evaluation engines vary slightly (Microsoft vs. LibreCalc vs. Google Sheets). Standardize on one.
- Some formulas are context-dependent (relative references change when dragged). Need to handle cell positioning.
- Real spreadsheets have messy data — training on clean data may not transfer perfectly.

## Connections
- [[sql-generation]] — both are "query" generation for tabular data
- [[data-wrangling]] — data transformations in different paradigms
- [[financial-calculation]] — financial formulas are a major spreadsheet use case
