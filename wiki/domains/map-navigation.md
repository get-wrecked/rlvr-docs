---
domain: map-navigation
category: agent/navigation
verification_type: constraint | rule
dataset_scale: >10K tasks (benchmarks); unlimited (OSM-based generation)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Map Navigation

## Overview

Map navigation tasks require an agent to navigate maps, provide directions, plan routes, and answer geographic questions. Tasks range from simple wayfinding ("How do I get from A to B?") to complex route planning with constraints ("Find the shortest route visiting all three stores that avoids highways and arrives by 5 PM"). Verification is clean for route tasks: check that the route is valid (follows real roads), connects the required waypoints, and satisfies constraints (distance, time, road type). The domain is practically valuable and benefits from the enormous open-source geographic dataset OpenStreetMap. It tests spatial reasoning, constraint satisfaction, and real-world knowledge.

## Verification Mechanism

1. **Route validity checking** (rule-based): Verify that a proposed route follows actual roads/paths in the map data. Each segment must correspond to a real road. Check via OSM graph traversal: every pair of consecutive waypoints must be connected by a navigable road segment.
2. **Route connectivity** (rule-based): Verify the route connects the start and end points (and any required waypoints). Graph reachability check.
3. **Route optimality** (constraint-based): Compare route distance/time to the optimal route computed by a routing engine (OSRM, GraphHopper, Valhalla). Reward = 1 if agent route is within X% of optimal, 0 otherwise. Can use continuous reward based on ratio.
4. **Constraint satisfaction** (constraint-based): Verify all route constraints are met: avoids highways, stays under distance limit, passes through required waypoints in order, arrives before deadline. Each constraint is individually checkable.
5. **Geographic answer match** (exact match): For geographic QA ("What is the nearest hospital to point X?"), verify against OSM POI data. Exact match on the correct POI.
6. **Direction following** (simulation-based): For instruction-following navigation (TOUCHDOWN), simulate following the agent's directions on the map and check if the destination is reached. Compare final position to goal within a distance threshold.

## Dataset Sources

- **OpenStreetMap (OSM)**: https://www.openstreetmap.org/ — Complete open-source map of the world. Road networks, POIs, building footprints, land use. The foundational data source for generating unlimited navigation tasks. Download via Geofabrik, Overpass API.
- **OSRM (Open Source Routing Machine)**: https://project-osrm.org/ — Fast routing engine for OSM data. Provides optimal routes for verification.
- **GraphHopper**: https://www.graphhopper.com/ — Another OSM-based routing engine with flexible profiles.
- **TOUCHDOWN**: https://github.com/lil-lab/touchdown — 9.3K navigation instructions in Google Street View of Manhattan. Agent must follow natural-language directions to reach a goal location. Verification: distance to goal < threshold.
- **StreetNav**: Street-level navigation following natural language directions.
- **ALFRED/Room-to-Room (R2R)**: https://bringmeaspoon.org/ — 21K navigation instructions in indoor 3D environments (Matterport3D). Not map-based but related.
- **Talk2Nav**: Natural language navigation instructions on OpenStreetMap.
- **Map2Seq**: Sequence-based navigation on maps with natural language instructions.
- **Nominatim**: https://nominatim.openstreetmap.org/ — Geocoding service for OSM. Convert addresses to coordinates and vice versa.
- **Overpass Turbo**: https://overpass-turbo.eu/ — Query OSM data for specific features (all restaurants in a city, all parks near a point).

## Task Format

**Route planning**:
- Input: Start location + end location + optional constraints (avoid highways, prefer scenic routes, stay under 30 minutes). Map context (area name, or OSM data excerpt).
- Output: Sequence of road names / turn-by-turn directions, or sequence of coordinates.
- Verification: Route is valid (follows roads), connects start to end, satisfies constraints, compared to optimal via routing engine.

**Wayfinding QA**:
- Input: Map image or description + question (e.g., "What is the fastest way from the library to the train station?").
- Output: Route description or distance/time estimate.
- Verification: Compare to routing engine result.

**Instruction following (TOUCHDOWN-style)**:
- Input: Street-level images + natural-language navigation instruction (e.g., "Walk straight until you see a yellow building on your left. Turn right at the next intersection.").
- Output: Sequence of actions (forward, turn left, turn right, stop).
- Verification: Final position within threshold distance of goal.

**POI queries**:
- Input: Location + query (e.g., "Find the three closest pharmacies to this address").
- Output: List of POIs with distances.
- Verification: Compare against OSM POI data. Check correctness of POIs and ordering.

**Traveling Salesman-style tasks**:
- Input: Set of locations to visit + constraints (minimize total distance, visit in a specific order).
- Output: Ordered route.
- Verification: Route is valid, visits all required locations, satisfies constraints. Compare total distance to known good solutions.

## Difficulty Curriculum

1. **Simple A-to-B routing** (easy): Find a route between two nearby points. One reasonable path.
2. **Avoid/prefer constraints** (easy-medium): Route from A to B avoiding highways, or preferring bike paths.
3. **POI finding** (medium): Find nearest POI of a given type. Requires map querying.
4. **Multi-waypoint routing** (medium): Visit 3-5 locations in a reasonable order.
5. **Time-constrained routing** (medium-hard): Arrive by deadline, accounting for traffic/distance.
6. **Instruction following, simple** (medium): Follow 5-10 step directions in a clear area.
7. **Instruction following, complex** (hard): Follow ambiguous or long directions in complex urban areas. TOUCHDOWN level.
8. **Constrained TSP** (hard): Optimize route across 5-10 locations with multiple constraints.
9. **Multi-modal transit planning** (very hard): Plan routes using walking, bus, subway, taxi. Requires understanding transit schedules and transfers.
10. **Dynamic routing** (very hard): Replan routes based on changing conditions (road closures, traffic).

## Limitations & Risks

- **OSM data quality varies**: OSM coverage is excellent in developed countries but sparse in some regions. Missing roads, outdated POIs, and incorrect tags are common.
- **Real-time factors not captured**: Traffic, construction, seasonal road closures, and weather are not in static map data. Route optimality verification assumes static conditions.
- **Ambiguous natural language**: "Nearby", "quick", "scenic" are subjective. Verification must operationalize these terms with specific thresholds.
- **Instruction following is hard to verify**: For TOUCHDOWN-style tasks, the verification is coarse (distance to goal). The agent might reach the goal via an entirely different route than intended.
- **Map rendering for vision models**: If the input is a map image, the model must parse roads, labels, and symbols. Map readability varies enormously with zoom level, style, and density.
- **Coordinate precision**: LLMs may struggle with precise geographic coordinates. Small errors in coordinates can mean large real-world distances.
- **Scale limitation of benchmarks**: TOUCHDOWN has ~9K examples in one city (Manhattan). Generating diverse navigation tasks at scale across many cities is feasible with OSM but requires significant engineering.

## Connections

- [[gui-navigation]] — Map app navigation is a specific GUI navigation task
- [[web-navigation]] — Web-based map interaction combines both domains
- [[spatial-reasoning]] — Navigation fundamentally requires spatial reasoning
- [[puzzle-games]] — Route optimization shares problem structure with combinatorial puzzles
- [[calendar-scheduling]] — Time-constrained routing connects to scheduling problems
