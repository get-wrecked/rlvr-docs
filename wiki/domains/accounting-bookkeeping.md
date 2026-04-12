---
domain: Accounting & Bookkeeping
category: domain-expert
verification_type: execution
dataset_scale: 100K+ (from accounting problems + textbooks)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Accounting & Bookkeeping

## Overview
Perform accounting tasks with verifiable numerical outputs: journal entries, balance sheet preparation, depreciation calculations, tax computation, financial ratio analysis. Verification: fundamental accounting equation (Assets = Liabilities + Equity) must always hold, and numerical outputs must match.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: dict) -> float:
    if task_type == "journal_entry":
        # Debits must equal credits
        total_debits = sum(e["amount"] for e in answer["entries"] if e["type"] == "debit")
        total_credits = sum(e["amount"] for e in answer["entries"] if e["type"] == "credit")
        if abs(total_debits - total_credits) > 0.01:
            return 0.0
        # Check accounts are correct
        if answer["entries"] == problem["expected_entries"]:
            return 1.0
        return 0.5  # Balanced but different accounts
    
    elif task_type == "financial_statement":
        # Check accounting equation
        assets = answer.get("total_assets", 0)
        liabilities = answer.get("total_liabilities", 0)
        equity = answer.get("total_equity", 0)
        if abs(assets - (liabilities + equity)) > 0.01:
            return 0.0
        # Check individual line items
        correct = sum(1 for k, v in problem["expected"].items() 
                     if abs(answer.get(k, 0) - v) < 0.01)
        return correct / len(problem["expected"])
    
    elif task_type == "depreciation":
        correct = compute_depreciation(problem["method"], problem["cost"], 
                                       problem["salvage"], problem["life"], problem["year"])
        return 1.0 if abs(answer - correct) < 0.01 else 0.0
    
    elif task_type == "ratio_analysis":
        correct = compute_ratio(problem["ratio_type"], problem["financial_data"])
        return 1.0 if abs(answer - correct) / max(abs(correct), 0.001) < 0.01 else 0.0
```

## Dataset Sources
- **CPA exam questions**: Thousands of multiple-choice + simulation questions.
- **Accounting textbook exercises**: Horngren, Kieso — with solutions.
- **SEC EDGAR filings**: Real financial statements for analysis.
- **Kaggle accounting datasets**: Financial statement data.
- **Khan Academy accounting**: Structured exercises.

## Task Format
- **Input**: "Company X purchased equipment for $50,000 with a 5-year life and $5,000 salvage value. Compute Year 3 depreciation using double-declining balance."
- **Output**: "$7,200"

## Difficulty Curriculum
- Level 1: Basic journal entries, T-accounts
- Level 3: Multi-step financial statements
- Level 5: Complex depreciation, inventory methods
- Level 7: Consolidation, foreign currency translation
- Level 9: Advanced tax computation, forensic accounting

## Connections
- [[financial-calculation]] — financial computations
- [[spreadsheet-formulas]] — spreadsheet-based accounting
- [[economic-modeling]] — economic/financial analysis
