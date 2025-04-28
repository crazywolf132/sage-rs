//! Tiny diff stat generator - placeholder.

use crate::model::DiffStats;

pub fn stats(_a: &str, _b: &str) -> DiffStats {
    // TODO: Real implementation
    DiffStats {
        added: 0,
        removed: 0,
        files: 0,
    }
}
