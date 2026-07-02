//! Greedy graph coloring matching networkx.greedy_color semantics exactly.
//!
//! Supported strategies and their nx-exact node ordering:
//!
//! - `largest_first`: stable descending sort by degree; ties preserve
//!   edge-list insertion order (mirrors Python's `sorted(..., reverse=True)`
//!   which is stable).
//!
//! - `saturation_largest_first` (DSATUR): pick the uncolored node with the
//!   highest saturation (distinct neighbor colors), breaking ties by degree,
//!   then by earliest insertion order. This matches nx's `max(saturation,
//!   key=lambda v: (saturation[v], G.degree(v)))`, where Python `max`
//!   returns the first maximum in dict-iteration order (= insertion order).
//!
//! `smallest_last` is intentionally omitted: its bucket-queue uses
//! `set.pop()`, whose order is CPython-hash-dependent for string nodes and
//! varies with `PYTHONHASHSEED`, making value-exact replication impossible
//! without embedding CPython's hash algorithm.
//!
//! The greedy step is identical in all cases: for each node in strategy
//! order, assign the smallest non-negative integer not used by any
//! already-colored neighbor.

use indexmap::IndexMap;

/// Which strategy to use when ordering nodes for coloring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    LargestFirst,
    SaturationLargestFirst,
}

impl Strategy {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "largest_first" => Some(Self::LargestFirst),
            "saturation_largest_first" | "DSATUR" => Some(Self::SaturationLargestFirst),
            _ => None,
        }
    }
}

/// The graph: nodes interned to `0..n` in first-seen order.
/// `names` maps intern index → original string name.
/// `adj` holds simple-graph adjacency (no self-edges, no parallel edges).
/// `selfloop[i]` records whether node `i` carries a self-loop; nx counts a
/// self-loop as +2 toward `G.degree`, which the ordering strategies observe.
pub struct Graph {
    pub names: Vec<String>,
    pub adj: Vec<Vec<usize>>,
    pub selfloop: Vec<bool>,
}

impl Graph {
    /// Parse an edge list where each line is `u v` (whitespace-separated).
    /// Lines starting with `#` or blank are skipped.
    /// Parallel edges are deduplicated. A self-loop (`u u`) registers the node
    /// with no self-adjacency — matching nx, which keeps the node and colors it
    /// (a self-loop never constrains a node's own color) while still counting
    /// the loop twice in `G.degree`.
    pub fn from_edge_list(input: &str) -> Self {
        // IndexMap preserves insertion order, mirrors nx.Graph dict internals.
        let mut node_index: IndexMap<String, usize> = IndexMap::new();
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut selfloop_idx: std::collections::HashSet<usize> = std::collections::HashSet::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.split_whitespace();
            let u = match parts.next() {
                Some(s) => s,
                None => continue,
            };
            let v = match parts.next() {
                Some(s) => s,
                None => continue, // single-token lines ignored
            };
            let n = node_index.len();
            let ui = *node_index.entry(u.to_owned()).or_insert(n);
            if u == v {
                selfloop_idx.insert(ui);
                continue;
            }
            let n = node_index.len();
            let vi = *node_index.entry(v.to_owned()).or_insert(n);
            edges.push((ui, vi));
        }

        let n = node_index.len();
        let names: Vec<String> = node_index.into_keys().collect();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut seen: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        for (u, v) in edges {
            let key = (u.min(v), u.max(v));
            if seen.insert(key) {
                adj[u].push(v);
                adj[v].push(u);
            }
        }

        let selfloop: Vec<bool> = (0..n).map(|i| selfloop_idx.contains(&i)).collect();
        Graph {
            names,
            adj,
            selfloop,
        }
    }

    pub fn node_count(&self) -> usize {
        self.names.len()
    }

    /// nx `G.degree`: incident edges, with a self-loop counted twice.
    pub fn degree(&self, node: usize) -> usize {
        self.adj[node].len() + if self.selfloop[node] { 2 } else { 0 }
    }
}

