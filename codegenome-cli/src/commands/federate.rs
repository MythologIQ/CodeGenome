use std::path::Path;

use codegenome_core::federation::config;
use codegenome_core::federation::index;

pub fn run(workspace_config: &str, store_dir: &str) {
    let cfg = match config::load(Path::new(workspace_config)) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Workspace config failed: {e}");
            std::process::exit(1);
        }
    };
    match index::build_workspace(&cfg, Path::new(store_dir)) {
        Ok(graph) => println!(
            "Federated {} repositories -> {} nodes, {} edges",
            graph.repositories.len(),
            graph.aggregate_nodes.len(),
            graph.federated_edges.len()
        ),
        Err(e) => {
            eprintln!("Federate failed: {e}");
            std::process::exit(1);
        }
    }
}
