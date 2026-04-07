use codegenome_core::belief::create::{create_belief, BeliefSpec};
use codegenome_core::belief::store::persist_beliefs;
use codegenome_core::governance::write_gate::WriteGatePolicy;
use codegenome_core::store::meta;
use codegenome_core::store::ondisk::OnDiskStore;

use crate::tools::inputs::AssertInput;
use crate::tools::CodegenomeTools;

impl CodegenomeTools {
    /// Write-gated: create a belief about a code artifact.
    pub fn assert_belief(&self, input: &AssertInput) -> String {
        let Some((_overlay, index)) = self.load_with_index() else {
            return r#"{"error":"no index found"}"#.into();
        };
        let Some(subject) = index.resolve(
            &input.subject_file, input.subject_line,
        ) else {
            return format!(
                r#"{{"error":"no symbol at {}:{}"}}"#,
                input.subject_file, input.subject_line
            );
        };

        // Write gate check
        let policy = WriteGatePolicy::default_policy();
        let freshness = meta::check_freshness(
            &self.store_dir, &self.source_dir,
        );
        let request = codegenome_core::governance::write_gate::WriteRequest {
            actor: input.actor.clone(),
            toolchain_version: "belief-api".into(),
            source_freshness: freshness,
            min_edge_confidence: input.confidence,
        };
        let decision = policy.evaluate(&request);
        if let codegenome_core::governance::policy::Decision::Deny(reason) = &decision {
            return serde_json::json!({
                "error": "write denied",
                "reason": reason,
            }).to_string();
        }

        let spec = BeliefSpec {
            claim: input.claim.clone(),
            subject,
            confidence: input.confidence,
            supporting_evidence: vec![],
            contradicting_evidence: vec![],
            actor: input.actor.clone(),
        };
        let belief = create_belief(&spec);
        let addr = belief.0.address;

        let store = OnDiskStore::new(&self.store_dir);
        if let Err(e) = persist_beliefs(&store, &[belief]) {
            return serde_json::json!({"error": e}).to_string();
        }

        serde_json::json!({
            "status": "created",
            "belief_address": format!("{addr:?}"),
            "claim": input.claim,
            "subject": format!("{}:{}", input.subject_file, input.subject_line),
            "confidence": input.confidence,
        }).to_string()
    }
}
