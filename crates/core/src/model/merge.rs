/// Result of a merge attempted by the Git adapter.
#[derive(Clone, Copy, Debug)]
pub enum MergeResult {
    /// Fast-forward merge (no changes)
    FastForward,
    /// No-op merge (no changes)
    NoOp,
    /// Conflicting merge (manual resolution required)
    Conflict,
}
