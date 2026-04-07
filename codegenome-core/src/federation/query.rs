use std::collections::{HashMap, VecDeque};

use crate::federation::workspace::WorkspaceGraph;
use crate::graph::edge::Edge;
use crate::identity::UorAddress;

pub fn trace_between(graph: &WorkspaceGraph, from_repo: &str, to_repo: &str) -> Vec<Edge> {
    let by_name: HashMap<_, _> = graph
        .repositories
        .iter()
        .map(|r| (r.name.as_str(), r.node))
        .collect();
    let (Some(start), Some(goal)) = (by_name.get(from_repo), by_name.get(to_repo)) else {
        return Vec::new();
    };
    bfs(*start, *goal, &graph.federated_edges)
}

fn bfs(start: UorAddress, goal: UorAddress, edges: &[Edge]) -> Vec<Edge> {
    let mut queue = VecDeque::from([start]);
    let mut prev: HashMap<UorAddress, Edge> = HashMap::new();
    while let Some(node) = queue.pop_front() {
        if node == goal {
            break;
        }
        for edge in edges.iter().filter(|e| e.source == node) {
            if prev.contains_key(&edge.target) || edge.target == start {
                continue;
            }
            prev.insert(edge.target, edge.clone());
            queue.push_back(edge.target);
        }
    }
    rebuild(goal, start, &prev)
}

fn rebuild(goal: UorAddress, start: UorAddress, prev: &HashMap<UorAddress, Edge>) -> Vec<Edge> {
    let mut out = Vec::new();
    let mut cursor = goal;
    while cursor != start {
        let Some(edge) = prev.get(&cursor) else {
            return Vec::new();
        };
        out.push(edge.clone());
        cursor = edge.source;
    }
    out.reverse();
    out
}
