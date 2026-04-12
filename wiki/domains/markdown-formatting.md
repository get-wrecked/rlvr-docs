---
domain: markdown-formatting
category: format-constrained
verification_type: diff
dataset_scale: ~large (GitHub + conversion from other formats)
difficulty_range: easy/medium
modality: text
status: verified
---

# Markdown Formatting

## Overview

Markdown formatting tasks require converting unstructured or semi-structured content into well-formatted Markdown, or converting between Markdown and other formats (HTML, LaTeX, reStructuredText). This is a ubiquitous practical skill: documentation, README files, wiki pages, chat messages, and technical writing all use Markdown.

The domain has clean RLVR properties: Markdown can be parsed into an AST (abstract syntax tree), and structural comparison between the model output's AST and a reference AST provides deterministic verification. The domain is somewhat narrow in difficulty range but highly practical.

## Verification Mechanism

**Type: Diff-based (structural AST comparison)**

**Primary approach — AST comparison:**
```python
import markdown
from markdown.extensions import toc, tables, fenced_code

def verify_markdown(model_output: str, reference: str) -> float:
    # Parse both to AST
    md = markdown.Markdown(extensions=['tables', 'fenced_code', 'toc'])
    out_html = md.convert(model_output)
    md.reset()
    ref_html = md.convert(reference)
    
    # Compare HTML trees
    from lxml import etree
    out_tree = etree.fromstring(f"<root>{out_html}</root>")
    ref_tree = etree.fromstring(f"<root>{ref_html}</root>")
    
    return tree_similarity(out_tree, ref_tree)
```

**Alternative: element-level checks:**
- Correct number of headings at each level
- Correct number of list items (ordered/unordered)
- Code blocks present with correct language tags
- Tables with correct row/column counts
- Links and images with correct targets
- Emphasis/bold on correct text spans

**Reward structure:**
- Parseable Markdown (always true — Markdown is lenient): 0.1
- Heading structure matches: 0.2
- Content structure matches (lists, tables, code blocks): 0.3
- Text content matches: 0.3
- Exact structural match: 0.1 bonus

**Verification confidence: HIGH.** Markdown parsing is deterministic. AST comparison handles cosmetic differences (extra blank lines, indentation style, ATX vs. Setext headings). The main challenge is that Markdown is very forgiving — almost anything is "valid" Markdown — so quality differentiation must come from structural comparison to a reference.

## Dataset Sources

- **GitHub README files:** Millions of well-formatted Markdown files across GitHub repositories. Pairs can be created by extracting content and re-formatting.
- **Stack Overflow posts:** Posts in Markdown with known formatting.
- **CommonMark spec tests:** The CommonMark specification includes hundreds of input/output test cases.
- **Pandoc test suite:** Conversion test cases between Markdown and other formats.
- **Technical documentation:** ReadTheDocs, MkDocs sites — content with known Markdown structure.
- **Synthetic generation:** Take structured content (JSON, HTML, plain text with implicit structure) and create "convert to Markdown" tasks.
- **Wikipedia to Markdown:** Convert Wikipedia markup to Markdown — large-scale paired data.

## Task Format

**Input (plain text to Markdown):**
```
Format the following text as Markdown with appropriate headings, lists, and emphasis:

Project Setup Guide

First, you need to install the dependencies. Run npm install in the project directory. 
Make sure you have Node.js version 16 or higher.

Configuration: You'll need to set three environment variables:
DATABASE_URL - The connection string for your database
API_KEY - Your API authentication key  
DEBUG - Set to true for development

Important: Never commit your .env file to version control.
```

**Output:**
```markdown
# Project Setup Guide

First, you need to install the dependencies. Run `npm install` in the project directory.
Make sure you have **Node.js version 16 or higher**.

## Configuration

You'll need to set three environment variables:

- `DATABASE_URL` — The connection string for your database
- `API_KEY` — Your API authentication key
- `DEBUG` — Set to `true` for development

> **Important:** Never commit your `.env` file to version control.
```

**Input (HTML to Markdown):**
```
Convert this HTML to clean Markdown:
<h1>Title</h1><p>Some <strong>bold</strong> and <em>italic</em> text.</p>
<ul><li>Item 1</li><li>Item 2</li></ul>
```

## Difficulty Curriculum

1. **Easy:** Format flat text with headings and paragraphs. Basic emphasis (bold, italic).
2. **Medium:** Lists (nested, ordered/unordered), code blocks (inline and fenced), links, images, blockquotes.
3. **Hard:** Tables, footnotes, task lists, complex nesting, GFM-specific features. HTML-to-Markdown conversion preserving all structure.
4. **Very hard:** Multi-format conversion chains (reStructuredText to Markdown, AsciiDoc to Markdown). Handling ambiguous input where structural interpretation requires judgment.

## Limitations & Risks

- **Low ceiling:** Markdown formatting is not a deep reasoning task. It exercises format knowledge and attention to detail, but once mastered, provides diminishing returns for further training.
- **Markdown flavor ambiguity:** CommonMark, GitHub Flavored Markdown (GFM), and other flavors differ in table syntax, task lists, footnotes, etc. Must specify flavor in the task.
- **Subjectivity in formatting choices:** Some formatting decisions are stylistic (ATX vs. Setext headings, `*` vs. `-` for lists). Multiple valid answers exist. AST comparison (rather than string comparison) handles this.
- **Markdown is too lenient:** Almost any text is valid Markdown, so "parseable" is not a meaningful bar. Quality differentiation must rely on structural comparison to reference.

## Connections

- [[data-formatting]] — Markdown formatting is a specialized case of format conversion.
- [[html-css-generation]] — HTML is the compilation target for Markdown; bidirectional conversion.
- [[latex-typesetting]] — Markdown-to-LaTeX conversion is a specific task.
- [[instruction-following]] — Formatting instructions are a subset of general instructions.
