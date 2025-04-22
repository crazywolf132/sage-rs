use anyhow::{anyhow, Result};
use chrono::Utc;
use uuid::Uuid;

use crate::git;
use super::{History, Operation, OperationType, OperationMetadata};

impl History {
    pub fn record_operation(&mut self, op_type: OperationType, description: &str, command: &str, category: &str, metadata: OperationMetadata) -> Result<()> {
        // Get the current commit hash to use as a reference point for undoing.
        let ref_ = git::commit::hash()?;

        // Create a new operation record with a unique ID
        let id = Uuid::now_v7().to_string();
        let operation = Operation {
            id,
            type_: op_type,
            description: description.to_string(),
            command: command.to_string(),
            timestamp: Utc::now(),
            ref_,
            category: category.to_string(),
            metadata,
        };

        // Actually add the operation to the history
        self.add(operation)?;

        Ok(())
    }

    pub fn undo_operation(&mut self, id: &str) -> Result<()> {
        // Find the operation with the given ID
        let idx = self.operations.iter().position(|op| op.id == id)
            .ok_or(anyhow!("Operation not found"))?;
        let operation = self.operations[idx].clone();

        // Handle different operation types with specialized undo functions
        match operation.type_ {
            OperationType::Commit => self.undo_commit(&operation),
            OperationType::Merge => self.undo_merge(&operation),
            OperationType::Rebase => self.undo_rebase(&operation),
        }?;
        Ok(())
    }

    pub fn undo_last(&mut self, n: usize) -> Result<()> {
        // Check if n is valid
        if n == 0 {
            return Err(anyhow!("Invalid number of operations to undo"));
        }
        // Check if there are enough operations to undo
        if n > self.operations.len() {
            return Err(anyhow!("Not enough operations to undo"));
        }
        // Undo the operations one by one, always using the last valid operation
        for _ in 0..n {
            let op_id = match self.operations.last() {
                Some(op) => op.id.clone(),
                None => return Err(anyhow!("No operations to undo")),
            };
            self.undo_operation(&op_id)?;
            // Remove the operation after undoing it
            self.operations.pop();
        }
        Ok(())
    }

    /// undo_commit handles the undoing of a commit operation
    /// It resets the repository to the state before the commit was made.
    /// 
    /// Parameters:
    /// - op: The commit operation to undo
    /// 
    /// Returns:
    /// - Result<(), Error>: An error if the undo operation fails
    /// 
    /// The function handles stashed changes if they were part of the commit operation
    /// and resets to the parent of thee commit that was made.
    fn undo_commit(&mut self, operation: &Operation) -> Result<()> {

        // First we will do a soft reset using git.
        git::repo::reset_soft(&format!("{}~1", &operation.ref_))?;

        // Check if we need to handle stashed changes that are part of the commit
        if operation.metadata.stashed {
            git::stash::pop()?;
        }
        Ok(())
    }

    /// undo_merge handles the undoing of a merge operation
    /// 
    /// Parameters:
    /// - op: The merge operation to undo
    /// 
    /// Returns:
    /// - Result<(), Error>: An error if the undo operation fails
    /// 
    /// The function handles stashed changes if they were part of the merge operation
    /// and resets to the parent of the merge that was made.
    fn undo_merge(&mut self, operation: &Operation) -> Result<()> {
        // Check if a merge is currently in progress
        if git::repo::is_merge_in_progress()? {
            return git::repo::abort_merge();
        }

        // If there merge is already completed, reset to the state before the merge
        git::repo::reset_soft(&format!("{}~1", &operation.ref_))?;
        Ok(())
    }


    /// undo_rebase handles the undoing of a rebase operation
    /// 
    /// Parameters:
    /// - op: The rebase operation to undo
    /// 
    /// Returns:
    /// - Result<(), Error>: An error if the undo operation fails
    /// 
    /// The function handles stashed changes if they were part of the rebase operation
    /// and resets to the state before the rebase was started.
    fn undo_rebase(&mut self, operation: &Operation) -> Result<()> {
        // Check if a rebase is currently in progress
        if git::repo::is_rebase_in_progress()? {
            return git::repo::abort_rebase();
        }

        // If there rebase is already completed, reset to the state before the rebase
        git::repo::reset_soft(&format!("{}~1", &operation.ref_))?;
        Ok(())
    }
}