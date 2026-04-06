use std::path::Path;

use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    /// Smart re-index: check freshness first, rebuild if stale.
    pub fn reindex(&self, source_dir: &str) -> String {
        let src = if source_dir.is_empty() { "." } else { source_dir };
        match codegenome_core::index::run_pipeline(
            Path::new(src),
            &self.store_dir,
        ) {
            Ok(r) if r.is_fresh => serde_json::json!({
                "status": "fresh",
                "files": r.file_count,
                "nodes": r.node_count,
                "edges": r.edge_count,
            }).to_string(),
            Ok(r) => serde_json::json!({
                "status": "reindexed",
                "files": r.file_count,
                "nodes": r.node_count,
                "edges": r.edge_count,
                "elapsed_ms": r.elapsed_ms,
            }).to_string(),
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }
}
