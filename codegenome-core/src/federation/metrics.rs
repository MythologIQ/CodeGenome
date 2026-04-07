use std::collections::BTreeMap;

use serde::Serialize;

use crate::federation::query;
use crate::federation::workspace::WorkspaceGraph;
use crate::graph::edge::Edge;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DistributionStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WorkspaceReport {
    pub repository_count: usize,
    pub federated_edge_count: usize,
    pub dependency_coverage: f64,
    pub cross_repo_trace_depth: DistributionStats,
    pub evidence_source_counts: BTreeMap<String, usize>,
}

pub fn build_report(graph: &WorkspaceGraph) -> WorkspaceReport {
    WorkspaceReport {
        repository_count: graph.repositories.len(),
        federated_edge_count: graph.federated_edges.len(),
        dependency_coverage: coverage(graph),
        cross_repo_trace_depth: trace_depth(graph),
        evidence_source_counts: evidence_counts(&graph.federated_edges),
    }
}

fn coverage(graph: &WorkspaceGraph) -> f64 {
    let pairs = graph
        .repositories
        .len()
        .saturating_mul(graph.repositories.len().saturating_sub(1));
    if pairs == 0 {
        return 0.0;
    }
    graph.federated_edges.len() as f64 / pairs as f64
}

fn trace_depth(graph: &WorkspaceGraph) -> DistributionStats {
    let mut depths = Vec::new();
    for a in &graph.repositories {
        for b in &graph.repositories {
            if a.name == b.name {
                continue;
            }
            let trace = query::trace_between(graph, &a.name, &b.name);
            if !trace.is_empty() {
                depths.push(trace.len() as f64);
            }
        }
    }
    summarize(&depths)
}

fn evidence_counts(edges: &[Edge]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for edge in edges {
        *counts.entry(edge.provenance.actor.clone()).or_insert(0) += 1;
    }
    counts
}

fn summarize(values: &[f64]) -> DistributionStats {
    if values.is_empty() {
        return DistributionStats {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
        };
    }
    DistributionStats {
        min: values.iter().copied().fold(f64::INFINITY, f64::min),
        max: values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        mean: values.iter().sum::<f64>() / values.len() as f64,
    }
}
