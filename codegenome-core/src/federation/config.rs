use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub name: String,
    pub source_dir: PathBuf,
    pub store_dir: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub workspace_id: String,
    pub repositories: Vec<RepositoryConfig>,
}

impl RepositoryConfig {
    pub fn new(name: &str, source_dir: &Path, store_dir: &Path) -> Self {
        Self {
            name: name.into(),
            source_dir: source_dir.to_path_buf(),
            store_dir: store_dir.to_path_buf(),
        }
    }
}

pub fn load(path: &Path) -> Result<WorkspaceConfig, String> {
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    toml::from_str(&data).map_err(|e| e.to_string())
}
