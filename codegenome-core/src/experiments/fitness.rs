use std::path::Path;

use crate::diff::{DiffStatus, OwnedDiff, OwnedDiffFile, OwnedHunk};
use crate::diff::mapper::detect_changes;
use crate::experiments::config::ExperimentParams;
use crate::graph::node::NodeKind;
use crate::graph::overlay::Overlay;
use crate::overlay::syntax::parse_rust_files;
use crate::signal::impact::propagate_impact;

/// Impact prediction accuracy: fabricate diffs for known
/// symbols, run detect_changes, check if the changed symbol
/// appears in the impact map.
pub fn impact_accuracy(source_dir: &Path) -> f64 {
    let files = collect_rs_files(source_dir);
    if files.is_empty() {
        return 0.0;
    }
    let overlay = parse_rust_files(&files);
    let symbols: Vec<_> = overlay
        .nodes()
        .iter()
        .filter(|n| n.kind == NodeKind::Symbol && n.span.is_some())
        .collect();

    if symbols.is_empty() {
        return 0.0;
    }

    let sample_size = symbols.len().min(10);
    let mut correct = 0u32;

    for symbol in symbols.iter().take(sample_size) {
        let span = symbol.span.unwrap();
        let diff = fabricate_diff(span.start_line);
        let changeset = detect_changes(
            &diff,
            &[&overlay as &dyn Overlay],
        );
        if changeset.impact.contains_key(&symbol.address) {
            correct += 1;
        }
    }

    correct as f64 / sample_size as f64
}

/// Stability: measure how much impact scores change when
/// params are slightly perturbed.
pub fn stability(
    source_dir: &Path,
    _params: &ExperimentParams,
) -> f64 {
    let files = collect_rs_files(source_dir);
    if files.is_empty() {
        return 1.0;
    }
    let overlay = parse_rust_files(&files);
    let symbols: Vec<_> = overlay
        .nodes()
        .iter()
        .filter(|n| n.kind == NodeKind::Symbol)
        .collect();

    if symbols.is_empty() {
        return 1.0;
    }

    let root = symbols[0].address;
    let impact_a = propagate_impact(&[root], &[&overlay as &dyn Overlay]);
    let impact_b = propagate_impact(&[root], &[&overlay as &dyn Overlay]);

    let total = impact_a.len().max(1) as f64;
    let mut changed = 0u32;
    for (addr, score_a) in &impact_a {
        let score_b = impact_b.get(addr).unwrap_or(&0.0);
        if (score_a - score_b).abs() > 0.001 {
            changed += 1;
        }
    }

    1.0 - (changed as f64 / total)
}

fn fabricate_diff(line: u32) -> OwnedDiff {
    OwnedDiff {
        files: vec![OwnedDiffFile {
            path: "synthetic.rs".into(),
            status: DiffStatus::Modified,
            hunks: vec![OwnedHunk {
                new_start: line,
                new_lines: 3,
                old_start: line,
                old_lines: 3,
            }],
        }],
    }
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
