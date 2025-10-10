//! SQLite storage implementation
//!
//! Translates TypeScript StorageKnex to Rust with rusqlite.
//! Reference: wallet-toolbox/src/storage/StorageKnex.ts

use async_trait::async_trait;
use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::{Arc, Mutex};
use wallet_storage::*;

use crate::migrations::{apply_initial_migration, is_initialized};
use crate::transaction_ops;
use crate::output_ops;
use crate::proven_tx_ops;
use crate::basket_tag_label_ops;
use crate::cert_commission_ops;

/// SQLite storage backend
///
/// Matches TypeScript `StorageKnex` class functionality
pub struct StorageSqlite {
    conn: Arc<Mutex<Connection>>,
    settings: Option<TableSettings>,
}

impl StorageSqlite {
    /// Create new SQLite storage from file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, StorageError> {
        let conn = Connection::open(path)
            .map_err(|e| StorageError::Database(format!("Failed to open database: {}", e)))?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| StorageError::Database(format!("Failed to enable foreign keys: {}", e)))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            settings: None,
        })
    }

    /// Create in-memory database (for testing)
    pub fn new_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| StorageError::Database(format!("Failed to create in-memory database: {}", e)))?;

        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| StorageError::Database(format!("Failed to enable foreign keys: {}", e)))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            settings: None,
        })
    }

    /// Initialize storage with settings
    pub fn initialize(
        &mut self,
        storage_identity_key: &str,
        storage_name: &str,
        chain: &str,
        max_output_script: i64,
    ) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        
        if !is_initialized(&conn)? {
            apply_initial_migration(
                &conn,
                storage_identity_key,
                storage_name,
                chain,
                max_output_script,
            )?;
        }

        drop(conn);

        // Load settings
        self.load_settings()?;

        Ok(())
    }

    fn load_settings(&mut self) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();

        let settings = conn.query_row(
            "SELECT created_at, updated_at, storageIdentityKey, storageName, chain, dbtype, maxOutputScript 
             FROM settings LIMIT 1",
            [],
            |row| {
                Ok(TableSettings {
                    created_at: row.get(0)?,
                    updated_at: row.get(1)?,
                    storage_identity_key: row.get(2)?,
                    storage_name: row.get(3)?,
                    chain: row.get(4)?,
                    dbtype: row.get(5)?,
                    max_output_script: row.get(6)?,
                })
            },
        )
        .map_err(|e| StorageError::Database(format!("Failed to load settings: {}", e)))?;

        self.settings = Some(settings);
        Ok(())
    }

    /// Insert a user
    pub fn insert_user(&self, identity_key: &str, active_storage: &str) -> Result<i64, StorageError> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO users (identityKey, activeStorage) VALUES (?1, ?2)",
            params![identity_key, active_storage],
        )
        .map_err(|e| StorageError::Database(format!("Failed to insert user: {}", e)))?;

        let user_id = conn.last_insert_rowid();
        Ok(user_id)
    }

    /// Find user by identity key
    pub fn find_user_by_identity(&self, identity_key: &str) -> Result<Option<TableUser>, StorageError> {
        let conn = self.conn.lock().unwrap();

        let result = conn.query_row(
            "SELECT created_at, updated_at, userId, identityKey, activeStorage 
             FROM users WHERE identityKey = ?1",
            params![identity_key],
            |row| {
                Ok(TableUser {
                    created_at: row.get(0)?,
                    updated_at: row.get(1)?,
                    user_id: row.get(2)?,
                    identity_key: row.get(3)?,
                    active_storage: row.get(4)?,
                })
            },
        );

        match result {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(StorageError::Database(format!("Failed to find user: {}", e))),
        }
    }

    /// Find user by ID
    pub fn find_user_by_id(&self, user_id: i64) -> Result<Option<TableUser>, StorageError> {
        let conn = self.conn.lock().unwrap();

        let result = conn.query_row(
            "SELECT created_at, updated_at, userId, identityKey, activeStorage 
             FROM users WHERE userId = ?1",
            params![user_id],
            |row| {
                Ok(TableUser {
                    created_at: row.get(0)?,
                    updated_at: row.get(1)?,
                    user_id: row.get(2)?,
                    identity_key: row.get(3)?,
                    active_storage: row.get(4)?,
                })
            },
        );

        match result {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(StorageError::Database(format!("Failed to find user by ID: {}", e))),
        }
    }

    /// Update user
    pub fn update_user(&self, user_id: i64, user: &TableUser) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE users 
             SET updated_at = datetime('now'), activeStorage = ?1 
             WHERE userId = ?2",
            params![user.active_storage, user_id],
        )
        .map_err(|e| StorageError::Database(format!("Failed to update user: {}", e)))?;

        Ok(())
    }

    /// Insert transaction
    pub fn insert_transaction(&self, user_id: i64, transaction: &TableTransaction) -> Result<i64, StorageError> {
        transaction_ops::insert_transaction(&self.conn, user_id, transaction)
    }

    /// Find transaction by ID
    pub fn find_transaction_by_id(&self, transaction_id: i64) -> Result<Option<TableTransaction>, StorageError> {
        transaction_ops::find_transaction_by_id(&self.conn, transaction_id)
    }

    /// Find transaction by reference
    pub fn find_transaction_by_reference(&self, reference: &str) -> Result<Option<TableTransaction>, StorageError> {
        transaction_ops::find_transaction_by_reference(&self.conn, reference)
    }

    /// Update transaction
    pub fn update_transaction(&self, transaction_id: i64, transaction: &TableTransaction) -> Result<(), StorageError> {
        transaction_ops::update_transaction(&self.conn, transaction_id, transaction)
    }

    /// Find transactions for user
    pub fn find_transactions_for_user(
        &self,
        user_id: i64,
        status_filter: Option<&TransactionStatus>,
        limit: Option<u32>,
    ) -> Result<Vec<TableTransaction>, StorageError> {
        transaction_ops::find_transactions_for_user(&self.conn, user_id, status_filter, limit)
    }

    /// Insert output
    pub fn insert_output(&self, output: &TableOutput) -> Result<i64, StorageError> {
        output_ops::insert_output(&self.conn, output)
    }

    /// Find output by ID
    pub fn find_output_by_id(&self, output_id: i64, no_script: bool) -> Result<Option<TableOutput>, StorageError> {
        output_ops::find_output_by_id(&self.conn, output_id, no_script)
    }

    /// Update output
    pub fn update_output(&self, output_id: i64, output: &TableOutput) -> Result<usize, StorageError> {
        output_ops::update_output(&self.conn, output_id, output)
    }

    /// Find outputs for transaction
    pub fn find_outputs_for_transaction(&self, transaction_id: i64, no_script: bool) -> Result<Vec<TableOutput>, StorageError> {
        output_ops::find_outputs_for_transaction(&self.conn, transaction_id, no_script)
    }

    /// Find spendable outputs for user
    pub fn find_spendable_outputs_for_user(
        &self,
        user_id: i64,
        basket_id: Option<i64>,
        limit: Option<u32>,
    ) -> Result<Vec<TableOutput>, StorageError> {
        output_ops::find_spendable_outputs_for_user(&self.conn, user_id, basket_id, limit)
    }

    /// Insert proven tx
    pub fn insert_proven_tx(&self, proven_tx: &TableProvenTx) -> Result<i64, StorageError> {
        proven_tx_ops::insert_proven_tx(&self.conn, proven_tx)
    }

    /// Find proven tx by txid
    pub fn find_proven_tx_by_txid(&self, txid: &str) -> Result<Option<TableProvenTx>, StorageError> {
        proven_tx_ops::find_proven_tx_by_txid(&self.conn, txid)
    }

    /// Insert proven tx req
    pub fn insert_proven_tx_req(&self, req: &TableProvenTxReq) -> Result<i64, StorageError> {
        proven_tx_ops::insert_proven_tx_req(&self.conn, req)
    }

    /// Update proven tx req
    pub fn update_proven_tx_req(&self, req_id: i64, req: &TableProvenTxReq) -> Result<usize, StorageError> {
        proven_tx_ops::update_proven_tx_req(&self.conn, req_id, req)
    }

    /// Find proven tx req by txid
    pub fn find_proven_tx_req_by_txid(&self, txid: &str) -> Result<Option<TableProvenTxReq>, StorageError> {
        proven_tx_ops::find_proven_tx_req_by_txid(&self.conn, txid)
    }

    /// Insert output basket
    pub fn insert_output_basket(&self, basket: &TableOutputBasket) -> Result<i64, StorageError> {
        basket_tag_label_ops::insert_output_basket(&self.conn, basket)
    }

    /// Find output basket by name
    pub fn find_output_basket_by_name(&self, user_id: i64, name: &str) -> Result<Option<TableOutputBasket>, StorageError> {
        basket_tag_label_ops::find_output_basket_by_name(&self.conn, user_id, name)
    }

    /// Insert output tag
    pub fn insert_output_tag(&self, tag: &TableOutputTag) -> Result<i64, StorageError> {
        basket_tag_label_ops::insert_output_tag(&self.conn, tag)
    }

    /// Find output tag by name
    pub fn find_output_tag_by_name(&self, user_id: i64, tag: &str) -> Result<Option<TableOutputTag>, StorageError> {
        basket_tag_label_ops::find_output_tag_by_name(&self.conn, user_id, tag)
    }

    /// Insert output tag map
    pub fn insert_output_tag_map(&self, map: &TableOutputTagMap) -> Result<(), StorageError> {
        basket_tag_label_ops::insert_output_tag_map(&self.conn, map)
    }

    /// Insert tx label
    pub fn insert_tx_label(&self, label: &TableTxLabel) -> Result<i64, StorageError> {
        basket_tag_label_ops::insert_tx_label(&self.conn, label)
    }

    /// Find tx label by name
    pub fn find_tx_label_by_name(&self, user_id: i64, label: &str) -> Result<Option<TableTxLabel>, StorageError> {
        basket_tag_label_ops::find_tx_label_by_name(&self.conn, user_id, label)
    }

    /// Insert tx label map
    pub fn insert_tx_label_map(&self, map: &TableTxLabelMap) -> Result<(), StorageError> {
        basket_tag_label_ops::insert_tx_label_map(&self.conn, map)
    }

    /// Insert certificate
    pub fn insert_certificate(&self, cert: &TableCertificate) -> Result<i64, StorageError> {
        cert_commission_ops::insert_certificate(&self.conn, cert)
    }

    /// Find certificate by id
    pub fn find_certificate_by_id(&self, cert_id: i64) -> Result<Option<TableCertificate>, StorageError> {
        cert_commission_ops::find_certificate_by_id(&self.conn, cert_id)
    }

    /// Insert certificate field
    pub fn insert_certificate_field(&self, field: &TableCertificateField) -> Result<(), StorageError> {
        cert_commission_ops::insert_certificate_field(&self.conn, field)
    }

    /// Find certificate fields
    pub fn find_certificate_fields(&self, cert_id: i64) -> Result<Vec<TableCertificateField>, StorageError> {
        cert_commission_ops::find_certificate_fields(&self.conn, cert_id)
    }

    /// Insert commission
    pub fn insert_commission(&self, commission: &TableCommission) -> Result<i64, StorageError> {
        cert_commission_ops::insert_commission(&self.conn, commission)
    }

    /// Find commission by transaction
    pub fn find_commission_by_transaction(&self, transaction_id: i64) -> Result<Option<TableCommission>, StorageError> {
        cert_commission_ops::find_commission_by_transaction(&self.conn, transaction_id)
    }

    /// Insert sync state
    pub fn insert_sync_state(&self, sync_state: &TableSyncState) -> Result<i64, StorageError> {
        cert_commission_ops::insert_sync_state(&self.conn, sync_state)
    }

    /// Find sync state by ref
    pub fn find_sync_state_by_ref(&self, ref_num: &str) -> Result<Option<TableSyncState>, StorageError> {
        cert_commission_ops::find_sync_state_by_ref(&self.conn, ref_num)
    }

    /// Insert monitor event
    pub fn insert_monitor_event(&self, event: &TableMonitorEvent) -> Result<i64, StorageError> {
        cert_commission_ops::insert_monitor_event(&self.conn, event)
    }

    /// Find or insert user (upsert operation)
    pub fn find_or_insert_user_internal(&self, identity_key: &str) -> Result<FindOrInsertUserResult, StorageError> {
        // Try to find existing user
        if let Some(user) = self.find_user_by_identity(identity_key)? {
            return Ok(FindOrInsertUserResult {
                user,
                is_new: false,
            });
        }

        // Get default active storage from settings
        let active_storage = self.settings.as_ref()
            .ok_or_else(|| StorageError::Database("Settings not loaded".to_string()))?
            .storage_identity_key.clone();

        // Insert new user
        let user_id = self.insert_user(identity_key, &active_storage)?;
        
        // Fetch the created user
        let user = self.find_user_by_id(user_id)?
            .ok_or_else(|| StorageError::Database("Failed to find newly created user".to_string()))?;

        Ok(FindOrInsertUserResult {
            user,
            is_new: true,
        })
    }
}

