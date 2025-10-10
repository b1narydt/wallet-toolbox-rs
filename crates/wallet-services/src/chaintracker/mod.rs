//! ChainTracker Service Module
//!
//! **Reference**: TypeScript `src/services/chaintracker/`
//!
//! Provides blockchain state tracking and merkle proof verification

pub mod chaintracks;
pub mod types;

pub use chaintracks::ChaintracksClient;
pub use types::*;
