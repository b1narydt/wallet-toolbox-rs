// Placeholder entity structs to be filled during translation from TS
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityTransaction {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityProvenTx {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityProvenTxReq {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntitySyncState {}