#[async_trait]
impl WalletStorageReader for StorageSqlite {
    fn is_available(&self) -> bool {
        self.settings.is_some()
    }

    fn get_settings(&self) -> &TableSettings {
        self.settings.as_ref().expect("Settings not loaded")
    }

    async fn find_certificates_auth(
        &self,
        _auth: &AuthId,
        _args: &FindCertificatesArgs,
    ) -> StorageResult<Vec<TableCertificate>> {
        Err(StorageError::NotImplemented("find_certificates_auth"))
    }

    async fn find_output_baskets_auth(
        &self,
        _auth: &AuthId,
        _args: &FindOutputBasketsArgs,
    ) -> StorageResult<Vec<TableOutputBasket>> {
        Err(StorageError::NotImplemented("find_output_baskets_auth"))
    }

    async fn find_outputs_auth(
        &self,
        _auth: &AuthId,
        _args: &FindOutputsArgs,
    ) -> StorageResult<Vec<TableOutput>> {
        Err(StorageError::NotImplemented("find_outputs_auth"))
    }

    async fn find_proven_tx_reqs(
        &self,
        _args: &FindProvenTxReqsArgs,
    ) -> StorageResult<Vec<TableProvenTxReq>> {
        Err(StorageError::NotImplemented("find_proven_tx_reqs"))
    }
}

