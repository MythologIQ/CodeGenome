use std::path::PathBuf;

use crate::graph::edge::Edge;
use crate::graph::node::{Node, NodeKind, Provenance, Source, Timestamp};
use crate::graph::overlay::{Overlay, OverlayKind};
use crate::identity::address_of;
use crate::measurement::GroundTruthLevel;
use crate::overlay::syntax_extract::extract_symbols;

/// Syntax overlay: tree-sitter parse results as graph nodes
/// and edges. Implements the Overlay trait.
pub struct SyntaxOverlay {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl SyntaxOverlay {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

impl Overlay for SyntaxOverlay {
    fn kind(&self) -> OverlayKind {
        OverlayKind::Syntax
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

/// Parses Rust source files into a syntax overlay.
/// Pure function: files in -> overlay out.
pub fn parse_rust_files(
    files: &[(PathBuf, Vec<u8>)],
) -> SyntaxOverlay {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Failed to load Rust grammar");

    let provenance = Provenance {
        source: Source::ToolOutput,
        actor: "tree-sitter-rust".into(),
        timestamp: Timestamp(0),
        justification: None,
    };

    let mut all_nodes = Vec::new();
    let mut all_edges = Vec::new();

    for (path, source) in files {
        let file_content = format!("file:{}", path.display());
        let file_address = address_of(file_content.as_bytes());

        all_nodes.push(Node {
            address: file_address,
            kind: NodeKind::File,
            provenance: provenance.clone(),
            confidence: 1.0,
            created_at: Timestamp(0),
            content_hash: address_of(source),
            span: None,
        });

        let Some(tree) = parser.parse(source, None) else {
            continue;
        };

        let (sym_nodes, sym_edges) =
            extract_symbols(file_address, source, &tree, &provenance);
        all_nodes.extend(sym_nodes);
        all_edges.extend(sym_edges);
    }

    SyntaxOverlay {
        nodes: all_nodes,
        edges: all_edges,
    }
}
