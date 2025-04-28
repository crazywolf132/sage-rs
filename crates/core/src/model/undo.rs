//! Event-sourced undo/redo ledger

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UndoOp {
    CreateBranch {
        name: String,
    },
    DeleteBranch {
        name: String,
    },
    RenameBranch {
        old: String,
        new: String,
    },
    Restack {
        branch: String,
        old_base: String,
        new_base: String,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UndoLedger {
    ops: Vec<UndoOp>,
    redo: Vec<UndoOp>,
}

impl UndoLedger {
    pub fn push(&mut self, op: UndoOp) {
        self.ops.push(op);
        self.redo.clear(); // New op invalidates redo stack
    }

    pub fn undo(&mut self) -> Option<UndoOp> {
        self.ops.pop().map(|op| {
            self.redo.push(op.clone());
            op
        })
    }

    pub fn redo(&mut self) -> Option<UndoOp> {
        self.redo.pop().map(|op| {
            self.ops.push(op.clone());
            op
        })
    }
}
