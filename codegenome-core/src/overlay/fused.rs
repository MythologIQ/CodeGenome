use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::overlay::{Overlay, OverlayKind};
use crate::measurement::GroundTruthLevel;

pub struct FusedOverlay {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl FusedOverlay {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self { nodes, edges }
    }
}

impl Overlay for FusedOverlay {
    fn kind(&self) -> OverlayKind {
        OverlayKind::Custom("fused".into())
    }
    fn nodes(&self) -> &[Node] {
        &self.nodes
    }
    fn edges(&self) -> &[Edge] {
        &self.edges
    }
    fn ground_truth(&self) -> GroundTruthLevel {
        GroundTruthLevel::Constructible
    }
}

/// Backward-compatible wrapper. Delegates to index::merger.
pub fn fuse(overlays: &[&dyn Overlay]) -> FusedOverlay {
    crate::index::merger::fuse(overlays)
}
