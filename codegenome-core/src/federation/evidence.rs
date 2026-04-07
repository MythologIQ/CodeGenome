use std::collections::HashMap;
use std::path::Path;

use crate::federation::config::WorkspaceConfig;

pub fn dependency_edges(cfg: &WorkspaceConfig) -> Vec<(String, String, String)> {
    let by_pkg: HashMap<_, _> = cfg
        .repositories
        .iter()
        .filter_map(|repo| {
            package_and_deps(&repo.source_dir)
                .ok()
                .map(|(pkg, deps)| (pkg, deps))
        })
        .collect();
    cfg.repositories
        .iter()
        .flat_map(|repo| {
            let source = repo.name.clone();
            let deps = by_pkg.get(&source).cloned().unwrap_or_default();
            deps.into_iter()
                .filter(|dep| by_pkg.contains_key(dep))
                .map(move |dep| (source.clone(), dep, "cargo_dependency".into()))
        })
        .collect()
}

pub fn identity_edges(
    repositories: &[(String, Vec<crate::graph::node::Node>)],
) -> Vec<(String, String, String)> {
    let mut seen: HashMap<crate::identity::UorAddress, Vec<String>> = HashMap::new();
    for (repo, nodes) in repositories {
        for node in nodes {
            seen.entry(node.address).or_default().push(repo.clone());
        }
    }
    seen.into_values()
        .flat_map(|repos| {
            let mut out = Vec::new();
            for i in 0..repos.len() {
                for j in i + 1..repos.len() {
                    out.push((repos[i].clone(), repos[j].clone(), "shared_uor".into()));
                }
            }
            out
        })
        .collect()
}

fn package_and_deps(root: &Path) -> Result<(String, Vec<String>), String> {
    let path = root.join("Cargo.toml");
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let value: toml::Value = toml::from_str(&data).map_err(|e| e.to_string())?;
    let package = value
        .get("package")
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing package.name".to_string())?;
    let deps = value
        .get("dependencies")
        .and_then(|v| v.as_table())
        .map(|t| t.keys().cloned().collect())
        .unwrap_or_default();
    Ok((package.into(), deps))
}
