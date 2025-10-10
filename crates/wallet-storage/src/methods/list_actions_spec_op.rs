use crate::StorageError;
use serde::{Deserialize, Serialize};

// Placeholder type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActionsResult {
    pub actions: Vec<String>,
}

/// Placeholder for listActionsSpecOp method
pub fn list_actions_spec_op() -> Result<ListActionsResult, StorageError> {
    Err(StorageError::NotImplemented("listActionsSpecOp"))
}
