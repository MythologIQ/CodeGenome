pub mod context;
pub mod detect;
pub mod experiment_tool;
pub mod impact;
pub mod reindex;
pub mod status_tool;
pub mod trace;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use codegenome_core::graph::edge::Edge;
use codegenome_core::graph::node::Node;
use codegenome_core::graph::overlay::{Overlay, OverlayKind};
use codegenome_core::measurement::GroundTruthLevel;
use codegenome_core::store::backend::StoreBackend;
use codegenome_core::store::ondisk::OnDiskStore;

/// Shared server state.
pub struct CodegenomeTools {
    pub store_dir: PathBuf,
    pub run_manager: Arc<Mutex<RunManager>>,
}

impl Clone for CodegenomeTools {
    fn clone(&self) -> Self {
        Self {
            store_dir: self.store_dir.clone(),
            run_manager: Arc::clone(&self.run_manager),
        }
    }
}

pub struct RunManager {
    pub active_run: Option<RunHandle>,
}

pub struct RunHandle {
    pub run_id: String,
    pub max_iterations: u64,
    pub log_path: PathBuf,
    pub completed: Arc<std::sync::atomic::AtomicBool>,
}

impl CodegenomeTools {
    pub fn new(store_dir: impl Into<PathBuf>) -> Self {
        Self {
            store_dir: store_dir.into(),
            run_manager: Arc::new(Mutex::new(RunManager { active_run: None })),
        }
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
