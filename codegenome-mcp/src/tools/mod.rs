pub mod context;
pub mod detect;
pub mod impact;
pub mod trace;

use std::path::PathBuf;

use codegenome_core::graph::edge::Edge;
use codegenome_core::graph::node::Node;
use codegenome_core::graph::overlay::{Overlay, OverlayKind};
use codegenome_core::measurement::GroundTruthLevel;
use codegenome_core::store::backend::StoreBackend;
use codegenome_core::store::ondisk::OnDiskStore;

/// Shared server state holding the store path.
#[derive(Clone)]
pub struct CodegenomeTools {
    pub store_dir: PathBuf,
}

impl CodegenomeTools {
    pub fn new(store_dir: impl Into<PathBuf>) -> Self {
        Self { store_dir: store_dir.into() }
    }

    pub fn load_overlay(&self) -> Option<StoredOverlay> {
        let store = OnDiskStore::new(&self.store_dir);
        let (nodes, edges) = store
            .read_overlay(&OverlayKind::Custom("fused".into()))
            .ok()??;
        Some(StoredOverlay { nodes, edges })
    }
}

/// Wrapper to make stored data implement Overlay trait.
pub struct StoredOverlay {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Overlay for StoredOverlay {
    fn kind(&self) -> OverlayKind { OverlayKind::Custom("stored".into()) }
    fn nodes(&self) -> &[Node] { &self.nodes }
    fn edges(&self) -> &[Edge] { &self.edges }
    fn ground_truth(&self) -> GroundTruthLevel { GroundTruthLevel::Constructible }
}
