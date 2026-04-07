use std::fs;

use crate::federation::config::{RepositoryConfig, WorkspaceConfig};
use crate::federation::{index, query};
use crate::graph::edge::Relation;

#[test]
fn workspace_trace_crosses_repo_boundaries_only_through_federated_edges() {
    let dir = std::env::temp_dir().join(format!("cg_fed_query_{:?}", std::thread::current().id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let repo_a = dir.join("repo-a");
    let repo_b = dir.join("repo-b");
    fs::create_dir_all(repo_a.join("src")).unwrap();
    fs::create_dir_all(repo_b.join("src")).unwrap();
    fs::write(repo_a.join("Cargo.toml"), "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n\n[dependencies]\ncrate_b = \"0.1.0\"\n").unwrap();
    fs::write(
        repo_b.join("Cargo.toml"),
        "[package]\nname = \"crate_b\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(repo_a.join("src").join("lib.rs"), "pub fn a() {}\n").unwrap();
    fs::write(repo_b.join("src").join("lib.rs"), "pub fn b() {}\n").unwrap();

    let cfg = WorkspaceConfig {
        workspace_id: "ws".into(),
        repositories: vec![
            RepositoryConfig::new("crate_a", &repo_a, &dir.join("a-store")),
            RepositoryConfig::new("crate_b", &repo_b, &dir.join("b-store")),
        ],
    };
    let graph = index::build_workspace(&cfg, &dir.join("workspace-store")).unwrap();
    let trace = query::trace_between(&graph, "crate_a", "crate_b");
    assert!(!trace.is_empty());
    assert!(trace.iter().all(|e| e.relation == Relation::References));
    let _ = fs::remove_dir_all(&dir);
}
