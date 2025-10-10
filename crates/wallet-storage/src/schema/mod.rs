pub mod entities;
pub mod primitives;
pub mod tables;

// Re-export commonly used entities
pub use entities::{EntityBase, EntityUser, SyncMap, EntitySyncMap, SyncError};

// Common types to be refined during translation
pub type Id = String;
