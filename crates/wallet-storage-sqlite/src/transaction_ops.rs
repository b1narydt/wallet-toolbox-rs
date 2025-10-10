//! Transaction CRUD operations
//!
//! Implements database operations for the transactions table.
//! Reference: TypeScript StorageKnex transaction methods

use rusqlite::{Connection, params, OptionalExtension};
use std::sync::{Arc, Mutex};
use wallet_storage::*;

/// Insert a new transaction
pub fn insert_transaction(
    conn: &Arc<Mutex<Connection>>,
    user_id: i64,
    transaction: &TableTransaction,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO transactions (
            userId, provenTxId, status, reference, isOutgoing, satoshis,
            version, lockTime, description, txid, inputBEEF, rawTx
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            user_id,
            transaction.proven_tx_id,
            transaction.status.to_string(),
            transaction.reference,
            if transaction.is_outgoing { 1 } else { 0 },
            transaction.satoshis,
            transaction.version,
            transaction.lock_time,
            transaction.description,
            transaction.txid,
            transaction.input_beef.as_ref().map(|v| v.as_slice()),
            transaction.raw_tx.as_ref().map(|v| v.as_slice()),
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert transaction: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

/// Find transaction by ID
pub fn find_transaction_by_id(
    conn: &Arc<Mutex<Connection>>,
    transaction_id: i64,
) -> Result<Option<TableTransaction>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, transactionId, userId, provenTxId, status, reference,
                isOutgoing, satoshis, version, lockTime, description, txid, inputBEEF, rawTx
         FROM transactions WHERE transactionId = ?1",
        params![transaction_id],
        |row| {
            Ok(TableTransaction {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                transaction_id: row.get(2)?,
                user_id: row.get(3)?,
                proven_tx_id: row.get(4)?,
                status: row.get::<_, String>(5)?.parse().unwrap_or(TransactionStatus::Unprocessed),
                reference: row.get(6)?,
                is_outgoing: row.get::<_, i32>(7)? != 0,
                satoshis: row.get(8)?,
                version: row.get(9)?,
                lock_time: row.get(10)?,
                description: row.get(11)?,
                txid: row.get(12)?,
                input_beef: row.get::<_, Option<Vec<u8>>>(13)?,
                raw_tx: row.get::<_, Option<Vec<u8>>>(14)?,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find transaction: {}", e)))?;

    Ok(result)
}

/// Find transaction by reference
pub fn find_transaction_by_reference(
    conn: &Arc<Mutex<Connection>>,
    reference: &str,
) -> Result<Option<TableTransaction>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, transactionId, userId, provenTxId, status, reference,
                isOutgoing, satoshis, version, lockTime, description, txid, inputBEEF, rawTx
         FROM transactions WHERE reference = ?1",
        params![reference],
        |row| {
            Ok(TableTransaction {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                transaction_id: row.get(2)?,
                user_id: row.get(3)?,
                proven_tx_id: row.get(4)?,
                status: row.get::<_, String>(5)?.parse().unwrap_or(TransactionStatus::Unprocessed),
                reference: row.get(6)?,
                is_outgoing: row.get::<_, i32>(7)? != 0,
                satoshis: row.get(8)?,
                version: row.get(9)?,
                lock_time: row.get(10)?,
                description: row.get(11)?,
                txid: row.get(12)?,
                input_beef: row.get::<_, Option<Vec<u8>>>(13)?,
                raw_tx: row.get::<_, Option<Vec<u8>>>(14)?,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find transaction by reference: {}", e)))?;

    Ok(result)
}

/// Update transaction
pub fn update_transaction(
    conn: &Arc<Mutex<Connection>>,
    transaction_id: i64,
    transaction: &TableTransaction,
) -> Result<(), StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "UPDATE transactions 
         SET updated_at = datetime('now'),
             provenTxId = ?1,
             status = ?2,
             isOutgoing = ?3,
             satoshis = ?4,
             version = ?5,
             lockTime = ?6,
             description = ?7,
             txid = ?8,
             inputBEEF = ?9,
             rawTx = ?10
         WHERE transactionId = ?11",
        params![
            transaction.proven_tx_id,
            transaction.status.to_string(),
            if transaction.is_outgoing { 1 } else { 0 },
            transaction.satoshis,
            transaction.version,
            transaction.lock_time,
            transaction.description,
            transaction.txid,
            transaction.input_beef.as_ref().map(|v| v.as_slice()),
            transaction.raw_tx.as_ref().map(|v| v.as_slice()),
            transaction_id,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to update transaction: {}", e)))?;

    Ok(())
}

/// Find transactions for user with optional filters
pub fn find_transactions_for_user(
    conn: &Arc<Mutex<Connection>>,
    user_id: i64,
    status_filter: Option<&TransactionStatus>,
    limit: Option<u32>,
) -> Result<Vec<TableTransaction>, StorageError> {
    let conn = conn.lock().unwrap();

    let mut query = String::from(
        "SELECT created_at, updated_at, transactionId, userId, provenTxId, status, reference,
                isOutgoing, satoshis, version, lockTime, description, txid, inputBEEF, rawTx
         FROM transactions WHERE userId = ?1"
    );

    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(user_id)];

    if let Some(status) = status_filter {
        query.push_str(" AND status = ?2");
        params.push(Box::new(status.to_string()));
    }

    query.push_str(" ORDER BY created_at DESC");

    if let Some(lim) = limit {
        query.push_str(&format!(" LIMIT {}", lim));
    }

    let mut stmt = conn.prepare(&query)
        .map_err(|e| StorageError::Database(format!("Failed to prepare query: {}", e)))?;

    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(TableTransaction {
            created_at: row.get(0)?,
            updated_at: row.get(1)?,
            transaction_id: row.get(2)?,
            user_id: row.get(3)?,
            proven_tx_id: row.get(4)?,
            status: row.get::<_, String>(5)?.parse().unwrap_or(TransactionStatus::Unprocessed),
            reference: row.get(6)?,
            is_outgoing: row.get::<_, i32>(7)? != 0,
            satoshis: row.get(8)?,
            version: row.get(9)?,
            lock_time: row.get(10)?,
            description: row.get(11)?,
            txid: row.get(12)?,
            input_beef: row.get::<_, Option<Vec<u8>>>(13)?,
            raw_tx: row.get::<_, Option<Vec<u8>>>(14)?,
        })
    })
    .map_err(|e| StorageError::Database(format!("Failed to query transactions: {}", e)))?;

    let mut transactions = Vec::new();
    for row in rows {
        transactions.push(row.map_err(|e| StorageError::Database(format!("Row error: {}", e)))?);
    }

    Ok(transactions)
}

/// Delete transaction (for testing)
#[cfg(test)]
pub fn delete_transaction(
    conn: &Arc<Mutex<Connection>>,
    transaction_id: i64,
) -> Result<(), StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "DELETE FROM transactions WHERE transactionId = ?1",
        params![transaction_id],
    )
    .map_err(|e| StorageError::Database(format!("Failed to delete transaction: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use crate::migrations::apply_initial_migration;

    fn create_test_storage() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        
        apply_initial_migration(&conn, "test_key", "Test", "main", 100000).unwrap();
        
        // Insert test user
        conn.execute(
            "INSERT INTO users (identityKey, activeStorage) VALUES (?1, ?2)",
            params!["test_user_key", "test_storage"],
        ).unwrap();

        Arc::new(Mutex::new(conn))
    }

    #[test]
    fn test_insert_and_find_transaction() {
        let conn = create_test_storage();
        
        let transaction = TableTransaction::new(
            0, // transaction_id (will be set by DB)
            1, // user_id
            TransactionStatus::Unprocessed,
            "ref_12345",
            true, // isOutgoing
            50000,
            "Test transaction",
        );

        let tx_id = insert_transaction(&conn, 1, &transaction).unwrap();
        assert!(tx_id > 0);

        let found = find_transaction_by_id(&conn, tx_id).unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.transaction_id, tx_id);
        assert_eq!(found.reference, "ref_12345");
        assert_eq!(found.satoshis, 50000);
        assert!(found.is_outgoing);
    }

    #[test]
    fn test_find_transaction_by_reference() {
        let conn = create_test_storage();
        
        let transaction = TableTransaction::new(
            0, 1, TransactionStatus::Unprocessed, "unique_ref_abc", false, 25000, "Test"
        );
        insert_transaction(&conn, 1, &transaction).unwrap();

        let found = find_transaction_by_reference(&conn, "unique_ref_abc").unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.reference, "unique_ref_abc");
        assert_eq!(found.satoshis, 25000);
        assert!(!found.is_outgoing);
    }

    #[test]
    fn test_update_transaction() {
        let conn = create_test_storage();
        
        let mut transaction = TableTransaction::new(
            0, 1, TransactionStatus::Unprocessed, "ref_update", true, 10000, "Original"
        );
        let tx_id = insert_transaction(&conn, 1, &transaction).unwrap();

        // Update transaction
        transaction.transaction_id = tx_id;
        transaction.satoshis = 20000;
        transaction.description = "Updated".to_string();
        transaction.status = TransactionStatus::Completed;
        
        update_transaction(&conn, tx_id, &transaction).unwrap();

        // Verify update
        let found = find_transaction_by_id(&conn, tx_id).unwrap().unwrap();
        assert_eq!(found.satoshis, 20000);
        assert_eq!(found.description, "Updated");
        assert_eq!(found.status, TransactionStatus::Completed);
    }

    #[test]
    fn test_find_transactions_for_user() {
        let conn = create_test_storage();
        
        // Insert multiple transactions
        for i in 0..5 {
            let tx = TableTransaction::new(
                0,
                1,
                TransactionStatus::Unprocessed,
                &format!("ref_{}", i),
                i % 2 == 0,
                1000 * i,
                &format!("Transaction {}", i),
            );
            insert_transaction(&conn, 1, &tx).unwrap();
        }

        // Find all for user
        let all = find_transactions_for_user(&conn, 1, None, None).unwrap();
        assert_eq!(all.len(), 5);

        // Find with limit
        let limited = find_transactions_for_user(&conn, 1, None, Some(3)).unwrap();
        assert_eq!(limited.len(), 3);
    }

    #[test]
    fn test_find_transactions_with_status_filter() {
        let conn = create_test_storage();
        
        // Insert transactions with different statuses
        let mut tx1 = TableTransaction::new(
            0, 1, TransactionStatus::Completed, "ref_completed", true, 1000, "Completed"
        );
        insert_transaction(&conn, 1, &tx1).unwrap();

        let mut tx2 = TableTransaction::new(
            0, 1, TransactionStatus::Failed, "ref_failed", false, 2000, "Failed"
        );
        insert_transaction(&conn, 1, &tx2).unwrap();

        // Verify transactions can be found
        let tx1_found = find_transaction_by_reference(&conn, "ref_completed").unwrap().unwrap();
        let tx2_found = find_transaction_by_reference(&conn, "ref_failed").unwrap().unwrap();

        // Find completed only
        let completed = find_transactions_for_user(
            &conn,
            1,
            Some(&TransactionStatus::Completed),
            None
        ).unwrap();
        
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].status, TransactionStatus::Completed);
    }

    #[test]
    fn test_transaction_with_binary_data() {
        let conn = create_test_storage();
        
        let mut transaction = TableTransaction::new(
            0, 1, TransactionStatus::Unprocessed, "ref_binary", true, 5000, "Binary test"
        );
        transaction.raw_tx = Some(vec![0x01, 0x02, 0x03, 0x04]);
        transaction.input_beef = Some(vec![0xAA, 0xBB, 0xCC]);

        let tx_id = insert_transaction(&conn, 1, &transaction).unwrap();
        
        let found = find_transaction_by_id(&conn, tx_id).unwrap().unwrap();
        assert_eq!(found.raw_tx, Some(vec![0x01, 0x02, 0x03, 0x04]));
        assert_eq!(found.input_beef, Some(vec![0xAA, 0xBB, 0xCC]));
    }
}
