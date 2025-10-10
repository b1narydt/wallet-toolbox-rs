//! TableSettings - Storage settings and configuration
//!
//! Translates TypeScript TableSettings interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableSettings.ts

use serde::{Deserialize, Serialize};

/// Chain type (local copy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Main,
    Test,
}

impl std::str::FromStr for Chain {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "main" => Ok(Chain::Main),
            "test" => Ok(Chain::Test),
            _ => Err(format!("Invalid chain: {}", s)),
        }
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Main => write!(f, "main"),
            Chain::Test => write!(f, "test"),
        }
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for Chain {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let s = value.as_str()?;
        s.parse().map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        )))
    }
}

/// Database type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DbType {
    SQLite,
    MySQL,
    IndexedDB,
}

impl std::str::FromStr for DbType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SQLite" => Ok(DbType::SQLite),
            "MySQL" => Ok(DbType::MySQL),
            "IndexedDB" => Ok(DbType::IndexedDB),
            _ => Err(format!("Invalid dbtype: {}", s)),
        }
    }
}

impl std::fmt::Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbType::SQLite => write!(f, "SQLite"),
            DbType::MySQL => write!(f, "MySQL"),
            DbType::IndexedDB => write!(f, "IndexedDB"),
        }
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for DbType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let s = value.as_str()?;
        s.parse().map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        )))
    }
}

/// Settings table - stores storage configuration
///
/// Matches TypeScript `TableSettings` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableSettings {
    pub created_at: String,
    pub updated_at: String,
    
    /// The identity key (public key) assigned to this storage
    #[serde(rename = "storageIdentityKey")]
    pub storage_identity_key: String,
    
    /// The human readable name assigned to this storage
    #[serde(rename = "storageName")]
    pub storage_name: String,
    
    pub chain: Chain,
    
    pub dbtype: DbType,
    
    #[serde(rename = "maxOutputScript")]
    pub max_output_script: i64,
}

impl TableSettings {
    pub fn new(
        storage_identity_key: impl Into<String>,
        storage_name: impl Into<String>,
        chain: Chain,
        dbtype: DbType,
        max_output_script: i64,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            storage_identity_key: storage_identity_key.into(),
            storage_name: storage_name.into(),
            chain,
            dbtype,
            max_output_script,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_settings_new() {
        let settings = TableSettings::new(
            "key123", "MyWallet", Chain::Main, DbType::SQLite, 10000
        );
        assert_eq!(settings.storage_identity_key, "key123");
        assert_eq!(settings.storage_name, "MyWallet");
        assert_eq!(settings.chain, Chain::Main);
        assert_eq!(settings.dbtype, DbType::SQLite);
        assert_eq!(settings.max_output_script, 10000);
    }

    #[test]
    fn test_table_settings_serialization() {
        let settings = TableSettings::new(
            "key", "name", Chain::Test, DbType::MySQL, 5000
        );
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"storageIdentityKey\":\"key\""));
        assert!(json.contains("\"storageName\":\"name\""));
        assert!(json.contains("\"chain\":\"test\""));
        let deserialized: TableSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_chain_serialization() {
        assert_eq!(
            serde_json::to_string(&Chain::Main).unwrap(),
            "\"main\""
        );
        assert_eq!(
            serde_json::to_string(&Chain::Test).unwrap(),
            "\"test\""
        );
    }

    #[test]
    fn test_dbtype_serialization() {
        assert_eq!(
            serde_json::to_string(&DbType::SQLite).unwrap(),
            "\"SQLite\""
        );
        assert_eq!(
            serde_json::to_string(&DbType::MySQL).unwrap(),
            "\"MySQL\""
        );
        assert_eq!(
            serde_json::to_string(&DbType::IndexedDB).unwrap(),
            "\"IndexedDB\""
        );
    }
}
