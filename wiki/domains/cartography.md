---
domain: Cartography & Map Rendering
category: geospatial
verification_type: constraint
dataset_scale: massive (OpenStreetMap)
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Cartography & Map Rendering

## Overview
Generate maps and cartographic outputs: project geographic data onto specified projections, label placement without overlaps, thematic mapping (choropleth, proportional symbols), and map generalization. Verification: check topological correctness, label legibility, projection accuracy.

## Verification Mechanism
```python
def verify(task_type: str, map_output: Any, specs: dict) -> float:
    if task_type == "projection":
        # Check coordinates are correctly projected
        for test_point in specs["test_points"]:
            projected = extract_point_from_map(map_output, test_point["label"])
            expected = project(test_point["lat_lon"], specs["projection"])
            if distance(projected, expected) > specs["tolerance"]:
                return 0.0
        return 1.0
    
    elif task_type == "label_placement":
        labels = extract_labels(map_output)
        score = 0
        checks = 0
        
        # No overlaps
        checks += 1
        overlaps = count_overlaps(labels)
        if overlaps == 0:
            score += 1
        
        # All required labels present
        for req in specs["required_labels"]:
            checks += 1
            if req in [l["text"] for l in labels]:
                score += 1
        
        # Labels within map bounds
        checks += 1
        if all(in_bounds(l, specs["bounds"]) for l in labels):
            score += 1
        
        return score / checks
    
    elif task_type == "choropleth":
        # Check color mapping matches data values
        regions = extract_colored_regions(map_output)
        for region, value in specs["data"].items():
            expected_color = specs["color_scale"](value)
            actual_color = regions.get(region)
            if not color_close(actual_color, expected_color):
                return 0.0
        return 1.0
```

## Dataset Sources
- **OpenStreetMap**: Complete global map data.
- **Natural Earth**: Simplified world boundary data.
- **USGS National Map**: US topographic data.
- **Mapbox/Maplibre style specs**: Map styling examples.
- **Cartography textbook exercises**: Slocum, Robinson.
- **D3.js geo examples**: Hundreds of map visualization examples.

## Task Format
- **Input**: "Create a choropleth map of US states colored by population density, using Albers equal-area projection"
- **Output**: Code/SVG that produces the map

## Difficulty Curriculum
- Level 1: Simple point/polygon rendering on standard projections
- Level 3: Choropleth maps, label placement
- Level 5: Multi-layer maps, custom projections
- Level 7: Cartographic generalization, flow maps
- Level 9: Interactive cartographic visualizations

## Connections
- [[geospatial-analysis]] — geospatial computation
- [[data-visualization]] — visualization in general
- [[svg-generation]] — SVG map output
