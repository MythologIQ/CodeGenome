use std::path::Path;

use codegenome_substrate::federation::report;

pub fn run(store_dir: &str, json: bool) {
    match report::load_report(Path::new(store_dir)) {
        Ok(rep) if json => {
            println!("{}", serde_json::to_string_pretty(&rep).unwrap_or_default());
        }
        Ok(rep) => {
            println!("Repositories: {}", rep.repository_count);
            println!("Federated edges: {}", rep.federated_edge_count);
            println!("Dependency coverage: {:.3}", rep.dependency_coverage);
            println!("Trace depth mean: {:.3}", rep.cross_repo_trace_depth.mean);
            for (source, count) in rep.evidence_source_counts {
                println!("{source}: {count}");
            }
        }
        Err(e) => {
            eprintln!("Workspace report failed: {e}");
            std::process::exit(1);
        }
    }
}
