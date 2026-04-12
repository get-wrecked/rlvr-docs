---
domain: financial-calculation
category: miscellaneous
verification_type: exact_match
dataset_scale: ~large (textbook + synthetic)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Financial Calculation

## Overview

Financial calculation tasks require computing results using standard financial formulas: compound interest, loan amortization, present/future value, bond pricing, option pricing (Black-Scholes), tax calculations, depreciation, ROI, break-even analysis, and portfolio metrics. These are well-defined mathematical operations with exact numerical answers.

The domain is practical (financial literacy is broadly valuable) and has clean verification (formulas are exact). It exercises multi-step arithmetic, formula application, and careful attention to parameters (compounding frequency, tax brackets, etc.).

## Verification Mechanism

**Type: Exact match (numerical with tolerance)**

```python
def verify_financial(task_type: str, params: dict, model_answer: float, 
                      tolerance: float = 0.01) -> float:
    if task_type == 'compound_interest':
        # A = P(1 + r/n)^(nt)
        P, r, n, t = params['principal'], params['rate'], params['compounds_per_year'], params['years']
        expected = P * (1 + r/n) ** (n * t)
    
    elif task_type == 'monthly_payment':
        # M = P * [r(1+r)^n] / [(1+r)^n - 1]
        P, annual_rate, years = params['principal'], params['rate'], params['years']
        r = annual_rate / 12
        n = years * 12
        expected = P * (r * (1+r)**n) / ((1+r)**n - 1)
    
    elif task_type == 'present_value':
        # PV = FV / (1 + r)^t
        FV, r, t = params['future_value'], params['rate'], params['years']
        expected = FV / (1 + r) ** t
    
    elif task_type == 'tax':
        expected = compute_tax(params['income'], params['brackets'])
    
    # ... more formulas
    
    # Allow tolerance for rounding (financial calculations often round to cents)
    if abs(model_answer - expected) <= tolerance:
        return 1.0
    elif abs(model_answer - expected) <= tolerance * 10:
        return 0.5
    return 0.0
```

**Tolerance:** Financial calculations typically round to 2 decimal places (cents). Allow $0.01 tolerance for simple calculations, $1.00 for large amounts.

**Verification confidence: VERY HIGH.** Financial formulas are exact. The only ambiguity is rounding convention (round half-up, round half-even, truncate) — specify in the task.

## Dataset Sources

- **Finance textbooks:** Brealey/Myers (corporate finance), Bodie/Kane/Marcus (investments), standard CFA curriculum. Worked examples with known answers.
- **CFA exam practice questions:** Thousands of quantitative finance problems with solutions.
- **IRS tax tables:** Official US tax brackets for tax calculation problems. Similarly for other countries.
- **Mortgage calculators:** Bankrate, NerdWallet — can generate (input, output) pairs for loan calculations.
- **Synthetic generation:** Parameterize formulas with random inputs. Unlimited scale.
  - "Calculate the monthly payment on a ${P} loan at {r}% for {t} years"
  - "What is ${X} worth in {t} years at {r}% compounded {frequency}?"
- **Kaggle finance datasets:** For context-rich problems (real company financials).
- **Series 7 / Series 66 exam prep:** Financial licensing exam questions.

## Task Format

**Input (simple):**
```
Calculate the future value of $10,000 invested at 5% annual interest, 
compounded monthly, for 10 years. Round to the nearest cent.
```

**Output:**
```
$16,470.09
```

**Input (amortization):**
```
Calculate the monthly payment for a $300,000 mortgage at 6.5% annual interest 
for 30 years. Then compute the total interest paid over the life of the loan.
Round to the nearest cent.
```

**Output:**
```
Monthly payment: $1,896.20
Total interest paid: $382,633.04
```

**Input (tax):**
```
Calculate the US federal income tax for a single filer with $95,000 taxable income 
using 2024 tax brackets:
- 10% on income up to $11,600
- 12% on $11,601 to $47,150
- 22% on $47,151 to $100,525
Show the tax for each bracket and the total.
```

**Output:**
```
10% bracket: $1,160.00
12% bracket: $4,266.00
22% bracket: $10,527.00
Total tax: $15,953.00
Effective rate: 16.79%
```

## Difficulty Curriculum

1. **Easy:** Simple interest, single compound interest calculation, percentage calculations.
2. **Medium:** Compound interest with non-annual compounding, monthly payment calculation, present/future value, basic tax bracket calculation.
3. **Hard:** Full amortization schedule, bond pricing (coupon + face value discounting), NPV/IRR calculations, multi-bracket tax with deductions and credits.
4. **Very hard:** Black-Scholes option pricing, portfolio optimization (Markowitz), capital structure (WACC), multi-year depreciation schedules (MACRS), international tax with foreign tax credits.

## Limitations & Risks

- **Rounding ambiguity:** Financial calculations are sensitive to rounding at intermediate steps vs. only at the end. Specify rounding rules.
- **Jurisdiction-specific rules:** Tax calculations depend heavily on jurisdiction, filing status, and year. Must specify all parameters.
- **Formula knowledge vs. reasoning:** Many tasks reduce to plugging numbers into formulas. The reasoning depth is in knowing *which* formula to apply, not in deep logical chains.
- **Regulatory changes:** Tax brackets, interest rate regulations, and financial rules change frequently. Training data must be timestamped or use hypothetical brackets.
- **Not financial advice:** Models trained on financial calculations should not be interpreted as providing financial advice.

## Connections

- [[unit-conversion]] — Currency conversion is a financial calculation.
- [[date-time-computation]] — Interest calculations depend on precise day counting (30/360, actual/365).
- [[recipe-scaling]] — Both involve precise arithmetic with specific formulas.
- [[legal-logic]] — Tax law is a form of legal rule application.
