pub mod mapper;
mod types;

pub use mapper::detect_changes;
pub use types::{ChangeSet, DiffStatus, OwnedDiff, OwnedDiffFile, OwnedHunk};
