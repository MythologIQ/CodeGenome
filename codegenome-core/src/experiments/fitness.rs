use std::path::Path;

use crate::experiments::config::ExperimentParams;
use crate::graph::edge::Relation;
use crate::graph::node::NodeKind;
use crate::graph::overlay::Overlay;
use crate::identity::UorAddress;
use crate::overlay::syntax::parse_rust_files;

/// Impact prediction accuracy: for each sampled symbol,
/// propagate from it with attenuation and check what
/// fraction of its siblings are reached above threshold.
pub fn impact_accuracy(
    source_dir: &Path,
    params: &ExperimentParams,
) -> f64 {
    let files = collect_rs_files(source_dir);
    if files.is_empty() {
        return 0.0;
    }
    let overlay = parse_rust_files(&files);
    let symbols = symbols_with_spans(&overlay);
    if symbols.is_empty() {
        return 0.0;
    }

    let threshold = param_or(params, "confidence_threshold", 0.5);
    let atten = param_or(params, "attenuation_factor", 0.8);
    let overlays: &[&dyn Overlay] = &[&overlay];
    let sample_size = symbols.len().min(10);
    let mut total_score = 0.0;
    let mut total_tests = 0u32;

    for symbol in symbols.iter().take(sample_size) {
        let siblings = find_siblings(&overlay, symbol.address);
        if siblings.is_empty() {
            continue;
        }
        // Find parent file to propagate from
        let parent = find_parent(&overlay, symbol.address);
        let root = parent.unwrap_or(symbol.address);
        let impact = depth_propagate(
            root, overlays, atten, threshold,
        );
        let hits = siblings
            .iter()
            .filter(|addr| impact.contains_key(addr))
            .count();
        total_score += hits as f64 / siblings.len() as f64;
        total_tests += 1;
    }

    if total_tests == 0 {
        return 0.0;
    }
    total_score / total_tests as f64
}

/// Stability: run depth-attenuated propagation with two
/// different attenuation factors and measure how many nodes'
/// inclusion changes. Uses BFS depth to apply attenuation.
pub fn stability(
    source_dir: &Path,
    params: &ExperimentParams,
) -> f64 {
    let files = collect_rs_files(source_dir);
    if files.is_empty() {
        return 1.0;
    }
    let overlay = parse_rust_files(&files);
    // Start from a File node so Contains edges propagate
    let file_nodes: Vec<_> = overlay
        .nodes()
        .iter()
        .filter(|n| n.kind == NodeKind::File)
        .collect();
    if file_nodes.is_empty() {
        return 1.0;
    }

    let atten = param_or(params, "attenuation_factor", 0.8);
    let threshold = param_or(params, "confidence_threshold", 0.5);
    let overlays: &[&dyn Overlay] = &[&overlay];
    let root = file_nodes[0].address;

    let set_a = depth_propagate(root, overlays, atten, threshold);
    let perturbed = atten * 0.9;
    let set_b = depth_propagate(root, overlays, perturbed, threshold);

    let all_keys: std::collections::HashSet<_> = set_a
        .keys()
        .chain(set_b.keys())
        .collect();
    let total = all_keys.len().max(1) as f64;
    let sum_diff: f64 = all_keys
        .iter()
        .map(|addr| {
            let a = set_a.get(addr).unwrap_or(&0.0);
            let b = set_b.get(addr).unwrap_or(&0.0);
            (a - b).abs()
        })
        .sum();
    let mean_diff = sum_diff / total;

    1.0 - mean_diff
}

/// BFS propagation that applies attenuation per hop.
fn depth_propagate(
    root: UorAddress,
    overlays: &[&dyn Overlay],
    attenuation: f64,
    threshold: f64,
) -> std::collections::HashMap<UorAddress, f64> {
    use std::collections::{HashMap, VecDeque};
    let mut scores: HashMap<UorAddress, f64> = HashMap::new();
    scores.insert(root, 1.0);
    let mut queue = VecDeque::new();
    queue.push_back((root, 1.0));

    while let Some((node, score)) = queue.pop_front() {
        for overlay in overlays {
            for edge in overlay.edges() {
                if edge.source != node {
                    continue;
                }
                let child_score =
                    score * edge.confidence * attenuation;
                if child_score < threshold {
                    continue;
                }
                let entry = scores.entry(edge.target).or_insert(0.0);
                if child_score > *entry {
                    *entry = child_score;
                    queue.push_back((edge.target, child_score));
                }
            }
        }
    }
    scores
}

fn find_parent(
    overlay: &dyn Overlay,
    addr: UorAddress,
) -> Option<UorAddress> {
    overlay
        .edges()
        .iter()
        .find(|e| {
            e.target == addr && e.relation == Relation::Contains
        })
        .map(|e| e.source)
}

/// Find sibling symbols: nodes sharing a Contains parent
/// with `addr` in the same file.
fn find_siblings(
    overlay: &dyn Overlay,
    addr: UorAddress,
) -> Vec<UorAddress> {
    let parents: Vec<UorAddress> = overlay
        .edges()
        .iter()
        .filter(|e| {
            e.target == addr && e.relation == Relation::Contains
        })
        .map(|e| e.source)
        .collect();

    let mut siblings = Vec::new();
    for edge in overlay.edges() {
        if edge.relation == Relation::Contains
            && parents.contains(&edge.source)
            && edge.target != addr
        {
            siblings.push(edge.target);
        }
    }
    siblings
}

fn symbols_with_spans(
    overlay: &dyn Overlay,
) -> Vec<crate::graph::node::Node> {
    overlay
        .nodes()
        .iter()
        .filter(|n| {
            n.kind == NodeKind::Symbol && n.span.is_some()
        })
        .cloned()
        .collect()
}

fn collect_rs_files(
    dir: &Path,
) -> Vec<(std::path::PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_rs_files(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            if let Ok(content) = std::fs::read(&path) {
                files.push((path, content));
            }
        }
    }
    files
}

fn param_or(
    params: &ExperimentParams,
    key: &str,
    default: f64,
) -> f64 {
    params.values.get(key).copied().unwrap_or(default)
}
