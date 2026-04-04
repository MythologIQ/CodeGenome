use std::path::PathBuf;

use crate::graph::edge::{Edge, Relation};
use crate::graph::node::{Node, NodeKind, Provenance, Source, Timestamp};
use crate::graph::overlay::{Overlay, OverlayKind};
use crate::identity::{address_of, UorAddress};
use crate::measurement::GroundTruthLevel;
use crate::overlay::flow_dfg;
use crate::overlay::flow_extract::*;

pub struct FlowOverlay {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Overlay for FlowOverlay {
    fn kind(&self) -> OverlayKind {
        OverlayKind::Flow
    }
    fn nodes(&self) -> &[Node] {
        &self.nodes
    }
    fn edges(&self) -> &[Edge] {
        &self.edges
    }
    fn ground_truth(&self) -> GroundTruthLevel {
        GroundTruthLevel::Available
    }
}

impl FlowOverlay {
    /// Build flow overlay from source files. Pure function.
    pub fn from_source(files: &[(PathBuf, Vec<u8>)]) -> Self {
        let prov = Provenance {
            source: Source::ToolOutput,
            actor: "flow-extractor".into(),
            timestamp: Timestamp(0),
            justification: None,
        };

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (path, source) in files {
            let Some(tree) = parse_file(source) else {
                continue;
            };
            let cfg = extract_control_flow(source, &tree);
            let dfg = flow_dfg::extract_data_flow(source, &tree);

            for cf in &cfg {
                let (src, tgt) = stmt_node_pair(
                    path, &cf.source_span, &cf.target_span, &prov,
                    &mut nodes,
                );
                edges.push(Edge {
                    source: src,
                    target: tgt,
                    relation: Relation::ControlFlow,
                    confidence: 1.0,
                    provenance: prov.clone(),
                    evidence: vec![],
                });
            }
            for df in &dfg {
                let (src, tgt) = stmt_node_pair(
                    path, &df.def_span, &df.use_span, &prov,
                    &mut nodes,
                );
                edges.push(Edge {
                    source: src,
                    target: tgt,
                    relation: Relation::DataFlow,
                    confidence: 1.0,
                    provenance: prov.clone(),
                    evidence: vec![],
                });
            }
        }

        Self { nodes, edges }
    }
}

/// Create or reuse statement-level nodes for edge endpoints.
fn stmt_node_pair(
    path: &std::path::Path,
    src_span: &crate::graph::node::Span,
    tgt_span: &crate::graph::node::Span,
    prov: &Provenance,
    nodes: &mut Vec<Node>,
) -> (UorAddress, UorAddress) {
    let src_addr = stmt_address(path, src_span);
    let tgt_addr = stmt_address(path, tgt_span);
    ensure_node(src_addr, src_span, prov, nodes);
    ensure_node(tgt_addr, tgt_span, prov, nodes);
    (src_addr, tgt_addr)
}

fn ensure_node(
    addr: UorAddress,
    span: &crate::graph::node::Span,
    prov: &Provenance,
    nodes: &mut Vec<Node>,
) {
    if !nodes.iter().any(|n| n.address == addr) {
        nodes.push(Node {
            address: addr,
            kind: NodeKind::Symbol,
            provenance: prov.clone(),
            confidence: 1.0,
            created_at: Timestamp(0),
            content_hash: addr,
            span: Some(*span),
        });
    }
}

fn stmt_address(
    path: &std::path::Path,
    span: &crate::graph::node::Span,
) -> UorAddress {
    let content = format!(
        "stmt:{}:{}:{}",
        path.display(),
        span.start_byte,
        span.end_byte,
    );
    address_of(content.as_bytes())
}

fn parse_file(source: &[u8]) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .ok()?;
    parser.parse(source, None)
}
