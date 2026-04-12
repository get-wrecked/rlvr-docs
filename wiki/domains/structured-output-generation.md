---
domain: structured-output-generation
category: information-processing
verified: true
verification_type: exact
scalability: very-high
maturity: medium
tags: [json, xml, yaml, schema-validation, structured-data, api]
---

# Structured Output Generation

## Overview

Structured output generation tasks require the model to produce output that conforms to a formal schema — JSON matching a JSON Schema, XML matching an XSD, YAML matching a specification, SQL results, or other machine-readable formats. Verification is schema validation: the output either conforms to the schema or it does not.

This domain is highly practical (LLMs are increasingly used to produce structured API responses) and has a particularly clean verification story. Schema validators are deterministic, well-tested, and binary. There is no ambiguity: the output is valid or invalid.

Data is abundant because the internet is full of structured data (APIs, configuration files, database dumps). Any JSON document with its schema can become a training example.

## Verification Mechanism

**Verification type: Exact**

The verification pipeline:

1. **Parse**: Attempt to parse the model's output as the target format (JSON, XML, YAML). If parsing fails, reward = 0.
2. **Validate**: Validate the parsed output against the provided schema. If validation fails, reward = 0.
3. **Content check (optional)**: If gold content is available, compare the parsed output field-by-field against expected values. This adds content correctness beyond structural validity.

```python
import jsonschema

def verify(output: str, schema: dict, gold_content: dict = None) -> int:
    try:
        parsed = json.loads(output)
    except json.JSONDecodeError:
        return 0
    
    try:
        jsonschema.validate(parsed, schema)
    except jsonschema.ValidationError:
        return 0
    
    if gold_content is not None:
        return 1 if parsed == gold_content else 0
    
    return 1
```

For XML: `xmllint --schema schema.xsd output.xml`
For YAML: schema validation libraries (e.g., Cerberus, yamale)

The verification is fully programmatic, deterministic, and fast. Schema validators are mature, well-tested software.

## Dataset Sources

| Dataset | Format | Size | Source | Notes |
|---------|--------|------|--------|-------|
| JSON Schema Store | JSON | 500+ schemas | schemastore.org | Public schemas for common configs |
| GitHub API responses | JSON | Millions | GitHub | Well-documented schemas |
| OpenAPI specs | JSON/YAML | 100K+ | APIs.guru | API specifications with request/response schemas |
| WikiData JSON dumps | JSON | 100M+ entities | Wikidata | Structured knowledge base |
| Common Crawl structured data | JSON-LD | Billions | Web | Schema.org annotations |
| XML benchmarks | XML | Various | W3C, academic | Document type definitions |
| YAML configs | YAML | Millions | GitHub repos | Kubernetes, CI/CD configs |

**Scalability**: This is one of the most scalable RLVR domains. Every API on the internet produces JSON with an implicit or explicit schema. GitHub alone has millions of JSON/YAML configuration files. The key insight: any valid JSON document paired with its inferred schema becomes a training example. Data generation is essentially unlimited.

## Task Format

**Input**: A natural language description of the desired output, plus the target schema.

**Output**: A valid structured document conforming to the schema.

**Prompt template example**:
```
Generate a JSON object representing a person with the following information:
- Name: Marie Curie
- Birth year: 1867
- Fields: physics, chemistry
- Awards: Nobel Prize in Physics (1903), Nobel Prize in Chemistry (1911)

The output must conform to this JSON Schema:
{schema}

Output:
```

Alternative formulation (transformation):
```
Convert the following natural language description into a valid JSON object matching the schema.

Description: {natural_language_description}
Schema: {json_schema}

JSON:
```

## Difficulty Curriculum

1. **Level 1 — Flat JSON**: Simple key-value objects with string and number fields. `{"name": "Alice", "age": 30}`.
2. **Level 2 — Nested objects**: Objects containing objects. Requires maintaining structural hierarchy.
3. **Level 3 — Arrays and enums**: Lists of items, enum-constrained fields. Requires understanding cardinality and constraints.
4. **Level 4 — Complex schemas with validation rules**: Schemas with `required`, `minItems`, `pattern`, `additionalProperties: false`. Requires strict adherence.
5. **Level 5 — XML with namespaces**: XML documents with namespace declarations, attributes, and mixed content. More verbose and error-prone than JSON.
6. **Level 6 — Multi-format transformation**: Convert between formats (JSON to XML, CSV to YAML) while preserving content and conforming to target schema.
7. **Level 7 — Large, deeply nested documents**: Documents with 10+ levels of nesting, dozens of fields, complex cross-field constraints (if field A = X, then field B is required). Tests sustained accuracy.

## Limitations & Risks

- **Schema-only vs. content verification**: Schema validation confirms structure but not semantic correctness. A JSON object can be perfectly valid against its schema while containing nonsensical values. Content verification requires gold data.
- **Formatting sensitivity**: Minor formatting issues (trailing commas in JSON, wrong quoting) cause parse failure. The binary reward is harsh for nearly-correct output.
- **Schema complexity**: Very complex schemas (OpenAPI specs with hundreds of endpoints) may be too large to fit in context. Schema summarization or chunking may be needed.
- **Reward hacking**: A model could learn to produce valid-but-empty structures (e.g., `{}` for a permissive schema) to consistently earn rewards without doing useful work.
- **Format diversity**: Training on JSON alone may not transfer to XML or YAML. Cross-format generalization is not guaranteed.

## Connections

- **[information-extraction](information-extraction.md)**: IE outputs are typically structured (JSON, tuples). Structured output skills directly support IE.
- **[semantic-parsing](semantic-parsing.md)**: Semantic parsing produces formal representations, which are a form of structured output.
- **[document-parsing](document-parsing.md)**: Parsing documents into structured formats (markdown, tables) is a specialized form of structured output generation.
- **[table-understanding](table-understanding.md)**: Producing structured answers from tables requires structured output skills.
- **[text-to-code-nl](text-to-code-nl.md)**: Code generation shares the syntactic validity requirement — generated code must parse.
