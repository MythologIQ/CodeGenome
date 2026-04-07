use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    pub fn workspace_trace(&self, workspace_dir: &str, from_repo: &str, to_repo: &str) -> String {
        let Some(overlay) = self.load_federated_overlay(Some(workspace_dir)) else {
            return r#"{"error":"no federated workspace found"}"#.into();
        };
        let addresses = repo_addresses(&overlay.nodes, from_repo, to_repo);
        let Some((start, goal)) = addresses else {
            return r#"{"error":"unknown workspace repositories"}"#.into();
        };
        let trace = codegenome_core::federation::query::trace_between(
            &codegenome_core::federation::workspace::WorkspaceGraph {
                workspace_id: "workspace".into(),
                repositories: vec![
                    codegenome_core::federation::workspace::RepositoryMember {
                        name: from_repo.into(),
                        node: start,
                    },
                    codegenome_core::federation::workspace::RepositoryMember {
                        name: to_repo.into(),
                        node: goal,
                    },
                ],
                aggregate_nodes: overlay.nodes.clone(),
                federated_edges: overlay.edges.clone(),
            },
            from_repo,
            to_repo,
        );
        serde_json::to_string_pretty(&trace).unwrap_or_default()
    }
}

fn repo_addresses(
    nodes: &[codegenome_core::graph::node::Node],
    from_repo: &str,
    to_repo: &str,
) -> Option<(
    codegenome_core::identity::UorAddress,
    codegenome_core::identity::UorAddress,
)> {
    let start = nodes
        .iter()
        .find(|n| n.address == codegenome_core::identity::address_of(from_repo.as_bytes()))?;
    let goal = nodes
        .iter()
        .find(|n| n.address == codegenome_core::identity::address_of(to_repo.as_bytes()))?;
    Some((start.address, goal.address))
}
