use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::overlay::OverlayKind;
use crate::store::backend::StoreBackend;
use crate::store::ondisk::OnDiskStore;

const BELIEF_OVERLAY: &str = "beliefs";

/// Persist beliefs to the graph store as a dedicated overlay.
/// Beliefs are namespaced separately from the extraction pipeline.
pub fn persist_beliefs(
    store: &OnDiskStore,
    beliefs: &[(Node, Vec<Edge>)],
) -> Result<(), String> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (node, node_edges) in beliefs {
        nodes.push(node.clone());
        edges.extend(node_edges.iter().cloned());
    }
    store.write_overlay(
        &OverlayKind::Custom(BELIEF_OVERLAY.into()),
        &nodes,
        &edges,
    )
}

/// Load all persisted beliefs from the store.
pub fn load_beliefs(
    store: &OnDiskStore,
) -> Result<(Vec<Node>, Vec<Edge>), String> {
    store
        .read_overlay(&OverlayKind::Custom(BELIEF_OVERLAY.into()))?
        .ok_or_else(|| "No belief overlay found".into())
}
