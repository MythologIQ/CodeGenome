use std::fs;

use crate::federation::config::{RepositoryConfig, WorkspaceConfig};
use crate::federation::index;
use crate::graph::edge::{Edge, Relation};
use crate::graph::node::{Node, NodeKind, Provenance, Timestamp};
use crate::graph::overlay::OverlayKind;
use crate::identity::address_of;
use crate::store::backend::StoreBackend;
use crate::store::ondisk::OnDiskStore;

#[test]
fn workspace_graph_preserves_membership_and_creates_dependency_edge() {
    let dir = std::env::temp_dir().join(format!("cg_fed_{:?}", std::thread::current().id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let repo_a = make_repo(&dir, "repo-a", "crate_a", &["crate_b"]);
    let repo_b = make_repo(&dir, "repo-b", "crate_b", &[]);
    seed_fused(&repo_a.store_dir, &[node("shared")], &[]);
    seed_fused(&repo_b.store_dir, &[node("shared")], &[]);

    let config = WorkspaceConfig {
        workspace_id: "ws".into(),
        repositories: vec![
            RepositoryConfig::new("crate_a", &repo_a.source_dir, &repo_a.store_dir),
            RepositoryConfig::new("crate_b", &repo_b.source_dir, &repo_b.store_dir),
        ],
    };

    let graph = index::build_workspace(&config, &dir.join("workspace-store")).unwrap();
    assert_eq!(graph.repositories.len(), 2);
    assert!(graph
        .federated_edges
        .iter()
        .any(|e| e.relation == Relation::References));
    assert!(graph.federated_edges.iter().any(|e| e.source != e.target));
    let _ = fs::remove_dir_all(&dir);
}

fn seed_fused(store_dir: &std::path::Path, nodes: &[Node], edges: &[Edge]) {
    let store = OnDiskStore::new(store_dir);
    store
        .write_overlay(&OverlayKind::Custom("fused".into()), nodes, edges)
        .unwrap();
}

fn node(name: &str) -> Node {
    let address = address_of(name.as_bytes());
    Node {
        address,
        kind: NodeKind::Symbol,
        provenance: Provenance::tool("test", Timestamp(1)),
        confidence: 1.0,
        created_at: Timestamp(1),
        content_hash: address,
        span: None,
    }
}

struct RepoPaths {
    source_dir: std::path::PathBuf,
    store_dir: std::path::PathBuf,
}

fn make_repo(base: &std::path::Path, dir_name: &str, crate_name: &str, deps: &[&str]) -> RepoPaths {
    let source_dir = base.join(dir_name);
    let store_dir = base.join(format!("{dir_name}-store"));
    fs::create_dir_all(source_dir.join("src")).unwrap();
    let deps_block = deps
        .iter()
        .map(|d| format!("{d} = \"0.1.0\""))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        source_dir.join("Cargo.toml"),
        format!("[package]\nname = \"{crate_name}\"\nversion = \"0.1.0\"\n\n[dependencies]\n{deps_block}\n"),
    ).unwrap();
    fs::write(source_dir.join("src").join("lib.rs"), "pub fn x() {}\n").unwrap();
    RepoPaths {
        source_dir,
        store_dir,
    }
}
