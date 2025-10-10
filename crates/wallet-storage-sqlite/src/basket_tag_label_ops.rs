//! OutputBasket, OutputTag, TxLabel CRUD operations
//!
//! Reference: @wallet-toolbox/src/storage/StorageKnex.ts

use rusqlite::{Connection, params, OptionalExtension};
use std::sync::{Arc, Mutex};
use wallet_storage::*;

// ============ OUTPUT BASKET ============

pub fn insert_output_basket(
    conn: &Arc<Mutex<Connection>>,
    basket: &TableOutputBasket,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO output_baskets (userId, name, numberOfDesiredUTXOs, minimumDesiredUTXOValue, isDeleted)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            basket.user_id,
            basket.name,
            basket.number_of_desired_utxos,
            basket.minimum_desired_utxo_value,
            if basket.is_deleted { 1 } else { 0 },
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert output_basket: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

pub fn find_output_basket_by_name(
    conn: &Arc<Mutex<Connection>>,
    user_id: i64,
    name: &str,
) -> Result<Option<TableOutputBasket>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, basketId, userId, name, numberOfDesiredUTXOs, minimumDesiredUTXOValue, isDeleted
         FROM output_baskets WHERE userId = ?1 AND name = ?2",
        params![user_id, name],
        |row| {
            Ok(TableOutputBasket {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                basket_id: row.get(2)?,
                user_id: row.get(3)?,
                name: row.get(4)?,
                number_of_desired_utxos: row.get(5)?,
                minimum_desired_utxo_value: row.get(6)?,
                is_deleted: row.get::<_, i32>(7)? != 0,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find output_basket: {}", e)))?;

    Ok(result)
}

pub fn update_output_basket(
    conn: &Arc<Mutex<Connection>>,
    basket_id: i64,
    basket: &TableOutputBasket,
) -> Result<usize, StorageError> {
    let conn = conn.lock().unwrap();

    let rows = conn.execute(
        "UPDATE output_baskets
         SET updated_at = datetime('now'),
             numberOfDesiredUTXOs = ?1,
             minimumDesiredUTXOValue = ?2,
             isDeleted = ?3
         WHERE basketId = ?4",
        params![
            basket.number_of_desired_utxos,
            basket.minimum_desired_utxo_value,
            if basket.is_deleted { 1 } else { 0 },
            basket_id,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to update output_basket: {}", e)))?;

    Ok(rows)
}

// ============ OUTPUT TAG ============

pub fn insert_output_tag(
    conn: &Arc<Mutex<Connection>>,
    tag: &TableOutputTag,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO output_tags (userId, tag, isDeleted) VALUES (?1, ?2, ?3)",
        params![
            tag.user_id,
            tag.tag,
            if tag.is_deleted { 1 } else { 0 },
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert output_tag: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

pub fn find_output_tag_by_name(
    conn: &Arc<Mutex<Connection>>,
    user_id: i64,
    tag: &str,
) -> Result<Option<TableOutputTag>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, outputTagId, userId, tag, isDeleted
         FROM output_tags WHERE userId = ?1 AND tag = ?2",
        params![user_id, tag],
        |row| {
            Ok(TableOutputTag {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                output_tag_id: row.get(2)?,
                user_id: row.get(3)?,
                tag: row.get(4)?,
                is_deleted: row.get::<_, i32>(5)? != 0,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find output_tag: {}", e)))?;

    Ok(result)
}

// ============ OUTPUT TAG MAP ============

pub fn insert_output_tag_map(
    conn: &Arc<Mutex<Connection>>,
    map: &TableOutputTagMap,
) -> Result<(), StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO output_tags_map (outputTagId, outputId, isDeleted) VALUES (?1, ?2, ?3)",
        params![
            map.output_tag_id,
            map.output_id,
            if map.is_deleted { 1 } else { 0 },
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert output_tag_map: {}", e)))?;

    Ok(())
}

// ============ TX LABEL ============

pub fn insert_tx_label(
    conn: &Arc<Mutex<Connection>>,
    label: &TableTxLabel,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO tx_labels (userId, label, isDeleted) VALUES (?1, ?2, ?3)",
        params![
            label.user_id,
            label.label,
            if label.is_deleted { 1 } else { 0 },
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert tx_label: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

pub fn find_tx_label_by_name(
    conn: &Arc<Mutex<Connection>>,
    user_id: i64,
    label: &str,
) -> Result<Option<TableTxLabel>, StorageError> {
    let conn = conn.lock().unwrap();

    let result = conn.query_row(
        "SELECT created_at, updated_at, txLabelId, userId, label, isDeleted
         FROM tx_labels WHERE userId = ?1 AND label = ?2",
        params![user_id, label],
        |row| {
            Ok(TableTxLabel {
                created_at: row.get(0)?,
                updated_at: row.get(1)?,
                tx_label_id: row.get(2)?,
                user_id: row.get(3)?,
                label: row.get(4)?,
                is_deleted: row.get::<_, i32>(5)? != 0,
            })
        },
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find tx_label: {}", e)))?;

    Ok(result)
}

// ============ TX LABEL MAP ============

pub fn insert_tx_label_map(
    conn: &Arc<Mutex<Connection>>,
    map: &TableTxLabelMap,
) -> Result<(), StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO tx_labels_map (txLabelId, transactionId, isDeleted) VALUES (?1, ?2, ?3)",
        params![
            map.tx_label_id,
            map.transaction_id,
            if map.is_deleted { 1 } else { 0 },
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert tx_label_map: {}", e)))?;

    Ok(())
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
    fn test_output_basket_crud() {
        let conn = create_test_storage();
        
        let basket = TableOutputBasket::new(0, 1, "savings", 10, 50000);

        let id = insert_output_basket(&conn, &basket).unwrap();
        assert!(id > 0);

        let found = find_output_basket_by_name(&conn, 1, "savings").unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.name, "savings");
        assert_eq!(found.number_of_desired_utxos, 10);
    }

    #[test]
    fn test_output_tag_crud() {
        let conn = create_test_storage();
        
        let tag = TableOutputTag::new(0, 1, "important");

        let id = insert_output_tag(&conn, &tag).unwrap();
        assert!(id > 0);

        let found = find_output_tag_by_name(&conn, 1, "important").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().tag, "important");
    }

    #[test]
    fn test_tx_label_crud() {
        let conn = create_test_storage();
        
        let label = TableTxLabel::new(0, 1, "invoice-123");

        let id = insert_tx_label(&conn, &label).unwrap();
        assert!(id > 0);

        let found = find_tx_label_by_name(&conn, 1, "invoice-123").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().label, "invoice-123");
    }
}
