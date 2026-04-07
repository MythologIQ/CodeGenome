use serde::{Deserialize, Serialize};

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::identity::UorAddress;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryMember {
    pub name: String,
    pub node: UorAddress,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceGraph {
    pub workspace_id: String,
    pub repositories: Vec<RepositoryMember>,
    pub aggregate_nodes: Vec<Node>,
    pub federated_edges: Vec<Edge>,
    /// Symbol-level cross-repo edges (confidence 0.7 for name-resolved,
    /// 1.0 for identity matches). Separate from repo-level edges.
    pub symbol_edges: Vec<Edge>,
}
