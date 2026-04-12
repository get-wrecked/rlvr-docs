---
domain: Web Scraping
category: Code & Software
verification_type: execution-based (run scraper against cached pages, compare extracted data to reference)
dataset_scale: 500–10K scraping tasks
difficulty_range: single-element extraction → multi-page crawl with pagination and dynamic content
modality: text-to-code (BeautifulSoup, Scrapy, Selenium, CSS/XPath selectors)
status: early-stage
---

## Overview

Web scraping tasks the model with generating code that extracts structured data from HTML/web pages. Verification is execution-based: run the generated scraping code against cached (frozen) copies of web pages and compare the extracted data to a reference dataset.

This domain is viable for RLVR because (a) HTML pages can be cached for reproducibility, (b) extracted data can be compared structurally against ground truth, and (c) the task is practically valuable — web scraping is a common data-collection need. The key challenge is constructing a large enough dataset of cached pages with ground-truth extractions.

## Verification Mechanism

```
def verify_scraper(generated_code, cached_pages, expected_data):
    # 1. Set up environment with cached pages served locally
    local_server = start_static_server(cached_pages, port=8080)

    # 2. Execute generated scraping code
    try:
        exec_env = {
            "requests": mock_requests(local_server),  # redirect to local
            "BeautifulSoup": bs4.BeautifulSoup,
            "base_url": f"http://localhost:8080"
        }
        exec(generated_code, exec_env)
        extracted = exec_env.get("result", None)
    except Exception as e:
        local_server.stop()
        return 0.0
    finally:
        local_server.stop()

    if extracted is None:
        return 0.0

    # 3. Compare extracted data to reference
    return compare_extracted(extracted, expected_data)


def compare_extracted(extracted, expected, tolerance=0.95):
    """
    Compare extracted data to expected, handling various formats.
    """
    if isinstance(expected, list):
        # List of records: check coverage
        if not isinstance(extracted, list):
            return 0.0

        # Normalize both lists
        ext_normalized = [normalize_record(r) for r in extracted]
        exp_normalized = [normalize_record(r) for r in expected]

        # Check precision and recall
        matched = 0
        for exp_record in exp_normalized:
            if find_matching_record(exp_record, ext_normalized):
                matched += 1

        recall = matched / len(exp_normalized) if exp_normalized else 0.0
        precision = matched / len(ext_normalized) if ext_normalized else 0.0

        if precision == 0 or recall == 0:
            return 0.0
        f1 = 2 * precision * recall / (precision + recall)

        # Strict: require exact match; partial: return F1
        return 1.0 if f1 >= tolerance else 0.0

    elif isinstance(expected, dict):
        # Single record or structured output
        matches = sum(
            1 for k, v in expected.items()
            if normalize_value(extracted.get(k)) == normalize_value(v)
        )
        return matches / len(expected)

    elif isinstance(expected, str):
        # Single value extraction
        return 1.0 if normalize_value(extracted) == normalize_value(expected) else 0.0
```

Key considerations:

- **Page caching**: Web pages are saved as static HTML (with CSS/images) at a specific point in time. This ensures reproducibility. Tools: `wget --mirror`, `httrack`, or Playwright's `page.content()`.
- **Mock HTTP**: Redirect all HTTP requests from the scraping code to a local static server serving cached pages. This prevents external network access and ensures determinism.
- **Dynamic content**: Pages requiring JavaScript rendering need cached rendered HTML (save after JS execution via Playwright/Puppeteer) or the verification environment must include a headless browser.
- **Normalization**: Extracted text should be normalized (strip whitespace, lowercase for case-insensitive fields, parse numbers and dates) before comparison.
- **Partial credit**: For large extraction tasks (e.g., scrape all products), F1 score between extracted and expected records provides useful gradient signal.

## Dataset Sources

