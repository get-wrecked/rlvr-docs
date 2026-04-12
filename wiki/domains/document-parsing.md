---
domain: document-parsing
category: information-processing
verified: true
verification_type: exact
scalability: high
maturity: medium
tags: [document-understanding, layout, tables, forms, ocr]
---

# Document Parsing

## Overview

Document parsing transforms visually rich or semi-structured documents (PDFs, scanned pages, web pages, forms) into clean structured formats (markdown, HTML tables, JSON records). The model must understand document layout, table structure, heading hierarchy, reading order, and form fields.

This domain is practically important — vast amounts of information is locked in unstructured documents — and well-suited for RLVR because the parsed output can be compared against gold-standard structured representations.

## Verification Mechanism

**Verification type: Exact (with structured comparison)**

Verification compares the model's structured parse against a gold-standard parse. The comparison method depends on the output type:

- **Markdown/HTML output**: Normalize whitespace, compare structure (heading hierarchy, list nesting, table cell contents). Tools like `difflib` or tree-based comparison on parsed HTML/markdown AST.
- **Table extraction**: Compare extracted table as a grid (list of lists). Cell-level exact match with normalization.
- **Form field extraction**: Compare key-value pairs. Exact match on field names and values.
- **Layout segmentation**: Compare bounding boxes and labels against gold annotations. IoU (Intersection over Union) thresholds for box matching.

```python
def verify_table(predicted_table: list[list[str]], gold_table: list[list[str]]) -> float:
    if len(predicted_table) != len(gold_table):
        return 0.0
    correct_cells = 0
    total_cells = 0
    for pred_row, gold_row in zip(predicted_table, gold_table):
        if len(pred_row) != len(gold_row):
            return 0.0
        for pred_cell, gold_cell in zip(pred_row, gold_row):
            total_cells += 1
            if normalize(pred_cell) == normalize(gold_cell):
                correct_cells += 1
    return correct_cells / total_cells
```

For binary reward: require all cells correct (strict) or set a threshold (lenient).

## Dataset Sources

| Dataset | Task | Size | Source | Notes |
|---------|------|------|--------|-------|
| DocBank | Document layout | 500K pages | arXiv papers | Token-level layout labels |
| PubLayNet | Document layout | 360K pages | PubMed Central | 5 layout categories |
| TableBank | Table detection + structure | 417K tables | arXiv + PubMed | Table boundaries and structure |
| SciTSR | Table structure recognition | 15K tables | Scientific papers | Cell adjacency relations |
| FUNSD | Form understanding | 199 forms | Scanned forms | Entity and relation labels |
| CORD | Receipt parsing | 1K receipts | Korean receipts | 30 field types |
| XFUND | Multilingual forms | 1.4K forms | 7 languages | Cross-lingual form understanding |
| RVL-CDIP | Document classification | 400K pages | Scanned documents | 16 document types |
| DocVQA | Document QA | 50K questions | Diverse documents | Questions about document content |
| SROIE | Receipt OCR | 1K receipts | Scanned receipts | Key information extraction |

**Scalability**: arXiv provides millions of papers with both PDF and LaTeX source (the LaTeX serves as gold parse). Wikipedia has structured markup for comparison. Web pages have both rendered HTML and underlying source. Government forms often have both scanned and digital versions.

## Task Format

**Input**: A document (text extracted from PDF, OCR output, HTML source, or image description).

**Output**: A structured representation (markdown, JSON, table grid, form fields).

**Prompt template example**:
```
Parse the following document text into clean markdown with proper headings, lists, and tables.

Document text:
{raw_text}

Parsed markdown:
```

For table extraction:
```
Extract the table from the following text and output it as a JSON array of arrays (rows and cells).

Text:
{text_with_table}

Table JSON:
```

## Difficulty Curriculum

1. **Level 1 — Clean text with headings**: Well-formatted text with clear heading markers. Convert to markdown with correct heading levels.
2. **Level 2 — Simple tables**: Clearly delineated tables with aligned columns. Extract cell contents into a grid.
3. **Level 3 — Multi-column layouts**: Documents with multiple columns, sidebars, or mixed layouts. Requires reading order detection.
4. **Level 4 — Complex tables**: Tables with merged cells, hierarchical headers, or spanning rows/columns.
5. **Level 5 — Forms with handwriting**: Scanned forms with mixed printed and handwritten content. OCR errors add noise.
6. **Level 6 — Nested structures**: Documents with nested tables, tables within forms, figures with captions and references.
7. **Level 7 — Full document reconstruction**: Reconstruct complete document structure from raw PDF text extraction, including figure references, footnotes, cross-references, and bibliography.

## Limitations & Risks

- **Layout understanding from text alone**: If the model receives only extracted text (not images), spatial layout information is lost. Many parsing tasks fundamentally require visual input.
- **OCR error propagation**: If the input comes from OCR, errors in the input propagate to the parse. The model may be penalized for OCR errors it cannot fix.
- **Gold standard quality**: Document parsing gold standards are expensive to create and often contain errors, especially for complex layouts.
- **Format ambiguity**: Multiple valid markdown/HTML representations may exist for the same document. Strict string comparison can produce false negatives.
- **Domain specificity**: Models trained on scientific papers may not transfer to legal documents, financial forms, or medical records.

## Connections

- **[structured-output-generation](structured-output-generation.md)**: Document parsing produces structured output; schema validation skills transfer.
- **[table-understanding](table-understanding.md)**: Table extraction is a subtask of document parsing; parsed tables feed into table QA.
- **[information-extraction](information-extraction.md)**: IE from documents requires first parsing the document structure.
- **[data-entry-correction](data-entry-correction.md)**: OCR correction is often a prerequisite for document parsing.
- **[text-classification](text-classification.md)**: Document type classification (RVL-CDIP) is a classification task.
