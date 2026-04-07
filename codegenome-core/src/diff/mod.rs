pub mod mapper;
pub mod propagator;
mod types;

pub use mapper::detect_changes;
pub use types::{ChangeSet, DiffStatus, OwnedDiff, OwnedDiffFile, OwnedHunk};
