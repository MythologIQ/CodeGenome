use codegenome_core::diff::detect_changes;
use codegenome_core::diff::git_bridge;
use codegenome_core::graph::overlay::Overlay;

use crate::tools::inputs::DetectInput;
use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    /// Map git diff to affected symbols + impact.
    pub fn detect(&self, input: &DetectInput) -> String {
        let Some(overlay) = self.load_overlay() else {
            return r#"{"error":"no index found"}"#.into();
        };

        let diff = match git_bridge::git_diff(
            &self.source_dir,
            Some(&input.from_ref),
            input.to_ref.as_deref(),
        ) {
            Ok(d) => d,
            Err(e) => {
                return serde_json::json!({"error": e}).to_string()
            }
        };

        let overlays: Vec<&dyn Overlay> = vec![&overlay];
        let changeset = detect_changes(&diff, &overlays);

        let mut resp = serde_json::json!({
            "from_ref": input.from_ref,
            "to_ref": input.to_ref.as_deref().unwrap_or("workdir"),
            "changed_nodes": changeset.changed_nodes.len(),
            "affected_edges": changeset.affected_edges.len(),
            "impact_nodes": changeset.impact.len(),
            "staleness_nodes": changeset.staleness.len(),
        });
        resp["meta"] = self.response_meta();
        serde_json::to_string_pretty(&resp).unwrap_or_default()
    }
}
