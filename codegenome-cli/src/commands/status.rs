use codegenome_core::store::backend::StoreBackend;
use codegenome_core::store::ondisk::OnDiskStore;

pub fn run(store_dir: &str, json: bool) {
    let store = OnDiskStore::new(store_dir);
    let kinds = match store.list_overlays() {
        Ok(k) => k,
        Err(e) => {
            eprintln!("No index found at {store_dir}: {e}");
            return;
        }
    };

    let mut entries = Vec::new();
    for kind in &kinds {
        let (nodes, edges) = match store.read_overlay(kind) {
            Ok(Some((n, e))) => (n.len(), e.len()),
            _ => (0, 0),
        };
        entries.push((format!("{kind:?}"), nodes, edges));
    }

    if json {
        print_json(&entries);
    } else {
        print_table(&entries);
    }
}

fn print_table(entries: &[(String, usize, usize)]) {
    println!("{:<20} {:>8} {:>8}", "Overlay", "Nodes", "Edges");
    println!("{}", "-".repeat(38));
    for (name, nodes, edges) in entries {
        println!("{name:<20} {nodes:>8} {edges:>8}");
    }
}

fn print_json(entries: &[(String, usize, usize)]) {
    let items: Vec<_> = entries
        .iter()
        .map(|(name, nodes, edges)| {
            serde_json::json!({"overlay": name, "nodes": nodes, "edges": edges})
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items).unwrap_or_default());
}
