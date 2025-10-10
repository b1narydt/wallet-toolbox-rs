//! Database migrations for SQLite
//!
//! Translates TypeScript KnexMigrations to Rust SQL statements.
//! Reference: wallet-toolbox/src/storage/schema/KnexMigrations.ts

use rusqlite::Connection;
use wallet_storage::StorageError;

/// SQL for initial database schema creation
///
/// Matches TypeScript '2024-12-26-001 initial migration'
pub const INITIAL_MIGRATION: &str = r#"
-- proven_txs table
CREATE TABLE IF NOT EXISTS proven_txs (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    provenTxId INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    txid TEXT NOT NULL UNIQUE,
    height INTEGER NOT NULL,
    `index` INTEGER NOT NULL,
    merklePath BLOB NOT NULL,
    rawTx BLOB NOT NULL,
    blockHash TEXT NOT NULL,
    merkleRoot TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_proven_txs_blockHash ON proven_txs(blockHash);

-- proven_tx_reqs table  
CREATE TABLE IF NOT EXISTS proven_tx_reqs (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    provenTxReqId INTEGER PRIMARY KEY AUTOINCREMENT,
    provenTxId INTEGER REFERENCES proven_txs(provenTxId),
    status TEXT NOT NULL DEFAULT 'unknown',
    attempts INTEGER NOT NULL DEFAULT 0,
    notified INTEGER NOT NULL DEFAULT 0,
    txid TEXT NOT NULL UNIQUE,
    batch TEXT,
    history TEXT NOT NULL DEFAULT '{}',
    notify TEXT NOT NULL DEFAULT '{}',
    rawTx BLOB NOT NULL,
    inputBEEF BLOB
);

CREATE INDEX IF NOT EXISTS idx_proven_tx_reqs_status ON proven_tx_reqs(status);
CREATE INDEX IF NOT EXISTS idx_proven_tx_reqs_batch ON proven_tx_reqs(batch);

-- users table
CREATE TABLE IF NOT EXISTS users (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    userId INTEGER PRIMARY KEY AUTOINCREMENT,
    identityKey TEXT NOT NULL UNIQUE,
    activeStorage TEXT NOT NULL
);

-- certificates table
CREATE TABLE IF NOT EXISTS certificates (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    certificateId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    serialNumber TEXT NOT NULL,
    type TEXT NOT NULL,
    certifier TEXT NOT NULL,
    subject TEXT NOT NULL,
    verifier TEXT,
    revocationOutpoint TEXT NOT NULL,
    signature TEXT NOT NULL,
    isDeleted INTEGER NOT NULL DEFAULT 0,
    UNIQUE(userId, type, certifier, serialNumber)
);

-- certificate_fields table
CREATE TABLE IF NOT EXISTS certificate_fields (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    userId INTEGER NOT NULL REFERENCES users(userId),
    certificateId INTEGER NOT NULL REFERENCES certificates(certificateId),
    fieldName TEXT NOT NULL,
    fieldValue TEXT NOT NULL,
    masterKey TEXT NOT NULL DEFAULT '',
    UNIQUE(fieldName, certificateId)
);

-- output_baskets table
CREATE TABLE IF NOT EXISTS output_baskets (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    basketId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    name TEXT NOT NULL,
    numberOfDesiredUTXOs INTEGER NOT NULL DEFAULT 6,
    minimumDesiredUTXOValue INTEGER NOT NULL DEFAULT 10000,
    isDeleted INTEGER NOT NULL DEFAULT 0,
    UNIQUE(name, userId)
);

-- transactions table
CREATE TABLE IF NOT EXISTS transactions (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    transactionId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    provenTxId INTEGER REFERENCES proven_txs(provenTxId),
    status TEXT NOT NULL,
    reference TEXT NOT NULL UNIQUE,
    isOutgoing INTEGER NOT NULL,
    satoshis INTEGER NOT NULL DEFAULT 0,
    version INTEGER,
    lockTime INTEGER,
    description TEXT NOT NULL,
    txid TEXT,
    inputBEEF BLOB,
    rawTx BLOB
);

CREATE INDEX IF NOT EXISTS idx_transactions_status ON transactions(status);

-- commissions table
CREATE TABLE IF NOT EXISTS commissions (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    commissionId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    transactionId INTEGER NOT NULL UNIQUE REFERENCES transactions(transactionId),
    satoshis INTEGER NOT NULL,
    keyOffset TEXT NOT NULL,
    isRedeemed INTEGER NOT NULL DEFAULT 0,
    lockingScript BLOB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_commissions_transactionId ON commissions(transactionId);

-- outputs table
CREATE TABLE IF NOT EXISTS outputs (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    outputId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    transactionId INTEGER NOT NULL REFERENCES transactions(transactionId),
    basketId INTEGER REFERENCES output_baskets(basketId),
    spendable INTEGER NOT NULL DEFAULT 0,
    `change` INTEGER NOT NULL DEFAULT 0,
    vout INTEGER NOT NULL,
    satoshis INTEGER NOT NULL,
    providedBy TEXT NOT NULL,
    purpose TEXT NOT NULL,
    type TEXT NOT NULL,
    outputDescription TEXT,
    txid TEXT,
    senderIdentityKey TEXT,
    derivationPrefix TEXT,
    derivationSuffix TEXT,
    customInstructions TEXT,
    spentBy INTEGER REFERENCES transactions(transactionId),
    sequenceNumber INTEGER,
    spendingDescription TEXT,
    scriptLength INTEGER,
    scriptOffset INTEGER,
    lockingScript BLOB,
    UNIQUE(transactionId, vout, userId)
);

-- output_tags table
CREATE TABLE IF NOT EXISTS output_tags (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    outputTagId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    tag TEXT NOT NULL,
    isDeleted INTEGER NOT NULL DEFAULT 0,
    UNIQUE(tag, userId)
);

-- output_tags_map table
CREATE TABLE IF NOT EXISTS output_tags_map (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    outputTagId INTEGER NOT NULL REFERENCES output_tags(outputTagId),
    outputId INTEGER NOT NULL REFERENCES outputs(outputId),
    isDeleted INTEGER NOT NULL DEFAULT 0,
    UNIQUE(outputTagId, outputId)
);

CREATE INDEX IF NOT EXISTS idx_output_tags_map_outputId ON output_tags_map(outputId);

-- tx_labels table
CREATE TABLE IF NOT EXISTS tx_labels (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    txLabelId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    label TEXT NOT NULL,
    isDeleted INTEGER NOT NULL DEFAULT 0,
    UNIQUE(label, userId)
);

-- tx_labels_map table
CREATE TABLE IF NOT EXISTS tx_labels_map (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    txLabelId INTEGER NOT NULL REFERENCES tx_labels(txLabelId),
    transactionId INTEGER NOT NULL REFERENCES transactions(transactionId),
    isDeleted INTEGER NOT NULL DEFAULT 0,
    UNIQUE(txLabelId, transactionId)
);

CREATE INDEX IF NOT EXISTS idx_tx_labels_map_transactionId ON tx_labels_map(transactionId);

-- monitor_events table
CREATE TABLE IF NOT EXISTS monitor_events (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event TEXT NOT NULL,
    details TEXT
);

CREATE INDEX IF NOT EXISTS idx_monitor_events_event ON monitor_events(event);

-- settings table
CREATE TABLE IF NOT EXISTS settings (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    storageIdentityKey TEXT NOT NULL,
    storageName TEXT NOT NULL,
    chain TEXT NOT NULL,
    dbtype TEXT NOT NULL,
    maxOutputScript INTEGER NOT NULL
);

-- sync_states table
CREATE TABLE IF NOT EXISTS sync_states (
    created_at TEXT NOT NULL DEFAULT(datetime('now')),
    updated_at TEXT NOT NULL DEFAULT(datetime('now')),
    syncStateId INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL REFERENCES users(userId),
    storageIdentityKey TEXT NOT NULL DEFAULT '',
    storageName TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'unknown',
    init INTEGER NOT NULL DEFAULT 0,
    refNum TEXT NOT NULL UNIQUE,
    syncMap TEXT NOT NULL,
    `when` TEXT,
    satoshis INTEGER,
    errorLocal TEXT,
    errorOther TEXT
);

CREATE INDEX IF NOT EXISTS idx_sync_states_status ON sync_states(status);
CREATE INDEX IF NOT EXISTS idx_sync_states_refNum ON sync_states(refNum);
"#;

/// Apply initial migration and insert settings
pub fn apply_initial_migration(
    conn: &Connection,
    storage_identity_key: &str,
    storage_name: &str,
    chain: &str,
    max_output_script: i64,
) -> Result<(), StorageError> {
    conn.execute_batch(INITIAL_MIGRATION)
        .map_err(|e| StorageError::Database(format!("Migration failed: {}", e)))?;

    // Insert initial settings
    conn.execute(
        "INSERT INTO settings (storageIdentityKey, storageName, chain, dbtype, maxOutputScript) 
         VALUES (?1, ?2, ?3, 'SQLite', ?4)",
        rusqlite::params![storage_identity_key, storage_name, chain, max_output_script],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert settings: {}", e)))?;

    Ok(())
}

/// Check if database is initialized
pub fn is_initialized(conn: &Connection) -> Result<bool, StorageError> {
    let result: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='settings'",
        [],
        |row| row.get(0),
    );

    match result {
        Ok(count) => Ok(count > 0),
        Err(e) => Err(StorageError::Database(format!("Failed to check initialization: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_initial_migration() {
        let conn = Connection::open_in_memory().unwrap();
        
        apply_initial_migration(
            &conn,
            "test_key_12345",
            "Test Storage",
            "main",
            100000,
        )
        .unwrap();

        // Verify key tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        // Verify all expected tables
        let expected_tables = vec![
            "users", "transactions", "outputs", "certificates", "certificate_fields",
            "output_baskets", "output_tags", "output_tags_map", "tx_labels", "tx_labels_map",
            "proven_txs", "proven_tx_reqs", "commissions", "sync_states", "settings", "monitor_events"
        ];
        
        for table in &expected_tables {
            assert!(tables.contains(&table.to_string()), "Missing table: {}", table);
        }
        
        // SQLite creates some internal tables, so just verify we have at least our 16 tables
        assert!(tables.len() >= 16, "Expected at least 16 tables, found {}", tables.len());
    }

    #[test]
    fn test_is_initialized() {
        let conn = Connection::open_in_memory().unwrap();
        
        // Should be false initially
        assert!(!is_initialized(&conn).unwrap());

        // Apply migration
        apply_initial_migration(&conn, "key", "name", "main", 100).unwrap();

        // Should be true after migration
        assert!(is_initialized(&conn).unwrap());
    }

    #[test]
    fn test_settings_inserted() {
        let conn = Connection::open_in_memory().unwrap();
        
        apply_initial_migration(
            &conn,
            "my_storage_key",
            "My Storage",
            "test",
            50000,
        )
        .unwrap();

        // Verify settings
        let (key, name, chain, max_script): (String, String, String, i64) = conn
            .query_row(
                "SELECT storageIdentityKey, storageName, chain, maxOutputScript FROM settings",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();

        assert_eq!(key, "my_storage_key");
        assert_eq!(name, "My Storage");
        assert_eq!(chain, "test");
        assert_eq!(max_script, 50000);
    }
}