/// Run greedy coloring and return `node_name → color` in insertion order.
///
/// The returned `IndexMap` preserves the graph's node insertion order,
/// matching networkx's dict output ordering.
pub fn greedy_color(graph: &Graph, strategy: Strategy) -> IndexMap<String, usize> {
    let n = graph.node_count();
    if n == 0 {
        return IndexMap::new();
    }

    match strategy {
        Strategy::LargestFirst => greedy_largest_first(graph),
        Strategy::SaturationLargestFirst => greedy_saturation_largest_first(graph),
    }
}

fn assign_color(node: usize, colors: &[Option<usize>], adj: &[Vec<usize>]) -> usize {
    // Collect neighbor colors into a sorted vec, then find first gap.
    let mut nbr: Vec<usize> = adj[node].iter().filter_map(|&v| colors[v]).collect();
    nbr.sort_unstable();
    nbr.dedup();
    let mut color = 0usize;
    for c in nbr {
        if c == color {
            color += 1;
        } else {
            break;
        }
    }
    color
}

fn greedy_largest_first(graph: &Graph) -> IndexMap<String, usize> {
    let n = graph.node_count();
    // Stable descending sort by degree; ties keep insertion order.
    // Python: sorted(G, key=G.degree, reverse=True) — stable sort.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&b| std::cmp::Reverse(graph.degree(b)));

    let mut colors: Vec<Option<usize>> = vec![None; n];
    for node in order {
        colors[node] = Some(assign_color(node, &colors, &graph.adj));
    }

    // Output in graph insertion order (mirrors nx dict output).
    let mut result = IndexMap::with_capacity(n);
    for (i, name) in graph.names.iter().enumerate() {
        result.insert(name.clone(), colors[i].unwrap());
    }
    result
}

fn greedy_saturation_largest_first(graph: &Graph) -> IndexMap<String, usize> {
    let n = graph.node_count();
    // distinct_colors[v] = set of colors already seen among v's colored neighbors.
    let mut distinct: Vec<std::collections::HashSet<usize>> =
        (0..n).map(|_| std::collections::HashSet::new()).collect();
    let mut colors: Vec<Option<usize>> = vec![None; n];
    let mut colored_count = 0usize;

    // nx: if no pre-colored nodes, first pick = max(G, key=G.degree).
    // Python's built-in max() returns the FIRST maximum found when keys are equal
    // (it replaces only on strictly-greater, so ties keep the earliest-seen element).
    // Rust's max_by_key returns the LAST maximum (it replaces on greater-or-equal).
    // To match Python: iterate 0..n (insertion order) and keep the node only when
    // its key is strictly greater than the current best — ties fall through.
    {
        let first = (0..n)
            .reduce(|best, v| {
                if graph.degree(v) > graph.degree(best) {
                    v
                } else {
                    best
                }
            })
            .unwrap();
        let color = assign_color(first, &colors, &graph.adj);
        colors[first] = Some(color);
        colored_count += 1;
        for &nb in &graph.adj[first] {
            distinct[nb].insert(color);
        }
    }

    while colored_count < n {
        // nx re-computes saturation from the colors dict each iteration.
        // Since distinct[] is maintained incrementally and monotonically, this is equivalent.
        // Pick uncolored node with max (saturation, degree); ties → earliest insertion order.
        // nx: max(saturation, key=lambda v: (saturation[v], G.degree(v))) where saturation
        // iterates in G's insertion order. Python max() keeps the FIRST maximum (earliest node).
        let node = (0..n)
            .filter(|&v| colors[v].is_none())
            .reduce(|best, v| {
                let key_v = (distinct[v].len(), graph.degree(v));
                let key_b = (distinct[best].len(), graph.degree(best));
                if key_v > key_b { v } else { best }
            })
            .unwrap();

        let color = assign_color(node, &colors, &graph.adj);
        colors[node] = Some(color);
        colored_count += 1;
        for &nb in &graph.adj[node] {
            distinct[nb].insert(color);
        }
    }

    let mut result = IndexMap::with_capacity(n);
    for (i, name) in graph.names.iter().enumerate() {
        result.insert(name.clone(), colors[i].unwrap());
    }
    result
}
