"""
Exact Match Verifier — covers ~1,282 domains (47% of all domains).

Handles:
  - Normalized string comparison (QA, classification, factual)
  - Numeric extraction and comparison (math, science, finance)
  - Multiple correct answers (any match = correct)
  - F1 token overlap for partial credit
  - LaTeX/boxed answer extraction
  - GSM8K #### format extraction
  - Multiple choice letter extraction (A/B/C/D)
"""

import re
import string
import math
from typing import Union
from . import VerifyResult


def normalize_text(text: str) -> str:
    """Normalize text for comparison: lowercase, remove articles/punctuation, collapse whitespace."""
    text = text.lower().strip()
    # Remove articles
    text = re.sub(r'\b(a|an|the)\b', ' ', text)
    # Remove punctuation
    text = text.translate(str.maketrans('', '', string.punctuation))
    # Collapse whitespace
    text = re.sub(r'\s+', ' ', text).strip()
    return text


def extract_answer(response: str) -> str:
    """Extract the answer from a model response using multiple strategies."""
    # Strategy 1: LaTeX \boxed{...}
    boxed = re.findall(r'\\boxed\{([^}]+)\}', response)
    if boxed:
        return boxed[-1].strip()

    # Strategy 2: GSM8K #### format
    gsm = re.search(r'####\s*(.+?)(?:\n|$)', response)
    if gsm:
        return gsm.group(1).strip()

    # Strategy 3: "The answer is X" / "Answer: X"
    answer_patterns = [
        r'[Tt]he (?:final )?answer is[:\s]*(.+?)(?:\.|$)',
        r'[Aa]nswer[:\s]+(.+?)(?:\.|$)',
        r'[Tt]herefore[,\s]+(?:the answer is\s+)?(.+?)(?:\.|$)',
    ]
    for pat in answer_patterns:
        m = re.search(pat, response)
        if m:
            return m.group(1).strip()

    # Strategy 4: Multiple choice letter (last occurrence of single letter)
    mc = re.findall(r'\b([A-E])\b', response)
    if mc:
        return mc[-1]

    # Strategy 5: Last line
    lines = response.strip().split('\n')
    return lines[-1].strip()


def extract_number(text: str) -> float | None:
    """Extract a numeric value from text."""
    text = text.strip()
    # Remove commas in numbers
    text = text.replace(',', '')
    # Remove $ and % signs
    text = re.sub(r'[\$%]', '', text)
    # Try direct float parse
    try:
        return float(text)
    except ValueError:
        pass
    # Try fraction
    frac = re.match(r'^(-?\d+)\s*/\s*(\d+)$', text)
    if frac:
        num, den = int(frac.group(1)), int(frac.group(2))
        if den != 0:
            return num / den
    # Extract last number from text
    numbers = re.findall(r'-?\d+\.?\d*', text)
    if numbers:
        try:
            return float(numbers[-1])
        except ValueError:
            pass
    return None


def compute_f1(prediction: str, gold: str) -> float:
    """Compute token-level F1 between prediction and gold."""
    pred_tokens = normalize_text(prediction).split()
    gold_tokens = normalize_text(gold).split()
    if not pred_tokens or not gold_tokens:
        return 0.0
    common = set(pred_tokens) & set(gold_tokens)
    if not common:
        return 0.0
    precision = len(common) / len(pred_tokens)
    recall = len(common) / len(gold_tokens)
    return 2 * precision * recall / (precision + recall)


def verify(task: dict, response: str) -> VerifyResult:
    """
    Verify a response against a task using exact match.

    Task format:
        {
            "gold": "answer" | ["answer1", "answer2"],  # correct answer(s)
            "type": "text" | "number" | "mcq",           # comparison type
            "tolerance": 1e-5,                            # for numbers (optional)
            "partial_credit": true,                       # enable F1 (optional)
        }
    """
    gold = task.get("gold", task.get("answer", ""))
    task_type = task.get("type", "auto")
    tolerance = task.get("tolerance", 1e-5)
    partial = task.get("partial_credit", False)

    # Handle multiple correct answers
    if isinstance(gold, list):
        golds = gold
    else:
        golds = [gold]

    # Extract answer from response
    extracted = extract_answer(response)

    # Auto-detect type
    if task_type == "auto":
        if any(re.match(r'^-?\d+\.?\d*$', str(g).strip().replace(',', '')) for g in golds):
            task_type = "number"
        elif any(re.match(r'^[A-E]$', str(g).strip()) for g in golds):
            task_type = "mcq"
        else:
            task_type = "text"

    # === NUMERIC COMPARISON ===
    if task_type == "number":
        pred_num = extract_number(extracted)
        if pred_num is None:
            pred_num = extract_number(response)
        if pred_num is None:
            return VerifyResult.wrong("Could not extract numeric answer")

        for g in golds:
            gold_num = extract_number(str(g))
            if gold_num is None:
                continue
            # Integer comparison (exact)
            if gold_num == int(gold_num) and abs(pred_num - gold_num) < 0.5:
                if round(pred_num) == int(gold_num):
                    return VerifyResult.correct(f"Numeric match: {pred_num} == {gold_num}")
            # Float comparison (tolerance)
            if abs(gold_num) < 1e-10:
                if abs(pred_num - gold_num) <= tolerance:
                    return VerifyResult.correct(f"Numeric match within tolerance")
            elif abs(pred_num - gold_num) / max(abs(gold_num), 1e-10) <= tolerance:
                return VerifyResult.correct(f"Numeric match within tolerance")

        return VerifyResult.wrong(f"Numeric mismatch: extracted {pred_num}, expected {golds}")

    # === MULTIPLE CHOICE ===
    if task_type == "mcq":
        # Extract letter
        letters = re.findall(r'\b([A-E])\b', response)
        pred_letter = letters[-1] if letters else extracted.strip().upper()

        for g in golds:
            if pred_letter == str(g).strip().upper():
                return VerifyResult.correct(f"MCQ match: {pred_letter}")

        return VerifyResult.wrong(f"MCQ mismatch: {pred_letter} != {golds}")

    # === TEXT COMPARISON ===
    pred_norm = normalize_text(extracted)

    for g in golds:
        gold_norm = normalize_text(str(g))
        if pred_norm == gold_norm:
            return VerifyResult.correct(f"Exact text match")

    # Partial credit via F1
    if partial:
        best_f1 = max(compute_f1(extracted, str(g)) for g in golds)
        if best_f1 > 0.5:
            return VerifyResult.partial(best_f1, f"Partial match (F1={best_f1:.3f})")

    return VerifyResult.wrong(f"Text mismatch: '{pred_norm}' not in {[normalize_text(str(g)) for g in golds]}")
