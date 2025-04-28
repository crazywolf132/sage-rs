//! High-level undo orchestration (wraps UndoLedger)

use crate::{error::CoreError, model::*};

pub struct UndoService<'a> {
    ledger: &'a mut UndoLedger,
}

impl<'a> UndoService<'a> {
    pub fn new(ledger: &'a mut UndoLedger) -> Self {
        Self { ledger }
    }
    pub fn record(&mut self, op: UndoOp) {
        self.ledger.push(op)
    }
    pub fn undo(&mut self) -> Result<UndoOp, CoreError> {
        self.ledger
            .undo()
            .ok_or(CoreError::InvalidOp("nothing to undo".into()))
    }
}
