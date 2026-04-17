//! Graph Property Verifier
//!
//! Verifies graph algorithm answers: shortest paths, MST weight, connected components,
//! topological sort, graph coloring, etc. Covers: graph-theory, graph-algorithm-execution,
//! network-analysis.
//!
//! Uses adjacency list representation. No external graph library needed.

use super::VerifyResult;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// A simple weighted graph (adjacency list).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Graph {
    pub directed: bool,
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String, f64)>, // (from, to, weight)
}

impl Graph {
    /// Get adjacency list.
    fn adjacency(&self) -> HashMap<&str, Vec<(&str, f64)>> {
        let mut adj: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
        for node in &self.nodes {
            adj.entry(node).or_default();
        }
        for (u, v, w) in &self.edges {
            adj.entry(u).or_default().push((v, *w));
            if !self.directed {
                adj.entry(v).or_default().push((u, *w));
            }
        }
        adj
    }

    /// Check if graph has any negative edge weights.
    pub fn has_negative_weights(&self) -> bool {
        self.edges.iter().any(|(_, _, w)| *w < 0.0)
    }

    /// Dijkstra's shortest path (requires non-negative weights).
    /// Returns None if no path exists OR if graph has negative weights.
    pub fn shortest_path_length(&self, source: &str, target: &str) -> Option<f64> {
        if self.has_negative_weights() {
            return None; // Dijkstra is invalid for negative weights
        }
        let adj = self.adjacency();
        let mut dist: HashMap<&str, f64> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(OrderedFloat, &str)>> = BinaryHeap::new();

        dist.insert(source, 0.0);
        heap.push(Reverse((OrderedFloat(0.0), source)));

        while let Some(Reverse((OrderedFloat(d), u))) = heap.pop() {
            if u == target {
                return Some(d);
            }
            if d > *dist.get(u).unwrap_or(&f64::INFINITY) {
                continue;
            }
            if let Some(neighbors) = adj.get(u) {
                for &(v, w) in neighbors {
                    let new_dist = d + w;
                    if new_dist < *dist.get(v).unwrap_or(&f64::INFINITY) {
                        dist.insert(v, new_dist);
                        heap.push(Reverse((OrderedFloat(new_dist), v)));
                    }
                }
            }
        }

        dist.get(target).copied()
    }

    /// Number of connected components (undirected).
    pub fn num_components(&self) -> usize {
        let adj = self.adjacency();
        let mut visited: HashSet<&str> = HashSet::new();
        let mut count = 0;

        for node in &self.nodes {
            if !visited.contains(node.as_str()) {
                count += 1;
                let mut queue = VecDeque::new();
                queue.push_back(node.as_str());
                while let Some(u) = queue.pop_front() {
                    if visited.insert(u) {
                        if let Some(neighbors) = adj.get(u) {
                            for &(v, _) in neighbors {
                                if !visited.contains(v) {
                                    queue.push_back(v);
                                }
                            }
                        }
                    }
                }
            }
        }

        count
    }

    /// Check if a topological sort is valid (for DAGs).
    pub fn is_valid_topo_sort(&self, order: &[String]) -> bool {
        if !self.directed {
            return false;
        }
        let position: HashMap<&str, usize> = order
            .iter()
            .enumerate()
            .map(|(i, n)| (n.as_str(), i))
            .collect();

        // Every edge (u, v) must have position[u] < position[v]
        for (u, v, _) in &self.edges {
            let pu = match position.get(u.as_str()) {
                Some(&p) => p,
                None => return false,
            };
            let pv = match position.get(v.as_str()) {
                Some(&p) => p,
                None => return false,
            };
            if pu >= pv {
                return false;
            }
        }

        true
    }

    /// Check if a graph coloring is valid (no adjacent nodes share a color).
    pub fn is_valid_coloring(&self, coloring: &HashMap<String, usize>) -> bool {
        for (u, v, _) in &self.edges {
            let cu = coloring.get(u);
            let cv = coloring.get(v);
            if let (Some(cu), Some(cv)) = (cu, cv) {
                if cu == cv {
                    return false;
                }
            } else {
                return false; // Missing color for a node
            }
        }
        true
    }

