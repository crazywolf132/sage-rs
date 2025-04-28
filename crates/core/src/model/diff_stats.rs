use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct DiffStats {
    pub added: usize,
    pub removed: usize,
    pub files: usize,
}
