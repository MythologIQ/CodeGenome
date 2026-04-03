use crate::diff::types::{ChangeSet, OwnedDiff};
use crate::graph::overlay::Overlay;
use crate::graph::node::Span;
use crate::identity::UorAddress;

/// Maps a diff to affected symbol addresses.
/// Intersects hunk line ranges with overlay Span data.
pub fn detect_changes(
    diff: &OwnedDiff,
    overlay: &dyn Overlay,
) -> ChangeSet {
    let mut changed_nodes = Vec::new();
    let mut affected_edges = Vec::new();

    for diff_file in &diff.files {
        let file_path = diff_file.path.to_string_lossy();
        let hunk_ranges: Vec<(u32, u32)> = diff_file
            .hunks
            .iter()
            .map(|h| (h.new_start, h.new_start + h.new_lines))
            .collect();

        for node in overlay.nodes() {
            if !node_matches_file(node, &file_path) {
                continue;
            }
            if let Some(span) = &node.span {
                if hunk_overlaps_span(&hunk_ranges, span) {
                    changed_nodes.push(node.address);
                }
            }
        }
    }

    let touched: std::collections::HashSet<UorAddress> =
        changed_nodes.iter().copied().collect();

    for edge in overlay.edges() {
        if touched.contains(&edge.source)
            || touched.contains(&edge.target)
        {
            affected_edges.push(edge.source);
        }
    }

    ChangeSet { changed_nodes, affected_edges }
}

fn node_matches_file(
    node: &crate::graph::Node,
    _file_path: &str,
) -> bool {
    // For now, all nodes are candidates. File-scoping
    // will be refined when nodes carry file provenance.
    node.span.is_some()
}

fn hunk_overlaps_span(
    hunk_ranges: &[(u32, u32)],
    span: &Span,
) -> bool {
    hunk_ranges.iter().any(|&(start, end)| {
        span.start_line < end && span.end_line >= start
    })
}