    /// Total edge weight (for MST verification).
    pub fn total_weight(&self) -> f64 {
        self.edges.iter().map(|(_, _, w)| w).sum()
    }
}

/// Wrapper for f64 that implements Ord for BinaryHeap.
#[derive(Debug, Clone, Copy, PartialEq)]
struct OrderedFloat(f64);
impl Eq for OrderedFloat {}
impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// Verify a graph algorithm answer.
pub fn verify(graph_json: &str, task: &str, answer_json: &str) -> VerifyResult {
    let graph: Graph = match serde_json::from_str(graph_json) {
        Ok(g) => g,
        Err(e) => return VerifyResult::wrong(format!("invalid graph: {e}")),
    };

    let answer: serde_json::Value = match serde_json::from_str(answer_json) {
        Ok(a) => a,
        Err(e) => return VerifyResult::wrong(format!("invalid answer: {e}")),
    };

    match task {
        "shortest_path" => {
            let source = answer["source"].as_str().unwrap_or("");
            let target = answer["target"].as_str().unwrap_or("");
            let proposed = answer["length"].as_f64().unwrap_or(f64::NAN);

            match graph.shortest_path_length(source, target) {
                Some(correct) if (proposed - correct).abs() < 1e-6 => VerifyResult::correct(),
                Some(correct) => VerifyResult::wrong(format!("expected {correct}, got {proposed}")),
                None => {
                    if proposed.is_infinite() || proposed.is_nan() {
                        VerifyResult::correct() // No path exists
                    } else {
                        VerifyResult::wrong("no path exists")
                    }
                }
            }
        }
        "num_components" => {
            let proposed = answer.as_u64().unwrap_or(0) as usize;
            let correct = graph.num_components();
            if proposed == correct {
                VerifyResult::correct()
            } else {
                VerifyResult::wrong(format!("expected {correct}, got {proposed}"))
            }
        }
        "topological_sort" => {
            let order: Vec<String> = answer
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            if graph.is_valid_topo_sort(&order) {
                VerifyResult::correct()
            } else {
                VerifyResult::wrong("invalid topological order")
            }
        }
        "coloring" => {
            let coloring: HashMap<String, usize> = answer
                .as_object()
                .map(|o| {
                    o.iter()
                        .filter_map(|(k, v)| v.as_u64().map(|c| (k.clone(), c as usize)))
                        .collect()
                })
                .unwrap_or_default();

            if graph.is_valid_coloring(&coloring) {
                let num_colors = coloring.values().collect::<HashSet<_>>().len();
                VerifyResult::partial(1.0, format!("valid {num_colors}-coloring"))
            } else {
                VerifyResult::wrong("adjacent nodes share a color")
            }
        }
        _ => VerifyResult::wrong(format!("unknown task: {task}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_graph() -> String {
        serde_json::json!({
            "directed": false,
            "nodes": ["A", "B", "C", "D"],
            "edges": [["A", "B", 1.0], ["B", "C", 2.0], ["A", "C", 4.0], ["C", "D", 1.0]]
        })
        .to_string()
    }

    fn dag() -> String {
        serde_json::json!({
            "directed": true,
            "nodes": ["A", "B", "C", "D"],
            "edges": [["A", "B", 1.0], ["A", "C", 1.0], ["B", "D", 1.0], ["C", "D", 1.0]]
        })
        .to_string()
    }

    fn disconnected_graph() -> String {
        serde_json::json!({
            "directed": false,
            "nodes": ["A", "B", "C", "D"],
            "edges": [["A", "B", 1.0], ["C", "D", 1.0]]
        })
        .to_string()
    }

    // ========== Shortest Path ==========

    #[test]
    fn shortest_path_direct() {
        let g = simple_graph();
        let answer = r#"{"source": "A", "target": "B", "length": 1.0}"#;
        assert_eq!(verify(&g, "shortest_path", answer).score, 1.0);
    }

    #[test]
    fn shortest_path_indirect() {
        // A→C directly is 4.0, but A→B→C is 3.0
        let g = simple_graph();
        let answer = r#"{"source": "A", "target": "C", "length": 3.0}"#;
        assert_eq!(verify(&g, "shortest_path", answer).score, 1.0);

        let wrong = r#"{"source": "A", "target": "C", "length": 4.0}"#;
        assert_eq!(verify(&g, "shortest_path", wrong).score, 0.0);
    }

    #[test]
    fn shortest_path_multi_hop() {
        // A→D: A→B(1)→C(2)→D(1) = 4
        let g = simple_graph();
        let answer = r#"{"source": "A", "target": "D", "length": 4.0}"#;
        assert_eq!(verify(&g, "shortest_path", answer).score, 1.0);
    }

    // ========== Connected Components ==========

    #[test]
    fn one_component() {
        let g = simple_graph();
        assert_eq!(verify(&g, "num_components", "1").score, 1.0);
    }

    #[test]
    fn two_components() {
        let g = disconnected_graph();
        assert_eq!(verify(&g, "num_components", "2").score, 1.0);
        assert_eq!(verify(&g, "num_components", "1").score, 0.0);
    }

    // ========== Topological Sort ==========

    #[test]
    fn valid_topo_sort() {
        let g = dag();
        let answer = r#"["A", "B", "C", "D"]"#;
        assert_eq!(verify(&g, "topological_sort", answer).score, 1.0);

        // Also valid: A, C, B, D
        let answer2 = r#"["A", "C", "B", "D"]"#;
        assert_eq!(verify(&g, "topological_sort", answer2).score, 1.0);
    }

    #[test]
    fn invalid_topo_sort() {
        let g = dag();
        let answer = r#"["D", "B", "C", "A"]"#; // D before A — wrong
        assert_eq!(verify(&g, "topological_sort", answer).score, 0.0);
    }

    // ========== Graph Coloring ==========

    #[test]
    fn valid_coloring() {
        let g = simple_graph();
        let answer = r#"{"A": 0, "B": 1, "C": 2, "D": 0}"#;
        assert_eq!(verify(&g, "coloring", answer).score, 1.0);
    }

    #[test]
    fn invalid_coloring() {
        let g = simple_graph();
        let answer = r#"{"A": 0, "B": 0, "C": 1, "D": 0}"#; // A-B both 0, but they're adjacent
        assert_eq!(verify(&g, "coloring", answer).score, 0.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_graphs() {
        let g1 = simple_graph();
        let g2 = disconnected_graph();

        assert_eq!(verify(&g1, "num_components", "1").score, 1.0);
        assert_eq!(verify(&g1, "num_components", "2").score, 0.0);

        assert_eq!(verify(&g2, "num_components", "2").score, 1.0);
        assert_eq!(verify(&g2, "num_components", "1").score, 0.0);
    }

    // ========== ADVERSARIAL ==========

    #[test]
    fn adversarial_self_loop_coloring() {
        let g = serde_json::json!({
            "directed": false,
            "nodes": ["A", "B"],
            "edges": [["A", "A", 1.0], ["A", "B", 2.0]]
        })
        .to_string();
        let coloring = r#"{"A": 0, "B": 1}"#;
        let result = verify(&g, "coloring", coloring);
        assert_eq!(result.score, 0.0, "Self-loop means A conflicts with itself");
    }

    #[test]
    fn adversarial_disconnected_no_path() {
        let g = serde_json::json!({
            "directed": false,
            "nodes": ["A", "B", "C", "D"],
            "edges": [["A", "B", 1.0], ["C", "D", 1.0]]
        })
        .to_string();
        let answer = r#"{"source": "A", "target": "D", "length": 99.0}"#;
        let result = verify(&g, "shortest_path", answer);
        assert_eq!(
            result.score, 0.0,
            "No path exists — any length claim is wrong"
        );
    }

    #[test]
    fn adversarial_empty_graph_components() {
        let g = serde_json::json!({
            "directed": false,
            "nodes": ["A", "B", "C"],
            "edges": []
        })
        .to_string();
        assert_eq!(verify(&g, "num_components", "3").score, 1.0);
        assert_eq!(verify(&g, "num_components", "1").score, 0.0);
    }
}
