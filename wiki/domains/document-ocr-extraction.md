---
domain: document-ocr-extraction
category: vision/document
verification_type: exact_match | diff
dataset_scale: >100K document images
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Document OCR & Data Extraction

## Overview

Document OCR and data extraction tasks require an agent to extract structured information from images of documents — receipts, invoices, forms, academic papers, business cards, tables, and handwritten text. The RLVR value is high: the ground truth is known (the actual text and fields in the document), verification is exact match, and the task has enormous practical value for enterprise automation. This domain tests fine-grained visual perception, layout understanding, text recognition, and the ability to parse diverse document formats.

## Verification Mechanism

1. **Exact text match** (exact match): Compare extracted text to ground truth character-by-character. Standard OCR metrics: Character Error Rate (CER) and Word Error Rate (WER). For RLVR, can threshold: reward = 1 if CER < epsilon, 0 otherwise.
2. **Field extraction match** (exact match): For structured extraction (key-value pairs from forms), compare each extracted field to ground truth. Per-field exact match after normalization (whitespace, case). F1 score over fields.
3. **Table extraction match** (diff-based): For table extraction, compare extracted table structure (rows, columns, cell values) to ground truth. Cell-level exact match. Structure comparison via tree edit distance.
4. **ANLS (Average Normalized Levenshtein Similarity)**: Standard metric for document VQA. Computes normalized edit distance between predicted and ground truth answers. ANLS = 1 - NLD if NLD < threshold, else 0. More forgiving than exact match for near-correct extractions.
5. **Bounding box + text match** (constraint-based): For OCR with localization, verify both the text content and its bounding box location. IoU for boxes + exact match for text.

## Dataset Sources

- **DocVQA**: https://www.docvqa.org/ — 50K questions on 12K document images. Industry documents, handwritten notes, printed forms. Answers are text spans from the document. ANLS evaluation.
- **InfographicVQA**: https://www.docvqa.org/datasets/infographicvqa — 30K questions on 5K infographic images. Requires layout understanding.
- **FUNSD**: https://guillaumejaume.github.io/FUNSD/ — 199 fully annotated scanned forms. Key-value pair extraction with semantic entity labels (header, question, answer). Entity-level F1 evaluation.
- **SROIE**: https://rrc.cvc.uab.es/?ch=13 — 1,000 scanned receipt images. Extract: company name, date, address, total. Per-field exact match.
- **CORD**: https://github.com/clovaai/cord — 11K receipt images with 30 field types. More diverse than SROIE.
- **RVL-CDIP**: https://huggingface.co/datasets/rvl_cdip — 400K document images across 16 categories (letter, memo, invoice, etc.). Classification task, not extraction — useful for pre-training.
- **IIT-CDIP**: ~11M document page images. Massive scale for pre-training.
- **PubTabNet**: https://github.com/ibm-aur-nlp/PubTabNet — 568K table images from PubMed papers with HTML structure annotations. Table structure recognition.
- **TableBank**: https://github.com/doc-analysis/TableBank — 417K table images from Word/LaTeX documents.
- **IAM Handwriting**: https://fki.tic.heia-fr.ch/databases/iam-handwriting-database — 1,539 handwritten English text pages. 13K lines, 115K words. Character/word-level ground truth.
- **XFUND**: https://github.com/doc-analysis/XFUND — Multilingual form understanding. 7 languages, 1,393 forms.
- **DeepForm**: https://github.com/jstray/deepform — Political ad spending forms with extraction targets.
- **Kleister**: https://github.com/applicaai/kleister-nda — NDA document information extraction benchmark.

## Task Format

**Document question answering**:
- Input: Document image + question (e.g., "What is the total amount?" over a receipt image).
- Output: Text answer extracted from the document (e.g., "$42.50").
- Verification: ANLS or exact match after normalization.

**Key-value extraction**:
- Input: Form/receipt image + list of fields to extract (e.g., "Extract: date, vendor, total, tax").
- Output: Structured key-value pairs (e.g., `{"date": "2024-03-15", "vendor": "Acme Corp", "total": "42.50", "tax": "3.25"}`).
- Verification: Per-field exact match. F1 over fields.

**Full-page OCR**:
- Input: Document page image.
- Output: Complete text transcription preserving layout/reading order.
- Verification: CER/WER against ground truth transcription.

**Table extraction**:
- Input: Image containing a table.
- Output: Structured table (HTML, CSV, or JSON).
- Verification: Cell-level exact match. TEDS (Tree Edit Distance-based Similarity) score.

## Difficulty Curriculum

1. **Printed text, clean scans** (easy): High-quality scanned documents with clear fonts. Standard OCR.
2. **Receipt extraction** (easy-medium): Receipts with standard fields. SROIE level.
3. **Form key-value extraction** (medium): Diverse form layouts. FUNSD level.
4. **Document QA, simple** (medium): Find specific information in a document. Lookup questions.
5. **Table extraction** (medium-hard): Parse table structure and content. PubTabNet level.
6. **Handwritten text recognition** (hard): IAM-level handwritten English. Variable handwriting quality.
7. **Document QA, reasoning** (hard): Questions requiring multi-field reasoning. "Is the total correct given the itemized amounts?"
8. **Degraded/noisy documents** (hard): Old scans, faded text, stamps, watermarks, coffee stains.
9. **Multi-page document understanding** (very hard): Extract information spanning multiple pages.
10. **Multilingual documents** (very hard): Non-Latin scripts, mixed-language documents. XFUND level.

## Limitations & Risks

- **Normalization is tricky**: "$42.50" vs "42.50" vs "$42.5" vs "42.50 USD" should all match for a currency field. Building robust normalization is non-trivial and domain-specific.
- **Layout ambiguity**: Some documents have ambiguous layouts where the reading order is unclear. Ground truth may reflect one interpretation, but another is equally valid.
- **Handwriting variability**: Handwritten documents have enormous variation in legibility. Ground truth annotators may disagree on illegible text.
- **Privacy concerns**: Document datasets often contain real personal information (names, addresses, financial data). Many datasets are anonymized/redacted but some are not.
- **Domain specificity**: A model trained on receipts may fail on medical forms. Document diversity is a real challenge.
- **Ground truth errors**: Large-scale annotation of document content is error-prone. Expect 1-5% annotation errors in most datasets, which creates noise in the reward signal.
- **Format fragility**: Exact match is too strict for some cases (date formats, number formatting) but any relaxation risks accepting incorrect answers.

## Connections

- [[visual-question-answering]] — DocVQA is a specialized form of VQA applied to documents
- [[chart-understanding]] — Charts in documents are a related extraction target
- [[spreadsheet-tasks]] — Extracting tabular data from documents feeds into spreadsheet workflows
- [[image-classification]] — Document classification is a prerequisite step for extraction pipelines
