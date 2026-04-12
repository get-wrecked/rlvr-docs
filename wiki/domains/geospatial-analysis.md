---
domain: Geospatial Analysis & GIS
category: data-science
verification_type: execution
dataset_scale: massive (OpenStreetMap, satellite imagery)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Geospatial Analysis & GIS

## Overview
Geospatial tasks with verifiable answers: compute distances/areas on the globe, route optimization, spatial queries (what's within 5km of X?), map feature extraction from satellite imagery, coordinate transformations. Verification via computation against geographic databases.

## Verification Mechanism
```python
from shapely.geometry import Point, Polygon
import geopandas as gpd
from geopy.distance import geodesic

def verify(task_type: str, query: dict, answer: Any) -> float:
    if task_type == "distance":
        correct = geodesic(query["point_a"], query["point_b"]).km
        return 1.0 if abs(answer - correct) / correct < 0.01 else 0.0
    
    elif task_type == "spatial_query":
        # "Which countries does the Amazon River pass through?"
        gdf = gpd.read_file("countries.shp")
        river = gpd.read_file("rivers.shp").query(f"name == '{query['feature']}'")
        result = gpd.sjoin(gdf, river, how="inner")["name"].tolist()
        return set_f1(set(answer), set(result))
    
    elif task_type == "area":
        polygon = Polygon(query["coordinates"])
        correct_area = polygon.area  # in appropriate CRS
        return 1.0 if abs(answer - correct_area) / correct_area < 0.05 else 0.0
    
    elif task_type == "route":
        # Check route validity and optimality
        route_length = compute_route_length(answer["waypoints"])
        optimal = compute_optimal_route(query["start"], query["end"])
        return min(1.0, optimal / route_length)
    
    elif task_type == "geocoding":
        correct_coords = geocode(query["address"])
        return 1.0 if geodesic(answer, correct_coords).km < 0.1 else 0.0
```

## Dataset Sources
- **OpenStreetMap**: Complete world map data. Freely downloadable.
- **Natural Earth**: Public domain map dataset.
- **GeoNames**: 11M+ place names with coordinates.
- **Overpass Turbo / Nominatim**: OSM query APIs.
- **USGS Earth Explorer**: Satellite imagery.
- **Copernicus**: European satellite data (Sentinel-2, etc.).
- **GIS StackExchange**: Q&A with geospatial problems.
- **Census TIGER/Line**: US boundary files.

## Task Format
- **Input**: "What is the shortest driving distance from Berlin to Prague?" or "List all cities with population > 1M within 500km of Paris"
- **Output**: Numerical answer or list of entities

## Difficulty Curriculum
- Level 1: Point-to-point distance, basic geocoding
- Level 3: Spatial joins, buffer queries, area computation
- Level 5: Multi-stop route optimization
- Level 7: Complex spatial analysis (accessibility maps, viewshed analysis)
- Level 9: Satellite image interpretation + geospatial reasoning

## Limitations & Risks
- Map data changes over time. Pin to specific OSM/data snapshots.
- Driving distance depends on routing algorithm and road data freshness.
- Satellite imagery interpretation adds vision complexity.

## Connections
- [[map-navigation]] — navigation as application
- [[sql-generation]] — spatial SQL (PostGIS)
- [[data-wrangling]] — geospatial data processing
