use std::path::PathBuf;

use codegenome_core::graph::overlay::{Overlay, OverlayKind};
use codegenome_core::overlay::flow::FlowOverlay;
use codegenome_core::overlay::fused;
use codegenome_core::overlay::semantic::SemanticOverlay;
use codegenome_core::overlay::syntax::parse_rust_files;
use codegenome_core::store::meta::{self, IndexMeta};
use codegenome_core::store::ondisk::OnDiskStore;
use codegenome_core::store::backend::StoreBackend;

pub fn run(source_dir: &str, store_dir: &str) {
    let start = std::time::Instant::now();
    let files = collect_rs_files(std::path::Path::new(source_dir));
    if files.is_empty() {
        eprintln!("No Rust source files found in {source_dir}");
        return;
    }

    let syntax = parse_rust_files(&files);
    let semantic = SemanticOverlay::from_syntax(&syntax, &files);
    let flow = FlowOverlay::from_source(&files);
    let fused = fused::fuse(&[&syntax, &semantic, &flow]);

    let store = OnDiskStore::new(store_dir);
    let _ = store.write_overlay(&OverlayKind::Custom("fused".into()), fused.nodes(), fused.edges());

    let source_hashes = meta::hash_source_files(std::path::Path::new(source_dir));
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let index_meta = IndexMeta {
        timestamp: now,
        file_count: files.len(),
        node_count: fused.nodes().len(),
        edge_count: fused.edges().len(),
        source_hashes,
    };
    let _ = meta::save(std::path::Path::new(store_dir), &index_meta);

    let elapsed = start.elapsed();
    println!("Indexed {} files → {} nodes, {} edges ({:.0}ms)",
        files.len(), fused.nodes().len(), fused.edges().len(),
        elapsed.as_millis(),
    );
}

fn collect_rs_files(dir: &std::path::Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_rs_files(&path));
            } else if path.extension().is_some_and(|e| e == "rs") {
                if let Ok(content) = std::fs::read(&path) {
                    files.push((path, content));
                }
            }
        }
    }
    files
}
