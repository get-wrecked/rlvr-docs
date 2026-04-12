---
domain: Natural Language to API Calls
category: tool-use
verification_type: execution
dataset_scale: 100K+ (ToolBench, API documentation)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Natural Language to API Calls

## Overview
Given a natural language instruction and API documentation, generate the correct API call (or sequence of calls) to accomplish the task. Verification: execute the call against a mock/real API and check the response matches expectations.

## Verification Mechanism
```python
def verify(instruction: str, api_call: dict, mock_api: MockAPI) -> float:
    # Parse the API call
    try:
        endpoint = api_call["endpoint"]
        method = api_call["method"]
        params = api_call.get("params", {})
        body = api_call.get("body", {})
    except KeyError:
        return 0.0
    
    # Check endpoint exists
    if not mock_api.has_endpoint(method, endpoint):
        return 0.0
    
    # Check required parameters
    required = mock_api.get_required_params(method, endpoint)
    if not all(p in params or p in body for p in required):
        return 0.0
    
    # Execute and check response
    response = mock_api.execute(method, endpoint, params=params, body=body)
    
    if response.status_code == 200:
        # Check response matches expected outcome
        expected = get_expected_outcome(instruction)
        if outcome_matches(response.json(), expected):
            return 1.0
        return 0.5  # Valid call but wrong outcome
    
    return 0.0  # API error

# For multi-step API sequences:
def verify_sequence(instruction: str, api_calls: list[dict], mock_api: MockAPI) -> float:
    state = mock_api.initial_state()
    for call in api_calls:
        response = mock_api.execute_stateful(call, state)
        if response.status_code >= 400:
            return 0.0
        state = mock_api.update_state(state, response)
    
    return 1.0 if mock_api.goal_reached(state, instruction) else 0.0
```

## Dataset Sources
- **ToolBench**: 16K+ real-world APIs with instructions and solutions.
- **API-Bank**: API usage benchmark.
- **Gorilla**: LLM API usage benchmark.
- **RestGPT**: REST API task completion.
- **ToolAlpaca**: Simulated tool use dataset.
- **RapidAPI Hub**: 50K+ public APIs — generate tasks from documentation.
- **OpenAPI/Swagger specs**: Thousands of public API specifications.
- **Postman collections**: Shared API request collections.

## Task Format
- **Input**: "Using the GitHub API, create a new issue titled 'Bug report' in repo user/project with label 'bug'" + API documentation
- **Output**: `POST /repos/user/project/issues {"title": "Bug report", "labels": ["bug"]}`

## Difficulty Curriculum
- Level 1: Single API call, simple parameters
- Level 3: Multi-step API calls with data flow between steps
- Level 5: Error handling, pagination, authentication flows
- Level 7: Complex multi-API orchestration
- Level 9: API design (given requirements, design the API spec itself)

## Limitations & Risks
- Mock APIs may not perfectly replicate real API behavior. Use real API responses as test data.
- API documentation quality varies wildly. Include documentation quality as a variable.
- Multi-step verification requires careful state management.
- Rate limiting and authentication complicate real API testing.

## Connections
- [[code-generation]] — API calls are code
- [[web-navigation]] — browser-based API interaction
- [[structured-output-generation]] — API calls are structured (JSON)
- [[shell-commands]] — CLI tool invocation is similar
