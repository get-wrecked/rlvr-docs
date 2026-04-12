---
domain: legal-logic
category: miscellaneous
verification_type: rule | exact_match
dataset_scale: ~medium (bar exam + LSAT + synthetic)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Legal Logic

## Overview

Legal logic tasks require applying legal rules to fact patterns to determine outcomes. This includes simplified statutory reasoning ("Does this fact pattern satisfy the elements of burglary?"), LSAT analytical reasoning (logic games), and rule-based contract or tort analysis. The key constraint for RLVR is that the legal rules must be *fully specified in the prompt* — the model applies given rules to given facts, not open-ended legal judgment.

This domain is valuable because it exercises formal rule application, conditional reasoning, multi-factor analysis, and careful reading — skills that transfer broadly. When the rules are fully specified, the task becomes a deterministic logic problem amenable to programmatic verification.

## Verification Mechanism

**Type: Rule-based / Exact match**

**For LSAT Analytical Reasoning (Logic Games):**
Each question has a single correct answer (A/B/C/D/E). Verification is exact match.

```python
def verify_lsat(model_answer: str, correct_answer: str) -> float:
    return 1.0 if model_answer.strip().upper() == correct_answer.strip().upper() else 0.0
```

**For rule-application tasks:**
Given explicit rules and facts, build a rule engine and compute the expected outcome.

```python
def verify_rule_application(rules: list, facts: dict, model_conclusion: str) -> float:
    """
    rules: list of if-then rules, e.g., 
      [{"if": ["enters_building", "at_night", "intent_to_commit_felony"], "then": "burglary"}]
    facts: dict of fact values, e.g., 
      {"enters_building": True, "at_night": True, "intent_to_commit_felony": True}
    """
    expected = apply_rules(rules, facts)
    return 1.0 if model_conclusion.strip().lower() == expected.lower() else 0.0
```

**For multi-step reasoning:**
Provide a chain of rules and verify the final conclusion matches what the rule engine computes. Partial credit for correctly applying some rules but not others.

**Verification confidence: HIGH for LSAT (ground truth exists), MEDIUM-HIGH for rule-application (depends on rule engine fidelity).** When rules are fully formalized, verification is deterministic. The risk is in tasks where legal reasoning requires interpretation beyond the stated rules — those tasks are NOT suitable for RLVR.

## Dataset Sources

- **LSAT Analytical Reasoning (Logic Games):** ~400 logic games with ~1600 questions from published LSAT exams (LSAC). Each has exactly one correct answer. Available through test prep publishers.
- **Bar exam MBE questions:** Multistate Bar Examination questions test rule application. ~2000 publicly available practice questions with answer keys.
- **LSAT Logical Reasoning:** ~3000 questions testing argument analysis. Single correct answer per question.
- **LegalBench (Guha et al., 2023):** 162 legal reasoning tasks with ground truth labels, collaboratively built by legal professionals.
- **CaseHOLD (Zheng et al., 2021):** Case holding prediction dataset — which legal rule applies to a case.
- **Synthetic rule-application tasks:** Define simplified legal rules (elements of crimes, contract formation requirements, negligence factors) and generate fact patterns with deterministic outcomes.
- **Prolog-style legal rule systems:** Formalized legal rules that can serve as both training data and verification engines.

## Task Format

**Input (LSAT Logic Game):**
```
A committee must select exactly 4 members from: F, G, H, J, K, L, M.
Rules:
- If F is selected, G must also be selected.
- H and K cannot both be selected.
- If J is selected, neither K nor L can be selected.
- Either G or M (but not both) must be selected.

Question: Which of the following is an acceptable selection?
A) F, G, H, J
B) F, G, H, M
C) G, H, L, M
D) F, G, J, L
E) G, H, K, M
```

**Output:**
```
C
```

**Input (rule application):**
```
Under the following simplified burglary statute, determine if the defendant is 
guilty of burglary:

Statute: A person commits burglary if they:
1. Enter a building or occupied structure
2. Without authorization
3. With the intent to commit a crime therein

Facts: The defendant walked through an unlocked door into an open retail store 
during business hours. The defendant intended to shoplift merchandise.

Is the defendant guilty of burglary under this statute? (Yes/No, with reasoning)
```

**Output:**
```
No. Element 2 is not satisfied. Entering a retail store during business hours 
through an open door constitutes implied authorization to enter, even if the 
defendant intended to commit a crime once inside.
```

(Verification: rule engine determines that "open retail store during business hours" maps to "authorization" = True, making element 2 unsatisfied.)

## Difficulty Curriculum

1. **Medium:** Simple rule application with 2-3 rules and clear facts. LSAT logic games with few constraints.
2. **Hard:** Multi-rule application with conflicting factors, LSAT logic games with 5-6 constraints and complex interactions, bar exam MBE questions requiring multi-step analysis.
3. **Very hard:** Rules with exceptions and exceptions-to-exceptions, multi-party scenarios (contracts, torts with multiple defendants), precedent-based reasoning where similar-but-different cases must be distinguished.

Note: There is no "easy" tier because even simple legal reasoning requires understanding conditional logic. And there is no "superhuman" tier because truly hard legal reasoning requires open-ended judgment beyond RLVR scope.

## Limitations & Risks

- **Boundary of formalizability:** Real legal reasoning involves ambiguity, policy considerations, and judgment calls that cannot be captured by a rule engine. RLVR is limited to the formalized subset.
- **Rule completeness assumption:** The approach assumes all relevant rules are stated in the prompt. In real law, relevant rules must be identified from vast legal databases — that task is not verifiable.
- **Cultural and jurisdictional specificity:** Legal rules vary by jurisdiction. Bar exam questions are US-specific. Must account for this in dataset diversity.
- **Risk of false confidence:** A model that does well on simplified legal logic may appear to have legal reasoning ability but fail catastrophically on real legal problems requiring judgment.
- **LSAT data licensing:** Published LSAT questions are copyrighted by LSAC. Fair use for training is arguable but not certain.

## Connections

- [[financial-calculation]] — Tax computation is legal rule application with numbers.
- [[instruction-following]] — Legal rule application is a specific case of following complex instructions.
- [[constraint-satisfaction]] — Legal analysis with multiple rules is constraint satisfaction over a fact space.
- [[math-competition]] — LSAT logic games exercise similar deductive reasoning skills.
