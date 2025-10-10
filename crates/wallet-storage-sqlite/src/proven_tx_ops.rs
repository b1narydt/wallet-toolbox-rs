//! ProvenTx and ProvenTxReq CRUD operations
//!
//! Reference: @wallet-toolbox/src/storage/StorageKnex.ts insertProvenTx, insertProvenTxReq

use rusqlite::{Connection, params, OptionalExtension};
use std::sync::{Arc, Mutex};
use wallet_storage::*;

/// Insert proven transaction
pub fn insert_proven_tx(
    conn: &Arc<Mutex<Connection>>,
    proven_tx: &TableProvenTx,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO proven_txs (txid, height, `index`, merklePath, rawTx, blockHash, merkleRoot)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            proven_tx.txid,
            proven_tx.height,
            proven_tx.index,
            &proven_tx.merkle_path,
            &proven_tx.raw_tx,
            proven_tx.block_hash,
            proven_tx.merkle_root,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert proven_tx: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

/// Find proven tx by txid
pub fn find_proven_tx_by_txid(
    conn: &Arc<Mutex<Connection>>,
    txid: &str,
) -> Result<Option<TableProvenTx>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, provenTxId, txid, height, `index`, merklePath, rawTx, blockHash, merkleRoot
         FROM proven_txs WHERE txid = ?1",
        params![txid],
        |row| {
            Ok(TableProvenTx {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                proven_tx_id: row.get(2)?,
                txid: row.get(3)?,
                height: row.get(4)?,
                index: row.get(5)?,
                merkle_path: row.get(6)?,
                raw_tx: row.get(7)?,
                block_hash: row.get(8)?,
                merkle_root: row.get(9)?,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find proven_tx: {}", e)))?;

    Ok(result)
}

/// Insert proven transaction request
pub fn insert_proven_tx_req(
    conn: &Arc<Mutex<Connection>>,
    req: &TableProvenTxReq,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO proven_tx_reqs (
            provenTxId, status, attempts, notified, txid, batch, history, notify, rawTx, inputBEEF
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            req.proven_tx_id,
            req.status.to_string(),
            req.attempts,
            if req.notified { 1 } else { 0 },
            req.txid,
            req.batch,
            req.history,
            req.notify,
            &req.raw_tx,
            req.input_beef.as_ref().map(|v| v.as_slice()),
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert proven_tx_req: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

/// Update proven transaction request
pub fn update_proven_tx_req(
    conn: &Arc<Mutex<Connection>>,
    req_id: i64,
    req: &TableProvenTxReq,
) -> Result<usize, StorageError> {
    let conn = conn.lock().unwrap();

    let rows = conn.execute(
        "UPDATE proven_tx_reqs
         SET updated_at = datetime('now'),
             provenTxId = ?1,
             status = ?2,
             attempts = ?3,
             notified = ?4,
             batch = ?5,
             history = ?6,
             notify = ?7
         WHERE provenTxReqId = ?8",
        params![
            req.proven_tx_id,
            req.status.to_string(),
            req.attempts,
            if req.notified { 1 } else { 0 },
            req.batch,
            req.history,
            req.notify,
            req_id,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to update proven_tx_req: {}", e)))?;

    Ok(rows)
}

/// Find proven tx req by txid
pub fn find_proven_tx_req_by_txid(
    conn: &Arc<Mutex<Connection>>,
    txid: &str,
) -> Result<Option<TableProvenTxReq>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, provenTxReqId, provenTxId, status, attempts, notified,
                txid, batch, history, notify, rawTx, inputBEEF
         FROM proven_tx_reqs WHERE txid = ?1",
        params![txid],
        |row| {
            Ok(TableProvenTxReq {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                proven_tx_req_id: row.get(2)?,
                proven_tx_id: row.get(3)?,
                status: row.get::<_, String>(4)?.parse().unwrap_or(ProvenTxReqStatus::Unknown),
                attempts: row.get(5)?,
                notified: row.get::<_, i32>(6)? != 0,
                txid: row.get(7)?,
                batch: row.get(8)?,
                history: row.get(9)?,
                notify: row.get(10)?,
                raw_tx: row.get(11)?,
                input_beef: row.get(12)?,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find proven_tx_req: {}", e)))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrations::apply_initial_migration;

    fn create_test_storage() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        apply_initial_migration(&conn, "test_key", "Test", "main", 100000).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[test]
    fn test_insert_and_find_proven_tx() {
        let conn = create_test_storage();
        
        let proven_tx = TableProvenTx::new(
            0,
            "abc123def456",
            850000,
            1,
            vec![0x01, 0x02, 0x03],
            vec![0xAA, 0xBB, 0xCC],
            "block_hash_123",
            "merkle_root_456",
        );

        let id = insert_proven_tx(&conn, &proven_tx).unwrap();
        assert!(id > 0);

        let found = find_proven_tx_by_txid(&conn, "abc123def456").unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.txid, "abc123def456");
        assert_eq!(found.height, 850000);
        assert_eq!(found.index, 1);
    }

    #[test]
    fn test_insert_proven_tx_req() {
        let conn = create_test_storage();
        
        let mut req = TableProvenTxReq::new(
            0,
            ProvenTxReqStatus::Unsent,
            "txid_unproven",
            "{}",
            "{}",
            vec![0x01, 0x02],
        );
        req.batch = Some("batch_1".to_string());

        let id = insert_proven_tx_req(&conn, &req).unwrap();
        assert!(id > 0);

        let found = find_proven_tx_req_by_txid(&conn, "txid_unproven").unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.status, ProvenTxReqStatus::Unsent);
        assert_eq!(found.batch, Some("batch_1".to_string()));
    }
}
