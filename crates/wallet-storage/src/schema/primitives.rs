// Placeholder primitive types for storage schema
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Json(pub String);
