use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::edge::{Edge, Relation};
use crate::graph::node::Node;
use crate::graph::query::{Direction, Query, QueryResult};
use crate::identity::UorAddress;

/// Execute a query against graph data. Pure function.
pub fn execute(
    query: &Query,
    nodes: &[Node],
    edges: &[Edge],
) -> QueryResult {
    let filtered = filter_edges(edges, &query.relation_filter);
    let adj = build_adj_index(&filtered, &query.direction);
    let (reachable, paths) = bfs_paths(
        query.target,
        &adj,
        query.max_depth,
        query.min_confidence,
    );

    let node_set: HashSet<UorAddress> =
        reachable.iter().copied().collect();
    let result_nodes: Vec<Node> = nodes
        .iter()
        .filter(|n| node_set.contains(&n.address))
        .cloned()
        .collect();
    let result_edges: Vec<Edge> = filtered
        .into_iter()
        .filter(|e| {
            node_set.contains(&e.source)
                && node_set.contains(&e.target)
        })
        .cloned()
        .collect();

    let confidence = if paths.is_empty() {
        0.0
    } else {
        compute_best_path_confidence(&paths, edges)
    };

    QueryResult {
        nodes: result_nodes,
        edges: result_edges,
        paths,
        confidence,
    }
}

type AdjEntry = (UorAddress, f64, Relation);
type AdjIndex = HashMap<UorAddress, Vec<AdjEntry>>;

fn build_adj_index(
    edges: &[&Edge],
    direction: &Direction,
) -> AdjIndex {
    let mut adj: AdjIndex = HashMap::new();
    for edge in edges {
        match direction {
            Direction::Downstream => {
                adj.entry(edge.source)
                    .or_default()
                    .push((
                        edge.target,
                        edge.confidence,
                        edge.relation.clone(),
                    ));
            }
            Direction::Upstream => {
                adj.entry(edge.target)
                    .or_default()
                    .push((
                        edge.source,
                        edge.confidence,
                        edge.relation.clone(),
                    ));
            }
            Direction::Both => {
                adj.entry(edge.source)
                    .or_default()
                    .push((
                        edge.target,
                        edge.confidence,
                        edge.relation.clone(),
                    ));
                adj.entry(edge.target)
                    .or_default()
                    .push((
                        edge.source,
                        edge.confidence,
                        edge.relation.clone(),
                    ));
            }
        }
    }
    adj
}

fn bfs_paths(
    start: UorAddress,
    adj: &AdjIndex,
    max_depth: u32,
    min_confidence: f64,
) -> (Vec<UorAddress>, Vec<Vec<UorAddress>>) {
    let mut visited = HashSet::new();
    let mut queue: VecDeque<(UorAddress, Vec<UorAddress>, f64)> =
        VecDeque::new();
    let mut reachable = Vec::new();
    let mut paths = Vec::new();

    visited.insert(start);
    reachable.push(start);
    queue.push_back((start, vec![start], 1.0));

    while let Some((node, path, conf)) = queue.pop_front() {
        let depth = path.len() - 1; // path includes start
        if depth >= max_depth as usize {
            continue;
        }
        if let Some(neighbors) = adj.get(&node) {
            for (next, edge_conf, _) in neighbors {
                let new_conf = conf * edge_conf;
                if new_conf < min_confidence {
                    continue;
                }
                if visited.insert(*next) {
                    reachable.push(*next);
                    let mut new_path = path.clone();
                    new_path.push(*next);
                    if new_path.len() > 1 {
                        paths.push(new_path.clone());
                    }
                    queue.push_back((*next, new_path, new_conf));
                }
            }
        }
    }

    (reachable, paths)
}

fn filter_edges<'a>(
    edges: &'a [Edge],
    filter: &Option<Vec<Relation>>,
) -> Vec<&'a Edge> {
    match filter {
        Some(relations) => edges
            .iter()
            .filter(|e| relations.contains(&e.relation))
            .collect(),
        None => edges.iter().collect(),
    }
}

fn compute_best_path_confidence(
    paths: &[Vec<UorAddress>],
    _edges: &[Edge],
) -> f64 {
    if paths.is_empty() {
        return 0.0;
    }
    // For now, return 1.0 for simple reachability
    // Full path confidence requires edge lookup per path step
    1.0
}
