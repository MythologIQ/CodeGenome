use codegenome_core::graph::edge::Relation;
use codegenome_core::graph::node::NodeKind;
use codegenome_core::graph::overlay::Overlay;
use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    /// Process trace: find an entrypoint and list its call chain.
    /// Returns PartOfProcess edges from the matching process node.
    pub fn trace(&self, entrypoint: &str) -> String {
        let Some(overlay) = self.load_overlay() else {
            return r#"{"error":"no index found"}"#.into();
        };

        // Find the process node matching the entrypoint name
        let proc_node = overlay.nodes.iter().find(|n| {
            n.kind == NodeKind::Process
        });
        let Some(proc) = proc_node else {
            return format!(r#"{{"error":"no process for '{entrypoint}'"}}"#);
        };

        let chain: Vec<_> = overlay.edges().iter()
            .filter(|e| e.source == proc.address && e.relation == Relation::PartOfProcess)
            .map(|e| {
                let loc = overlay.nodes.iter()
                    .find(|n| n.address == e.target)
                    .and_then(|n| n.span.as_ref())
                    .map(|s| format!("line {}:{}", s.start_line, s.end_line))
                    .unwrap_or_else(|| format!("{:?}", e.target));
                serde_json::json!({
                    "node": loc,
                    "confidence": e.confidence,
                })
            })
            .collect();

        serde_json::to_string_pretty(&chain).unwrap_or_default()
    }
}
