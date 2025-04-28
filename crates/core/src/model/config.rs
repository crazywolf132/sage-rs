#[derive(Clone, Debug)]
pub struct StackConfig {
    pub history_depth: usize,
    pub prefix: String,
}

impl Default for StackConfig {
    fn default() -> Self {
        Self {
            history_depth: 250,
            prefix: "stack/".to_string(),
        }
    }
}
