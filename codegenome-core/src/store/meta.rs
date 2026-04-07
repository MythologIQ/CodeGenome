use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexMeta {
    pub timestamp: u64,
    pub file_count: usize,
    pub node_count: usize,
    pub edge_count: usize,
    pub source_hashes: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WorkspaceMeta {
    pub workspace_id: String,
    pub repositories: Vec<String>,
}

#[derive(Debug)]
pub struct FreshnessReport {
    pub is_fresh: bool,
    pub last_indexed: u64,
    pub files_changed: usize,
    pub files_added: usize,
    pub files_removed: usize,
}

pub fn save(store_dir: &Path, meta: &IndexMeta) -> Result<(), String> {
    let path = store_dir.join("meta.json");
    let json = serde_json::to_string_pretty(meta).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

pub fn load(store_dir: &Path) -> Result<Option<IndexMeta>, String> {
    let path = store_dir.join("meta.json");
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let meta = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(Some(meta))
}

pub fn save_workspace(store_dir: &Path, meta: &WorkspaceMeta) -> Result<(), String> {
    let path = store_dir.join("workspace.json");
    let json = serde_json::to_string_pretty(meta).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

pub fn load_workspace(store_dir: &Path) -> Result<Option<WorkspaceMeta>, String> {
    let path = store_dir.join("workspace.json");
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let meta = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(Some(meta))
}

pub fn check_freshness(store_dir: &Path, source_dir: &Path) -> FreshnessReport {
    let meta = match load(store_dir) {
        Ok(Some(m)) => m,
        _ => {
            return FreshnessReport {
                is_fresh: false,
                last_indexed: 0,
                files_changed: 0,
                files_added: 0,
                files_removed: 0,
            }
        }
    };

    let current = hash_source_files(source_dir);
    let mut changed = 0;
    let mut added = 0;

    for (path, hash) in &current {
        match meta.source_hashes.get(path) {
            Some(old_hash) if old_hash != hash => changed += 1,
            None => added += 1,
            _ => {}
        }
    }
    let removed = meta
        .source_hashes
        .keys()
        .filter(|k| !current.contains_key(k.as_str()))
        .count();

    FreshnessReport {
        is_fresh: changed == 0 && added == 0 && removed == 0,
        last_indexed: meta.timestamp,
        files_changed: changed,
        files_added: added,
        files_removed: removed,
    }
}

pub fn hash_source_files(dir: &Path) -> HashMap<String, String> {
    let mut hashes = HashMap::new();
    collect_hashes(dir, &mut hashes);
    hashes
}

fn collect_hashes(dir: &Path, hashes: &mut HashMap<String, String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_hashes(&path, hashes);
        } else if path.extension().is_some_and(|e| e == "rs") {
            if let Ok(content) = std::fs::read(&path) {
                let hash = blake3::hash(&content).to_hex()[..16].to_string();
                hashes.insert(path.display().to_string(), hash);
            }
        }
    }
}