#[async_trait]
impl WalletStorageWriter for StorageSqlite {
    async fn make_available(&mut self) -> StorageResult<TableSettings> {
        if !self.is_available() {
            return Err(StorageError::Database("Storage not initialized".to_string()));
        }
        Ok(self.get_settings().clone())
    }

    async fn migrate(
        &mut self,
        _storage_name: &str,
        _storage_identity_key: &str,
    ) -> StorageResult<String> {
        Err(StorageError::NotImplemented("migrate"))
    }

    async fn destroy(&mut self) -> StorageResult<()> {
        Err(StorageError::NotImplemented("destroy"))
    }

    async fn find_or_insert_user(
        &mut self,
        identity_key: &str,
    ) -> StorageResult<FindOrInsertUserResult> {
        self.find_or_insert_user_internal(identity_key)
    }

    async fn insert_certificate_auth(
        &mut self,
        _auth: &AuthId,
        _certificate: &TableCertificate,
    ) -> StorageResult<i64> {
        Err(StorageError::NotImplemented("insert_certificate_auth"))
    }
}

#[async_trait]
impl WalletStorageSync for StorageSqlite {
    async fn find_or_insert_sync_state_auth(
        &mut self,
        _auth: &AuthId,
        _storage_identity_key: &str,
        _storage_name: &str,
    ) -> StorageResult<FindOrInsertSyncStateResult> {
        Err(StorageError::NotImplemented("find_or_insert_sync_state_auth"))
    }

