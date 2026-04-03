use serde::{Deserialize, Serialize};

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::identity::UorAddress;
use crate::measurement::GroundTruthLevel;

/// An overlay contributes nodes and edges without knowing
/// what other overlays exist. This is the un-braiding:
/// syntax doesn't know about semantics. The merger doesn't
/// know how many overlays there are.
pub trait Overlay: Send + Sync {
    fn kind(&self) -> OverlayKind;
    fn nodes(&self) -> &[Node];
    fn edges(&self) -> &[Edge];

    fn edges_touching(&self, addresses: &[UorAddress]) -> Vec<&Edge> {
        self.edges()
            .iter()
            .filter(|e| {
                addresses.contains(&e.source)
                    || addresses.contains(&e.target)
            })
            .collect()
    }

    fn ground_truth(&self) -> GroundTruthLevel;
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum OverlayKind {
    Syntax,
    Semantic,
    Flow,
    Runtime,
    Custom(String),
}
