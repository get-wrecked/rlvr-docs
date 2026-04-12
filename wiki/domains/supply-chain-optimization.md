---
domain: Supply Chain & Inventory Optimization
category: operations-research
verification_type: execution
dataset_scale: 50K+ (from OR benchmarks + competitions)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Supply Chain & Inventory Optimization

## Overview
Optimize supply chain decisions: determine order quantities (EOQ, newsvendor), design distribution networks, set safety stock levels, plan production schedules, optimize vehicle routing. Verification: compute total cost/service level against known OR solutions.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, solution: dict) -> float:
    if task_type == "eoq":
        correct = np.sqrt(2 * problem["demand"] * problem["order_cost"] / problem["holding_cost"])
        return 1.0 if abs(solution["Q"] - correct) / correct < 0.01 else 0.0
    
    elif task_type == "newsvendor":
        from scipy.stats import norm
        z = norm.ppf(problem["cu"] / (problem["cu"] + problem["co"]))
        correct_Q = problem["mean"] + z * problem["std"]
        return 1.0 if abs(solution["Q"] - correct_Q) / correct_Q < 0.01 else 0.0
    
    elif task_type == "vehicle_routing":
        # Check route feasibility (capacity, time windows)
        for route in solution["routes"]:
            load = sum(problem["demands"][c] for c in route)
            if load > problem["vehicle_capacity"]:
                return 0.0
        # Compute total distance
        total_dist = compute_total_distance(solution["routes"], problem["distances"])
        best_known = problem.get("best_known", total_dist)
        return min(1.0, best_known / total_dist)
    
    elif task_type == "safety_stock":
        correct = compute_safety_stock(problem["service_level"], problem["demand_std"], problem["lead_time"])
        return 1.0 if abs(solution["safety_stock"] - correct) / correct < 0.05 else 0.0
```

## Dataset Sources
- **CVRPLIB**: Vehicle routing benchmark instances (1K+ instances).
- **Solomon benchmarks**: VRPTW (time windows) instances.
- **OR-Library**: Facility location, warehouse, logistics.
- **M5 Competition**: Retail inventory data.
- **Supply chain textbook exercises**: Chopra & Meindl, Simchi-Levi.
- **Kaggle supply chain datasets**: Demand forecasting, inventory data.

## Task Format
- **Input**: "A retailer sells 10,000 units/year. Ordering cost is $50/order, holding cost is $2/unit/year. What is the optimal order quantity?"
- **Output**: "EOQ = 707 units"

## Difficulty Curriculum
- Level 1: Basic EOQ, single-product inventory
- Level 3: Newsvendor, multi-product inventory
- Level 5: Vehicle routing (CVRP), facility location
- Level 7: Multi-echelon inventory, stochastic demand
- Level 9: End-to-end supply chain network design

## Connections
- [[scheduling]] — production scheduling
- [[combinatorics-optimization]] — routing and allocation
- [[mathematical-programming]] — LP/IP formulations
- [[time-series-forecasting]] — demand forecasting feeds inventory