| Dataset | Size | Domain | Notes |
|---------|------|--------|-------|
| **SWDE** | 8 verticals, 80 websites | Structured extraction | Auto, book, camera, job, movie, NBA, restaurant, university |
| **WebSRC** | 6,500 QA pairs | Web-based reading comprehension | Question answering from web page screenshots/HTML |
| **MiniWoB++** | 100+ tasks | Web interaction | Form filling, navigation; extends to scraping |
| **Mind2Web** | 2,350 tasks | Web agent tasks | Real-world website tasks with interaction traces |
| **ACHE-focused datasets** | Varies | Focused crawling | Vertical-specific extraction from focused crawls |
| **Common Crawl extractions** | Scalable | General web | HTML pages with hand-labeled extractions for specific verticals |
| **WebArena** | 812 tasks | Web agent tasks | Multi-site tasks on realistic web environments |
| **Klarna product pages** | 51,000 | E-commerce | Product attribute extraction from retailer pages |

**Synthetic data generation**:
1. Cache popular website pages (e-commerce, news, Wikipedia, job boards).
2. Manually or semi-automatically label structured data in each page (product names, prices, dates, authors, etc.).
3. Create tasks: "Extract all product names and prices from this page."
4. Scale by using multiple pages from the same site with the same schema.

## Task Format

**Input prompt**:
```
Write Python code using BeautifulSoup to extract all product
information from the given HTML page. For each product, extract:
- name (text)
- price (float, in dollars)
- rating (float, out of 5)
- availability (boolean)

The page URL is available as `base_url` variable.
Store the result as a list of dicts in a variable called `result`.
```

**Cached HTML** (served locally, not shown to model):
```html
<div class="product-card">
  <h3 class="product-name">Wireless Mouse</h3>
  <span class="price">$29.99</span>
  <div class="rating">4.5 / 5</div>
  <span class="stock in-stock">In Stock</span>
</div>
<!-- ... more products ... -->
```

**Expected output**:
```python
[
  {"name": "Wireless Mouse", "price": 29.99, "rating": 4.5, "availability": True},
  # ...
]
```

## Difficulty Curriculum

| Level | Extraction Complexity | Example |
|-------|----------------------|---------|
| 1 | Single element, simple selector | Extract the page title |
| 2 | Multiple elements, class-based | All product names from a list |
| 3 | Nested structure, data parsing | Products with prices, ratings (parse strings to numbers) |
| 4 | Multiple pages, pagination | Follow "next page" links, aggregate results |
| 5 | Dynamic content, JavaScript-rendered | Content loaded via AJAX/JS (requires Playwright/Selenium) |
| 6 | Anti-scraping measures | Handle lazy loading, infinite scroll, CAPTCHA workarounds |

For RLVR, levels 1–4 are the most practical since they work with cached static HTML. Levels 5–6 require more complex environments.

## Limitations & Risks

- **Page staleness**: Cached pages become stale as websites change. Periodic re-caching is needed, but this changes the ground truth.
- **CSS/HTML diversity**: The same information can be structured in countless ways across websites. Models must generalize across HTML patterns, not memorize specific selectors.
- **JavaScript dependency**: Many modern sites render content via JavaScript. Cached HTML from a simple GET request may be empty. Using browser-rendered cached pages (Playwright snapshot) addresses this but adds complexity.
- **Legal and ethical concerns**: Scraping may violate terms of service. Training data should use pages where scraping is permitted or use synthetic HTML.
- **Limited dataset scale**: Compared to code generation, labeled scraping datasets are small. Synthetic HTML generation can help but may not capture real-world HTML complexity.
- **Robustness**: A scraper that works perfectly on cached pages may break on slightly different page versions. This is inherent to the domain but limits real-world reliability.

## Connections

- **Data Wrangling** is the natural next step: after extracting raw data from web pages, it needs to be cleaned, reshaped, and analyzed.
- **Regex Synthesis** is a building block for text extraction from HTML content.
- **Shell Commands** can perform simple scraping (curl + grep/sed), connecting the domains.
- **API Usage** is the structured alternative: when an API exists, scraping is unnecessary.
- **SQL Generation** operates on the structured data that scraping produces.
