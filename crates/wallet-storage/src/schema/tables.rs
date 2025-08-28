// Placeholder table types for storage schema
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableTransaction {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableProvenTx {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableProvenTxReq {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableOutput {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableOutputX {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableOutputBasket {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableOutputTag {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableTxLabel {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableCertificate {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableMonitorEvent {}
