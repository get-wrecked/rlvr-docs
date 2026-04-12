---
domain: Workflow Automation (IFTTT/Zapier-style)
category: agent-productivity
verification_type: simulation
dataset_scale: 100K+ (from automation platforms)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Workflow Automation (IFTTT/Zapier-style)

## Overview
Given a natural language description of an automation workflow ("When I receive an email with an attachment, save the attachment to Dropbox and notify me on Slack"), generate the correct trigger-action sequence using available service APIs. Verification: simulate the workflow against test events and check the correct actions are triggered.

## Verification Mechanism
```python
def verify(description: str, workflow: list[Step], test_events: list[Event]) -> float:
    simulator = WorkflowSimulator(available_services)
    
    passed = 0
    for event in test_events:
        # Run the workflow on the test event
        actions_taken = simulator.execute(workflow, event)
        expected_actions = compute_expected_actions(description, event)
        
        if actions_match(actions_taken, expected_actions):
            passed += 1
    
    return passed / len(test_events)

def actions_match(actual: list[Action], expected: list[Action]) -> bool:
    """Check if the right actions were triggered with the right parameters."""
    if len(actual) != len(expected):
        return False
    for a, e in zip(sorted(actual), sorted(expected)):
        if a.service != e.service or a.action_type != e.action_type:
            return False
        if not params_match(a.params, e.params):
            return False
    return True
```

## Dataset Sources
- **IFTTT dataset**: 300K+ IFTTT recipes from the platform.
- **Zapier template library**: Thousands of workflow templates.
- **Microsoft Power Automate templates**: Workflow templates.
- **n8n workflow templates**: Open-source automation templates.
- **Procedural generation**: Define services + triggers + actions, generate NL descriptions.

## Task Format
- **Input**: "When a new row is added to my Google Sheet named 'Orders', send a Slack message to #orders channel with the order details"
- **Output**: Workflow definition: Trigger(GoogleSheets, new_row, sheet="Orders") → Action(Slack, post_message, channel="#orders", text="{row_data}")

## Difficulty Curriculum
- Level 1: Single trigger → single action
- Level 3: Conditional logic (if/else in workflow)
- Level 5: Multi-step with data transformation
- Level 7: Complex branching, error handling, loops
- Level 9: Multi-service orchestration with timing constraints

## Limitations & Risks
- Service simulation may not perfectly match real APIs. Use mock services with defined behavior.
- Natural language descriptions of workflows are often ambiguous. Use precise test events to disambiguate.

## Connections
- [[natural-language-to-api]] — API call generation
- [[planning-classical]] — workflow as plan
- [[code-generation]] — workflows are programs
- [[calendar-scheduling]] — scheduling as a workflow component
