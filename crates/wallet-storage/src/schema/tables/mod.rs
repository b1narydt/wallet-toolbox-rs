//! Database table definitions
//!
//! Translates TypeScript table interfaces to Rust structs.
//! Reference: wallet-toolbox/src/storage/schema/tables/

pub mod table_user;
pub mod table_sync_state;
pub mod table_transaction;
pub mod table_output_basket;
pub mod table_output_tag;
pub mod table_output;
pub mod table_tx_label;
pub mod table_tx_label_map;
pub mod table_output_tag_map;
pub mod table_commission;
pub mod table_proven_tx;
pub mod table_proven_tx_req;
pub mod table_monitor_event;
pub mod table_settings;
pub mod table_certificate;
pub mod table_certificate_field;

pub use table_user::TableUser;
pub use table_sync_state::{TableSyncState, SyncStatus};
pub use table_transaction::{TableTransaction, TransactionStatus};
pub use table_output_basket::TableOutputBasket;
pub use table_output_tag::TableOutputTag;
pub use table_output::{TableOutput, StorageProvidedBy};
pub use table_tx_label::TableTxLabel;
pub use table_tx_label_map::TableTxLabelMap;
pub use table_output_tag_map::TableOutputTagMap;
pub use table_commission::TableCommission;
pub use table_proven_tx::TableProvenTx;
pub use table_proven_tx_req::{TableProvenTxReq, ProvenTxReqStatus};
pub use table_monitor_event::TableMonitorEvent;
pub use table_settings::{TableSettings, Chain as SettingsChain, DbType};
pub use table_certificate::TableCertificate;
pub use table_certificate_field::TableCertificateField;
