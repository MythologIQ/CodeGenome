use std::path::Path;

use crate::federation::metrics::{self, WorkspaceReport};
use crate::federation::workspace::WorkspaceGraph;
use codegenome_identity::graph::edge::Edge;
use codegenome_identity::graph::node::Node;
use codegenome_identity::graph::overlay::OverlayKind;
use codegenome_identity::store::backend::StoreBackend;
use codegenome_identity::store::meta;
use codegenome_identity::store::ondisk::OnDiskStore;

pub fn load_report(store_dir: &Path) -> Result<WorkspaceReport, String> {
    let graph = load_graph(store_dir)?;
    Ok(metrics::build_report(&graph))
}

fn load_graph(store_dir: &Path) -> Result<WorkspaceGraph, String> {
    let store = OnDiskStore::new(store_dir);
    let meta = meta::load_workspace(store_dir)?
        .ok_or_else(|| format!("No workspace metadata at {}", store_dir.display()))?;
    let (nodes, edges) = store
        .read_overlay(&OverlayKind::Federated)?
        .ok_or_else(|| format!("No federated overlay at {}", store_dir.display()))?;
    Ok(WorkspaceGraph {
        workspace_id: meta.workspace_id,
        repositories: meta
            .repositories
            .into_iter()
            .zip(nodes.iter().map(|n| n.address))
            .map(|(name, node)| crate::federation::workspace::RepositoryMember { name, node })
            .collect(),
        aggregate_nodes: nodes,
        federated_edges: edges,
        symbol_edges: Vec::new(),
    })
}

pub fn load_overlay(store_dir: &Path) -> Result<(Vec<Node>, Vec<Edge>), String> {
    let store = OnDiskStore::new(store_dir);
    store
        .read_overlay(&OverlayKind::Federated)?
        .ok_or_else(|| format!("No federated overlay at {}", store_dir.display()))
}
