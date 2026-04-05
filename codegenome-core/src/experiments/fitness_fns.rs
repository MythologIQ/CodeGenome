use std::path::Path;

use crate::experiments::config::ExperimentParams;
use crate::experiments::fitness::{build_overlays, depth_propagate, param_or};
use crate::graph::node::NodeKind;
use crate::graph::overlay::Overlay;

/// Average max propagation depth from sampled symbols, normalized.
pub fn propagation_depth(
    source_dir: &Path,
    params: &ExperimentParams,
) -> f64 {
    let Some((syntax, semantic, flow, _)) = build_overlays(source_dir) else {
        return 0.0;
    };
    let overlays: Vec<&dyn Overlay> = vec![&syntax, &semantic, &flow];
    let symbols: Vec<_> = syntax
        .nodes()
        .iter()
        .filter(|n| n.kind == NodeKind::Symbol && n.span.is_some())
        .collect();
    if symbols.is_empty() {
        return 0.0;
    }

    let threshold = param_or(params, "confidence_threshold", 0.5);
    let atten = param_or(params, "attenuation_factor", 0.8);
    let max_depth_param = param_or(params, "max_depth", 5.0);
    let sample_size = symbols.len().min(10);
    let mut total_depth = 0.0;

    for symbol in symbols.iter().take(sample_size) {
        let reached = depth_propagate(symbol.address, &overlays, atten, threshold);
        total_depth += reached.len() as f64;
    }

    let avg = total_depth / sample_size as f64;
    (avg / max_depth_param).clamp(0.0, 1.0)
}

/// Parse-propagate cycle time as fitness. Faster = higher.
pub fn cycle_time(
    source_dir: &Path,
    params: &ExperimentParams,
) -> f64 {
    let start = std::time::Instant::now();
    let Some((syntax, semantic, flow, _)) = build_overlays(source_dir) else {
        return 0.0;
    };
    let overlays: Vec<&dyn Overlay> = vec![&syntax, &semantic, &flow];
    let file_node = syntax
        .nodes()
        .iter()
        .find(|n| n.kind == NodeKind::File);
    if let Some(node) = file_node {
        let threshold = param_or(params, "confidence_threshold", 0.5);
        let atten = param_or(params, "attenuation_factor", 0.8);
        let _ = depth_propagate(node.address, &overlays, atten, threshold);
    }
    let ms = start.elapsed().as_millis() as f64;
    (1.0 - ms / 5000.0).clamp(0.0, 1.0)
}

/// Edge-to-node ratio across all three overlays.
pub fn graph_density(
    source_dir: &Path,
    _params: &ExperimentParams,
) -> f64 {
    let Some((syntax, semantic, flow, _)) = build_overlays(source_dir) else {
        return 0.0;
    };
    let total_nodes = syntax.nodes().len()
        + semantic.nodes().len()
        + flow.nodes().len();
    let total_edges = syntax.edges().len()
        + semantic.edges().len()
        + flow.edges().len();
    if total_nodes == 0 {
        return 0.0;
    }
    (total_edges as f64 / total_nodes as f64).min(1.0)
}
