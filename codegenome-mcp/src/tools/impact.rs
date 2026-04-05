use codegenome_core::graph::overlay::Overlay;
use codegenome_core::signal::impact::propagate_impact;
use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    /// Blast radius: propagate impact from a file:line location.
    /// Returns impacted nodes with confidence scores as JSON.
    pub fn impact(&self, file: &str, line: u32) -> String {
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

        let overlays: Vec<&dyn Overlay> = vec![&overlay];
        let impact = propagate_impact(&[node.address], &overlays);
        let mut results: Vec<_> = impact.iter()
            .filter(|(_, &s)| s > 0.01)
            .collect();
        results.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

        let items: Vec<_> = results.iter().take(50).map(|(addr, score)| {
            let loc = overlay.nodes.iter()
                .find(|n| n.address == **addr)
                .and_then(|n| n.span.as_ref())
                .map(|s| format!("line {}:{}", s.start_line, s.end_line))
                .unwrap_or_else(|| format!("{addr:?}"));
            serde_json::json!({"node": loc, "confidence": score})
        }).collect();

        serde_json::to_string_pretty(&items).unwrap_or_default()
    }
}
