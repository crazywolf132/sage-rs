//! A minimal, extensible data-model for **stack-based Git workflows**.
//!
//! ```no_run
//! use sage_stack::{BranchStatus, StackGraph};
//!
//! // Load the graph (or start empty) from a repo's .git directory.
//! let mut graph = StackGraph::load_or_default(".")?;
//!
//! // Create a new stack "payments", rooted at branch "payments/base".
//! let payments = graph.new_stack("payments", "payments/base")?;
//!
//! // Add a feature branch under the root.
//! payments.add_child("payments/base", "feat/credit-limits", None)?;
//!
//! // Persist back to disk.
//! graph.save(".")?;
//! # Ok::<(), stack_sage::StackError>(())
//! ```
pub mod branch;
pub mod error;
pub mod persist;
pub mod stack;

pub use branch::{BranchId, BranchInfo, BranchStatus};
pub use error::StackError;
pub use stack::{Stack, StackGraph};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_stack_and_children() {
        let mut g = StackGraph::default();
        let payments = g
            .new_stack("payments", "payments/base".into())
            .expect("stack");

        payments
            .add_child("payments/base", "feat/a".into(), Some("alice".into()))
            .unwrap();
        payments
            .add_child("feat/a", "feat/a-1".into(), Some("bob".into()))
            .unwrap();

        assert!(payments.contains_branch("feat/a-1"));
        assert_eq!(payments.children_of("payments/base"), &["feat/a"]);
        assert_eq!(
            payments
                .descendants("payments/base")
                .cloned()
                .collect::<Vec<_>>(),
            vec!["payments/base", "feat/a", "feat/a-1"]
        );
    }
}
