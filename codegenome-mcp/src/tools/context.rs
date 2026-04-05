use codegenome_core::graph::overlay::Overlay;
use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    /// Retrieve context around a file:line location.
    /// Returns the node + its immediate edge neighbors as JSON.
    pub fn context(&self, file: &str, line: u32, depth: u32) -> String {
        let Some(overlay) = self.load_overlay() else {
            return r#"{"error":"no index found"}"#.into();
        };
        let target = overlay.nodes.iter().find(|n| {
            n.span.as_ref().is_some_and(|s| {
                s.start_line <= line && s.end_line >= line
            })
        });
        let Some(node) = target else {
            return format!(r#"{{"error":"no symbol at {file}:{line}"}}"#);
        };

        let neighbors = overlay.edges_touching(&[node.address]);
        let result = serde_json::json!({
            "node": format!("line {}:{}", node.span.as_ref().map_or(0, |s| s.start_line), node.span.as_ref().map_or(0, |s| s.end_line)),
            "kind": format!("{:?}", node.kind),
            "confidence": node.confidence,
            "neighbors": neighbors.len(),
            "depth": depth,
        });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }
}
