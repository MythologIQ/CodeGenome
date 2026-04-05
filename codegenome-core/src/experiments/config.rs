use std::collections::HashMap;
use std::path::PathBuf;

use crate::graph::overlay::OverlayKind;

/// Immutable experiment infrastructure.
/// Like autoresearch's prepare.py — never modified.
pub struct ExperimentInfra {
    pub source_dir: PathBuf,
    pub overlays: Vec<OverlayKind>,
    pub fitness_fn: FitnessFunction,
    pub model_id: Option<String>,
}

/// Mutable experiment surface.
/// Like autoresearch's train.py — the thing that changes.
#[derive(Clone, Debug)]
pub struct ExperimentParams {
    pub values: HashMap<String, f64>,
}

impl Default for ExperimentParams {
    fn default() -> Self {
        let mut values = HashMap::new();
        values.insert("confidence_threshold".into(), 0.75);
        values.insert("max_depth".into(), 5.0);
        values.insert("attenuation_factor".into(), 0.85);
        Self { values }
    }
}

/// Fitness function selection.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FitnessFunction {
    /// Does impact prediction match actual breakage?
    ImpactAccuracy,
    /// How deep does signal penetrate?
    PropagationDepth,
    /// Parse + propagate latency in ms.
    CycleTime,
    /// Edges per node ratio.
    GraphDensity,
    /// Named custom function.
    Custom(String),
}

/// Status of an experiment run.
#[derive(Clone, Debug, PartialEq)]
pub enum ExperimentStatus {
    Pass,
    Fail,
    Inconclusive,
}

impl ExperimentParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, key: &str, value: f64) -> Self {
        self.values.insert(key.into(), value);
        self
    }
}
