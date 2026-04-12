---
domain: data-formatting
category: format-constrained
verification_type: diff
dataset_scale: ~infinite (generate from existing data)
difficulty_range: easy/medium/hard
modality: text | code
status: verified
---

# Data Formatting / Format Conversion

## Overview

Data formatting tasks require converting structured data between formats: CSV to JSON, XML to YAML, JSON to Protocol Buffers, SQL to NoSQL, TOML to INI, and many other combinations. This is a practical, high-value RLVR domain because (1) verification is exact — parse both reference and output, compare structured content, (2) tasks are infinitely generable from existing datasets, and (3) the skill is directly useful in software engineering and data processing.

## Verification Mechanism

**Type: Diff-based (structural comparison)**

The key insight: we don't compare strings, we compare parsed structures.

```python
import json, csv, yaml, xml.etree.ElementTree as ET

def verify_format_conversion(model_output: str, reference: str, 
                              output_format: str) -> float:
    try:
        parsed_output = parse(model_output, output_format)
        parsed_reference = parse(reference, output_format)
    except ParseError:
        return 0.0  # Output doesn't parse in target format
    
    return structural_similarity(parsed_output, parsed_reference)

def structural_similarity(a, b) -> float:
    """Deep comparison ignoring key ordering, whitespace, etc."""
    if type(a) != type(b):
        return 0.0
    if isinstance(a, dict):
        if set(a.keys()) != set(b.keys()):
            # Partial credit for subset match
            common = set(a.keys()) & set(b.keys())
            if not common:
                return 0.0
            return sum(structural_similarity(a[k], b[k]) for k in common) / max(len(a), len(b))
        return sum(structural_similarity(a[k], b[k]) for k in a) / len(a) if a else 1.0
    if isinstance(a, list):
        if len(a) != len(b):
            return 0.0
        return sum(structural_similarity(ai, bi) for ai, bi in zip(a, b)) / len(a) if a else 1.0
    return 1.0 if a == b else 0.0
```

**Verification stages:**
1. **Parsability:** Does the output parse as the target format? (binary check, 0 or 0.3 reward)
2. **Schema match:** Does it have the right keys/fields? (0.3 reward)
3. **Value match:** Are all values correct? (0.4 reward)

**Verification confidence: VERY HIGH.** Format parsers are deterministic. Structural comparison handles cosmetic differences (whitespace, key ordering) gracefully. The only edge case is type coercion (string "42" vs. integer 42), which should be handled by type-aware comparison.

## Dataset Sources

- **Any existing structured dataset:** Take any CSV, JSON, XML, or YAML file and create conversion tasks between all format pairs. Kaggle, HuggingFace datasets, data.gov, etc.
- **Schema.org structured data:** Millions of structured data examples from web scraping.
- **OpenAPI specifications:** JSON/YAML conversion tasks from real API specs.
- **Configuration files in open-source repos:** GitHub has millions of config files in various formats.
- **Synthetic generation:** Generate random nested data structures, serialize to source format, task model with converting to target format.

## Task Format

**Input:**
```
Convert the following CSV data to JSON (array of objects):

name,age,city
Alice,30,New York
Bob,25,London
Charlie,35,Tokyo
```

**Output:**
```json
[
  {"name": "Alice", "age": 30, "city": "New York"},
  {"name": "Bob", "age": 25, "city": "London"},
  {"name": "Charlie", "age": 35, "city": "Tokyo"}
]
```

**Input (harder):**
```
Convert the following XML to equivalent YAML, preserving all attributes and nested structure:

<catalog>
  <book id="bk101">
    <author>Gambardella, Matthew</author>
    <title>XML Developer's Guide</title>
    <genre>Computer</genre>
    <price>44.95</price>
  </book>
  <book id="bk102">
    <author>Ralls, Kim</author>
    <title>Midnight Rain</title>
    <genre>Fantasy</genre>
    <price>5.95</price>
  </book>
</catalog>
```

## Difficulty Curriculum

1. **Easy:** CSV to JSON, JSON to YAML — flat structures, common formats.
2. **Medium:** XML to JSON (attribute handling), nested structures, arrays with mixed types.
3. **Hard:** Protocol Buffers to JSON schema, XML with namespaces, TOML with complex tables, handling edge cases (escaping, encoding, null values).
4. **Very hard:** Lossy conversions requiring judgment (XML attributes have no JSON equivalent — choose representation), schema inference from data, multi-file conversion with cross-references.

## Limitations & Risks

- **Representation ambiguity:** Some conversions have multiple valid representations (e.g., XML attributes can become JSON keys or nested objects). Must either accept multiple valid outputs or specify the convention in the prompt.
- **Type preservation:** CSV has no type information. String "42" vs integer 42 creates verification ambiguity. Specify expected type handling in the task.
- **Encoding edge cases:** Unicode, special characters, CDATA sections in XML, multiline strings in YAML — all create subtle conversion challenges that may or may not be covered by the verification parser.
- **Low reasoning ceiling:** Most format conversions are mechanical. Once the model learns the format rules, there is limited further capability gain. Best used as one component of a broader training mix.

## Connections

- [[config-generation]] — Config files are structured data; generation often involves format compliance.
- [[latex-typesetting]] — Both involve structured output that must conform to format rules.
- [[html-css-generation]] — HTML is a structured format; conversion tasks overlap.
- [[markdown-formatting]] — Markdown as a target format for content conversion.
- [[protocol-compliance]] — Protocol messages are structured data with strict format rules.
