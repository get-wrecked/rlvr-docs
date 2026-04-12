---
domain: API Usage
category: Code & Software
verification_type: execution-based (call API, validate response against expected schema/data)
dataset_scale: 1K–50K API tasks
difficulty_range: single REST call → multi-step orchestration with auth and pagination
modality: text-to-API-call (or text-to-code-with-API)
status: growing
---

## Overview

API usage tasks the model with generating correct API calls — or code that makes API calls — to accomplish a specified goal. Verification is execution-based: run the generated code against a real or mock API endpoint and check that the response matches the expected result, the correct endpoint was called, and the parameters are valid.

This domain is a natural RLVR target because (a) APIs have well-defined specifications (OpenAPI/Swagger), enabling automated verification; (b) the task is extremely practical — tool-using AI agents depend on correct API invocation; and (c) the verification can range from simple schema validation to full end-to-end execution checking.

## Verification Mechanism

```
def verify_api_usage(task, generated_code, api_environment):
    # 1. Execute generated code against mock/sandboxed API
    try:
        result = run_in_sandbox(
            code=generated_code,
            env=api_environment,  # mock server or sandboxed real API
            timeout=30,
            network="mock_only"  # only allow calls to mock server
        )
    except TimeoutError:
        return 0.0
    except RuntimeError:
        return 0.0

    # 2. Verify API was called correctly
    recorded_calls = api_environment.get_recorded_calls()
    checks = []

    for expected_call in task.expected_api_calls:
        matching = find_matching_call(recorded_calls, expected_call)
        if matching is None:
            checks.append(False)
            continue

        # Check endpoint
        checks.append(matching.url == expected_call.endpoint)
        # Check method
        checks.append(matching.method == expected_call.method)
        # Check required parameters
        for param, value in expected_call.required_params.items():
            checks.append(matching.params.get(param) == value)
        # Check required headers (auth, content-type)
        for header, value in expected_call.required_headers.items():
            checks.append(matching.headers.get(header) == value)

    # 3. Verify final output/result
    if task.expected_output is not None:
        checks.append(
            normalize(result.output) == normalize(task.expected_output)
        )

    # 4. Verify response handling
    if task.expected_parsed_data is not None:
        checks.append(
            result.parsed_data == task.expected_parsed_data
        )

    return 1.0 if all(checks) else 0.0
```

Mock API server approach:

```
def setup_mock_api(api_spec):
    """
    Create a mock API server from an OpenAPI spec with
    predefined responses for verification.
    """
    mock = MockServer()
    for endpoint in api_spec.endpoints:
        mock.register(
            method=endpoint.method,
            path=endpoint.path,
            # Return predefined response when called correctly
            response=endpoint.mock_response,
            # Validate request against schema
            request_validator=endpoint.request_schema
        )
    mock.start()
    return mock
```

Key considerations:

- **Mock vs. real APIs**: Mock APIs are reproducible and free but may miss real-world edge cases. Sandboxed real APIs (rate-limited, test accounts) are more realistic but costly and potentially non-deterministic.
- **Authentication**: Tasks involving OAuth, API keys, or tokens require the mock environment to simulate auth flows.
- **Pagination and rate limiting**: Advanced tasks require the model to handle paginated responses and respect rate limits. The mock server can simulate these.
- **Error handling**: Verification should check that the model handles API errors gracefully (retries, error messages, fallback logic).

## Dataset Sources

| Dataset | Size | Scope | Notes |
|---------|------|-------|-------|
| **ToolBench** | 16,464 APIs | Real-world REST APIs | 49 categories from RapidAPI; includes execution traces |
| **API-Bank** | 2,138 API calls | 73 API tools | Multi-turn dialogues with API calls; banking, weather, etc. |
| **APIBench (Gorilla)** | 16,789 pairs | HuggingFace, TorchHub, TensorHub APIs | NL → API call with version constraints |
| **RestBench** | 157 tasks | Spotify, TMDB APIs | Two real-world APIs with complex multi-step tasks |
| **ToolAlpaca** | 3,938 instances | 400 simulated APIs | LLM-generated tool-use dataset |
| **MetaTool** | 21,127 instances | 195 APIs | Large-scale tool-use benchmark |
| **NexusRaven** | 3,208 instances | Function calling | Focus on accurate function call generation |
| **Seal-Tools** | 21 real APIs | Complex API tasks | Emphasis on multi-step, multi-tool planning |
| **T-Bench** | 682 tasks | 218 tools | Interactive tool-use benchmark with state tracking |

**Synthetic data generation**: From any OpenAPI/Swagger specification, generate (task description, expected API call) pairs by:
1. Enumerating endpoints with valid parameter combinations.
2. Generating natural-language descriptions via templates or LLMs.
3. Creating mock responses and defining expected behavior.

## Task Format

**Input prompt**:
```
Using the GitHub API, write Python code to:
1. List all open issues in the "pytorch/pytorch" repository
   that have the label "high priority"
2. For each issue, get the number of comments
3. Return a sorted list of (issue_title, comment_count) tuples,
   sorted by comment count descending

Use the requests library. The API base URL is
https://api.github.com and authentication token is in
the GITHUB_TOKEN environment variable.
```

**Expected output**: Python code that correctly constructs API calls with proper authentication, pagination handling, and response parsing.

**Verification**: Execute against a mock GitHub API server pre-loaded with test data; check that the correct endpoints were called with proper headers and the final output matches the expected sorted list.

## Difficulty Curriculum

| Level | Complexity | Example |
|-------|-----------|---------|
| 1 | Single GET request, no auth | Fetch weather data for a city |
| 2 | GET with authentication and parameters | Search GitHub repos with query params |
| 3 | POST/PUT with JSON body | Create a new resource with specific fields |
| 4 | Multi-step: read → process → write | Fetch data, transform, POST results |
| 5 | Pagination and error handling | Iterate all pages, handle 429 rate limits |
| 6 | Multi-API orchestration | Combine data from 3+ APIs with auth, pagination, and error handling |

## Limitations & Risks

- **API versioning**: APIs change over time. Mock servers must be versioned and kept in sync with the dataset. Stale API schemas cause false failures.
- **Non-determinism**: Real APIs may return results in different orders. Verification must normalize ordering where appropriate.
- **Rate limits and costs**: Using real APIs for RL training at scale is impractical without generous test accounts or mock servers.
- **Security**: Generated code might leak API keys, make destructive calls (DELETE), or exfiltrate data. Sandboxing and read-only API modes are essential.
- **Specification quality**: Many real APIs have incomplete or inaccurate OpenAPI specs. Verification against faulty specs causes false negatives.
- **Subjective correctness**: Some tasks ("find the best restaurant") involve preference, not just factual correctness. RLVR tasks should be scoped to objectively verifiable outcomes.

## Connections

- Generalizes **SQL Generation** (a SQL query is essentially an API call to a database).
- Directly relevant to **Infrastructure-as-Code** — Terraform/K8s configs ultimately invoke cloud APIs.
- **Shell Commands** overlap: curl-based API calls are shell commands.
- **Web Scraping** is the unstructured counterpart: when no API exists, scrape the web page instead.
- Core capability for agentic systems that combine **Code Generation** with tool use.
