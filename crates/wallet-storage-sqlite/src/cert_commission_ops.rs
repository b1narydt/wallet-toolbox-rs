//! Certificate, Commission, SyncState, MonitorEvent CRUD operations
//!
//! Reference: @wallet-toolbox/src/storage/StorageKnex.ts

use rusqlite::{Connection, params, OptionalExtension};
use std::sync::{Arc, Mutex};
use wallet_storage::*;

// ============ CERTIFICATE ============

pub fn insert_certificate(
    conn: &Arc<Mutex<Connection>>,
    cert: &TableCertificate,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO certificates (
            userId, serialNumber, type, certifier, subject, verifier, revocationOutpoint, signature, isDeleted
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            cert.user_id,
            cert.serial_number,
            cert.certificate_type,
            cert.certifier,
            cert.subject,
            cert.verifier,
            cert.revocation_outpoint,
            cert.signature,
            if cert.is_deleted { 1 } else { 0 },
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert certificate: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

pub fn find_certificate_by_id(
    conn: &Arc<Mutex<Connection>>,
    cert_id: i64,
) -> Result<Option<TableCertificate>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, certificateId, userId, serialNumber, type, certifier,
                subject, verifier, revocationOutpoint, signature, isDeleted
         FROM certificates WHERE certificateId = ?1",
        params![cert_id],
        |row| {
            Ok(TableCertificate {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                certificate_id: row.get(2)?,
                user_id: row.get(3)?,
                serial_number: row.get(4)?,     // serialNumber column
                certificate_type: row.get(5)?,   // type column
                certifier: row.get(6)?,
                subject: row.get(7)?,
                verifier: row.get(8)?,
                revocation_outpoint: row.get(9)?,
                signature: row.get(10)?,
                is_deleted: row.get::<_, i32>(11)? != 0,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find certificate: {}", e)))?;

    Ok(result)
}

pub fn update_certificate(
    conn: &Arc<Mutex<Connection>>,
    cert_id: i64,
    cert: &TableCertificate,
) -> Result<usize, StorageError> {
    let conn = conn.lock().unwrap();

    let rows = conn.execute(
        "UPDATE certificates
         SET updated_at = datetime('now'),
             verifier = ?1,
             isDeleted = ?2
         WHERE certificateId = ?3",
        params![
            cert.verifier,
            if cert.is_deleted { 1 } else { 0 },
            cert_id,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to update certificate: {}", e)))?;

    Ok(rows)
}

// ============ CERTIFICATE FIELD ============

pub fn insert_certificate_field(
    conn: &Arc<Mutex<Connection>>,
    field: &TableCertificateField,
) -> Result<(), StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO certificate_fields (userId, certificateId, fieldName, fieldValue, masterKey)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            field.user_id,
            field.certificate_id,
            field.field_name,
            field.field_value,
            field.master_key,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert certificate_field: {}", e)))?;

    Ok(())
}

pub fn find_certificate_fields(
    conn: &Arc<Mutex<Connection>>,
    cert_id: i64,
) -> Result<Vec<TableCertificateField>, StorageError> {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare(
        "SELECT created_at, updated_at, userId, certificateId, fieldName, fieldValue, masterKey
         FROM certificate_fields WHERE certificateId = ?1"
    )
    .map_err(|e| StorageError::Database(format!("Failed to prepare query: {}", e)))?;

    let rows = stmt.query_map(params![cert_id], |row| {
        Ok(TableCertificateField {
            created_at: row.get(0)?,
            updated_at: row.get(1)?,
            user_id: row.get(2)?,
            certificate_id: row.get(3)?,
            field_name: row.get(4)?,
            field_value: row.get(5)?,
            master_key: row.get(6)?,
        })
    })
    .map_err(|e| StorageError::Database(format!("Failed to query certificate_fields: {}", e)))?;

    let mut fields = Vec::new();
    for row in rows {
        fields.push(row.map_err(|e| StorageError::Database(format!("Row error: {}", e)))?);
    }

    Ok(fields)
}

// ============ COMMISSION ============

pub fn insert_commission(
    conn: &Arc<Mutex<Connection>>,
    commission: &TableCommission,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO commissions (userId, transactionId, satoshis, keyOffset, isRedeemed, lockingScript)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            commission.user_id,
            commission.transaction_id,
            commission.satoshis,
            commission.key_offset,
            if commission.is_redeemed { 1 } else { 0 },
            &commission.locking_script,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert commission: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

pub fn find_commission_by_transaction(
    conn: &Arc<Mutex<Connection>>,
    transaction_id: i64,
) -> Result<Option<TableCommission>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, commissionId, userId, transactionId, satoshis, keyOffset, isRedeemed, lockingScript
         FROM commissions WHERE transactionId = ?1",
        params![transaction_id],
        |row| {
            Ok(TableCommission {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                commission_id: row.get(2)?,
                user_id: row.get(3)?,
                transaction_id: row.get(4)?,
                satoshis: row.get(5)?,
                key_offset: row.get(6)?,
                is_redeemed: row.get::<_, i32>(7)? != 0,
                locking_script: row.get(8)?,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find commission: {}", e)))?;

    Ok(result)
}

// ============ SYNC STATE ============

pub fn insert_sync_state(
    conn: &Arc<Mutex<Connection>>,
    sync_state: &TableSyncState,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO sync_states (
            userId, storageIdentityKey, storageName, status, init, refNum, syncMap, `when`, satoshis, errorLocal, errorOther
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            sync_state.user_id,
            sync_state.storage_identity_key,
            sync_state.storage_name,
            sync_state.status.to_string(),
            if sync_state.init { 1 } else { 0 },
            sync_state.ref_num,
            sync_state.sync_map,
            sync_state.when,
            sync_state.satoshis,
            sync_state.error_local,
            sync_state.error_other,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert sync_state: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

pub fn find_sync_state_by_ref(
    conn: &Arc<Mutex<Connection>>,
    ref_num: &str,
) -> Result<Option<TableSyncState>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, syncStateId, userId, storageIdentityKey, storageName,
                status, init, refNum, syncMap, `when`, satoshis, errorLocal, errorOther
         FROM sync_states WHERE refNum = ?1",
        params![ref_num],
        |row| {
            Ok(TableSyncState {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                sync_state_id: row.get(2)?,
                user_id: row.get(3)?,
                storage_identity_key: row.get(4)?,
                storage_name: row.get(5)?,
                status: row.get::<_, String>(6)?.parse().unwrap_or(SyncStatus::Unknown),
                init: row.get::<_, i32>(7)? != 0,
                ref_num: row.get(8)?,
                sync_map: row.get(9)?,
                when: row.get(10)?,
                satoshis: row.get(11)?,
                error_local: row.get(12)?,
                error_other: row.get(13)?,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find sync_state: {}", e)))?;

    Ok(result)
}

// ============ MONITOR EVENT ============

pub fn insert_monitor_event(
    conn: &Arc<Mutex<Connection>>,
    event: &TableMonitorEvent,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO monitor_events (event, details) VALUES (?1, ?2)",
        params![event.event, event.details],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert monitor_event: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrations::apply_initial_migration;

    fn create_test_storage() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        apply_initial_migration(&conn, "test_key", "Test", "main", 100000).unwrap();
        
        conn.execute(
            "INSERT INTO users (identityKey, activeStorage) VALUES (?1, ?2)",
            params!["test_user", "test_storage"],
        ).unwrap();

        Arc::new(Mutex::new(conn))
    }

    #[test]
    fn test_certificate_crud() {
        let conn = create_test_storage();
        
        let cert = TableCertificate::new(
            0, 1, "identity", "serial_123", "certifier_key", "subject_key", "outpoint_abc", "signature_xyz",
        );

        let id = insert_certificate(&conn, &cert).unwrap();
        assert!(id > 0);

        let found = find_certificate_by_id(&conn, id).unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.serial_number, "serial_123");
        assert_eq!(found.certificate_type, "identity");
    }

    #[test]
    fn test_certificate_fields() {
        let conn = create_test_storage();
        
        let cert = TableCertificate::new(
            0, 1, "attestation", "serial_456", "cert_key", "subj_key", "outpoint", "sig",
        );
        let cert_id = insert_certificate(&conn, &cert).unwrap();

        let field = TableCertificateField::new(
            1, cert_id, "email", "user@example.com", "master_key_123",
        );
        insert_certificate_field(&conn, &field).unwrap();

        let fields = find_certificate_fields(&conn, cert_id).unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].field_name, "email");
        assert_eq!(fields[0].field_value, "user@example.com");
    }

    #[test]
    fn test_commission_crud() {
        let conn = create_test_storage();
        
        // Need transaction first
        conn.lock().unwrap().execute(
            "INSERT INTO transactions (userId, status, reference, isOutgoing, satoshis, description)
             VALUES (1, 'completed', 'ref_comm', 1, 10000, 'Test')",
            params![],
        ).unwrap();

        let commission = TableCommission::new(
            0, 1, 1, 500, "key_offset_123", vec![0x76, 0xA9],
        );

        let id = insert_commission(&conn, &commission).unwrap();
        assert!(id > 0);

        let found = find_commission_by_transaction(&conn, 1).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().satoshis, 500);
    }

    #[test]
    fn test_sync_state_crud() {
        let conn = create_test_storage();
        
        let sync_state = TableSyncState::new(
            0, 1, "storage_key", "Storage Name", SyncStatus::Success, false, "ref_unique_123", "{}",
        );

        let id = insert_sync_state(&conn, &sync_state).unwrap();
        assert!(id > 0);

        let found = find_sync_state_by_ref(&conn, "ref_unique_123").unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.status, SyncStatus::Success);
        assert_eq!(found.storage_name, "Storage Name");
    }

    #[test]
    fn test_monitor_event() {
        let conn = create_test_storage();
        
        let event = TableMonitorEvent::new(0, "app_start");
        // Note: details field can be set separately if needed

        let id = insert_monitor_event(&conn, &event).unwrap();
        assert!(id > 0);
    }
}
