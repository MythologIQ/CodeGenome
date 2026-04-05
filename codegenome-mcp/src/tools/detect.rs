use codegenome_core::diff::{detect_changes, DiffStatus, OwnedDiff, OwnedDiffFile, OwnedHunk};
use codegenome_core::graph::overlay::Overlay;
use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    /// Map a unified diff to affected symbols + impact.
    /// Accepts raw diff text, returns changed nodes as JSON.
    pub fn detect(&self, diff_text: &str) -> String {
        let Some(overlay) = self.load_overlay() else {
            return r#"{"error":"no index found"}"#.into();
        };
        let diff = parse_simple_diff(diff_text);
        let overlays: Vec<&dyn Overlay> = vec![&overlay];
        let changeset = detect_changes(&diff, &overlays);

        let result = serde_json::json!({
            "changed_nodes": changeset.changed_nodes.len(),
            "affected_edges": changeset.affected_edges.len(),
            "impact_nodes": changeset.impact.len(),
            "staleness_nodes": changeset.staleness.len(),
        });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }
}

/// Minimal diff parser: extracts hunk line numbers.
fn parse_simple_diff(text: &str) -> OwnedDiff {
    let mut files = Vec::new();
    let mut hunks = Vec::new();

    for line in text.lines() {
        if line.starts_with("@@") {
            if let Some((start, lines)) = parse_hunk_header(line) {
                hunks.push(OwnedHunk { new_start: start, new_lines: lines, old_start: 0, old_lines: 0 });
            }
        }
    }

    if !hunks.is_empty() {
        files.push(OwnedDiffFile {
            path: "unknown".into(),
            status: DiffStatus::Modified,
            hunks: std::mem::take(&mut hunks),
        });
    }
    OwnedDiff { files }
}

fn parse_hunk_header(line: &str) -> Option<(u32, u32)> {
    // @@ -a,b +c,d @@ → (c, d)
    let plus = line.find('+')? + 1;
    let rest = &line[plus..];
    let end = rest.find(' ').or_else(|| rest.find('@'))?;
    let nums = &rest[..end];
    let parts: Vec<&str> = nums.split(',').collect();
    let start: u32 = parts.first()?.parse().ok()?;
    let count: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    Some((start, count))
}