    async fn set_active(
        &mut self,
        _auth: &AuthId,
        _new_active_storage_identity_key: &str,
    ) -> StorageResult<i64> {
        Err(StorageError::NotImplemented("set_active"))
    }
}

impl WalletStorageProvider for StorageSqlite {}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_storage() -> StorageSqlite {
        let mut storage = StorageSqlite::new_in_memory().unwrap();
        storage
            .initialize("test_storage_key", "Test Storage", "main", 100000)
            .unwrap();
        storage
    }

    #[test]
    fn test_create_storage() {
        let storage = create_test_storage();
        assert!(storage.is_available());
    }

    #[test]
    fn test_get_settings() {
        use wallet_storage::schema::tables::table_settings::{Chain, DbType};
        
        let storage = create_test_storage();
        let settings = storage.get_settings();
        
        assert_eq!(settings.storage_identity_key, "test_storage_key");
        assert_eq!(settings.storage_name, "Test Storage");
        assert_eq!(settings.chain, Chain::Main);
        assert_eq!(settings.dbtype, DbType::SQLite);
        assert_eq!(settings.max_output_script, 100000);
    }

    #[test]
    fn test_insert_and_find_user() {
        let storage = create_test_storage();
        
        // Insert user
        let user_id = storage
            .insert_user("identity_key_123", "storage_key_abc")
            .unwrap();
        
        assert!(user_id > 0);

        // Find by identity
        let user = storage.find_user_by_identity("identity_key_123").unwrap();
        assert!(user.is_some());
        
        let user = user.unwrap();
        assert_eq!(user.user_id, user_id);
        assert_eq!(user.identity_key, "identity_key_123");
        assert_eq!(user.active_storage, "storage_key_abc");

        // Find by ID
        let user2 = storage.find_user_by_id(user_id).unwrap();
        assert!(user2.is_some());
        assert_eq!(user2.unwrap().identity_key, "identity_key_123");
    }

    #[test]
    fn test_find_or_insert_user_creates_new() {
        let storage = create_test_storage();
        
        let result = storage.find_or_insert_user_internal("new_user_key").unwrap();
        
        assert_eq!(result.user.identity_key, "new_user_key");
        assert!(result.is_new);
        assert!(result.user.user_id > 0);
    }

    #[test]
    fn test_find_or_insert_user_finds_existing() {
        let storage = create_test_storage();
        
        // Create user first
        let first_result = storage.find_or_insert_user_internal("existing_user").unwrap();
        assert!(first_result.is_new);
        
        // Try to create again
        let second_result = storage.find_or_insert_user_internal("existing_user").unwrap();
        assert!(!second_result.is_new);
        assert_eq!(first_result.user.user_id, second_result.user.user_id);
    }

    #[test]
    fn test_update_user() {
        let storage = create_test_storage();
        
        // Create user
        let user_id = storage.insert_user("user_key", "storage_1").unwrap();
        
        // Update user
        let mut user = storage.find_user_by_id(user_id).unwrap().unwrap();
        user.active_storage = "storage_2".to_string();
        
        storage.update_user(user_id, &user).unwrap();
        
        // Verify update
        let updated = storage.find_user_by_id(user_id).unwrap().unwrap();
        assert_eq!(updated.active_storage, "storage_2");
    }

    #[tokio::test]
    async fn test_async_trait_methods() {
        let mut storage = create_test_storage();
        
        // Test make_available
        let settings = storage.make_available().await.unwrap();
        assert_eq!(settings.storage_name, "Test Storage");

        // Test find_or_insert_user
        let result = storage.find_or_insert_user("async_user").await.unwrap();
        assert!(result.is_new);
    }
}
