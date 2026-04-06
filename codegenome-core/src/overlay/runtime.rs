use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::graph::edge::{Edge, Relation};
use crate::graph::node::{Node, Provenance, Source, Timestamp};
use crate::graph::overlay::{Overlay, OverlayKind};
use crate::identity::{address_of, UorAddress};
use crate::measurement::GroundTruthLevel;

pub struct RuntimeOverlay {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Overlay for RuntimeOverlay {
    fn kind(&self) -> OverlayKind { OverlayKind::Runtime }
    fn nodes(&self) -> &[Node] { &self.nodes }
    fn edges(&self) -> &[Edge] { &self.edges }
    fn ground_truth(&self) -> GroundTruthLevel { GroundTruthLevel::Available }
}

impl RuntimeOverlay {
    /// Parse a TSV trace file into runtime Calls edges.
    /// Format: caller\tcallee\tcount\tduration_ns
    pub fn from_trace_file(
        trace_path: &Path,
        source_files: &[(PathBuf, Vec<u8>)],
    ) -> Result<Self, String> {
        let file = std::fs::File::open(trace_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let names = build_name_index(source_files);
        let prov = Provenance {
            source: Source::ToolOutput,
            actor: "runtime-trace".into(),
            timestamp: Timestamp(0),
            justification: None,
        };

        let mut edges = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| e.to_string())?;
            if i == 0 || line.trim().is_empty() { continue; }
            if let Some(edge) = parse_trace_line(&line, &names, &prov) {
                edges.push(edge);
            }
        }
        Ok(Self { nodes: Vec::new(), edges })
    }
}

fn parse_trace_line(
    line: &str,
    names: &HashMap<String, UorAddress>,
    prov: &Provenance,
) -> Option<Edge> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 3 { return None; }
    let caller = names.get(parts[0])?;
    let callee = names.get(parts[1])?;
    let count: f64 = parts[2].parse().ok()?;
    let confidence = (count / 10.0).min(1.0);

    Some(Edge {
        source: *caller,
        target: *callee,
        relation: Relation::Calls,
        confidence,
        provenance: prov.clone(),
        evidence: vec![],
    })
}

fn build_name_index(files: &[(PathBuf, Vec<u8>)]) -> HashMap<String, UorAddress> {
    let mut index = HashMap::new();
    for (_, source) in files {
        let Some(tree) = parse_file(source) else { continue };
        let root = tree.root_node();
        let mut cursor = root.walk();
        for child in root.children(&mut cursor) {
            if let Some(name) = child.child_by_field_name("name")
                .and_then(|n| n.utf8_text(source).ok())
            {
                let content = format!("{}:{}", child.kind(), name);
                index.insert(name.to_string(), address_of(content.as_bytes()));
            }
        }
    }
    index
}

fn parse_file(source: &[u8]) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).ok()?;
    parser.parse(source, None)
}
