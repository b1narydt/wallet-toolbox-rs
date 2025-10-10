//! Output CRUD operations
//!
//! Implements database operations for the outputs table.
//! Reference: TypeScript StorageKnex output methods in @wallet-toolbox

use rusqlite::{Connection, params, OptionalExtension};
use std::sync::{Arc, Mutex};
use wallet_storage::*;

/// Insert a new output
/// 
/// Matches TypeScript `insertOutput(output: TableOutput, trx?: TrxToken): Promise<number>`
pub fn insert_output(
    conn: &Arc<Mutex<Connection>>,
    output: &TableOutput,
) -> Result<i64, StorageError> {
    let conn = conn.lock().unwrap();

    conn.execute(
        "INSERT INTO outputs (
            userId, transactionId, basketId, spendable, `change`, vout, satoshis,
            providedBy, purpose, type, outputDescription, txid, senderIdentityKey,
            derivationPrefix, derivationSuffix, customInstructions, spentBy,
            sequenceNumber, spendingDescription, scriptLength, scriptOffset, lockingScript
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)",
        params![
            output.user_id,
            output.transaction_id,
            output.basket_id,
            if output.spendable { 1 } else { 0 },
            if output.change { 1 } else { 0 },
            output.vout,
            output.satoshis,
            match output.provided_by {
                StorageProvidedBy::You => "you",
                StorageProvidedBy::Storage => "storage",
                StorageProvidedBy::YouAndStorage => "you-and-storage",
            },
            &output.purpose,
            &output.output_type,
            &output.output_description,
            output.txid.as_ref(),
            output.sender_identity_key.as_ref(),
            output.derivation_prefix.as_ref(),
            output.derivation_suffix.as_ref(),
            output.custom_instructions.as_ref(),
            output.spent_by,
            output.sequence_number,
            output.spending_description.as_ref(),
            output.script_length,
            output.script_offset,
            output.locking_script.as_ref().map(|v| v.as_slice()),
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to insert output: {}", e)))?;

    Ok(conn.last_insert_rowid())
}

/// Find output by ID with optional script exclusion
///
/// Matches TypeScript `findOutputById(id: number, trx?: TrxToken, noScript?: boolean)`
pub fn find_output_by_id(
    conn: &Arc<Mutex<Connection>>,
    output_id: i64,
    no_script: bool,
) -> Result<Option<TableOutput>, StorageError> {
    let conn = conn.lock().unwrap();

    let query = if no_script {
        // Exclude lockingScript for performance (matches outputColumnsWithoutLockingScript)
        "SELECT created_at, updated_at, outputId, userId, transactionId, basketId, spendable, `change`,
                vout, satoshis, providedBy, purpose, type, outputDescription, txid, senderIdentityKey,
                derivationPrefix, derivationSuffix, customInstructions, spentBy, sequenceNumber,
                spendingDescription, scriptLength, scriptOffset
         FROM outputs WHERE outputId = ?1"
    } else {
        "SELECT created_at, updated_at, outputId, userId, transactionId, basketId, spendable, `change`,
                vout, satoshis, providedBy, purpose, type, outputDescription, txid, senderIdentityKey,
                derivationPrefix, derivationSuffix, customInstructions, spentBy, sequenceNumber,
                spendingDescription, scriptLength, scriptOffset, lockingScript
         FROM outputs WHERE outputId = ?1"
    };

    let result = conn.query_row(
        query,
        params![output_id],
        |row| parse_output_row(row, no_script),
    )
    .optional()
    .map_err(|e| StorageError::Database(format!("Failed to find output: {}", e)))?;

    Ok(result)
}

/// Helper to parse output row from database
fn parse_output_row(row: &rusqlite::Row, no_script: bool) -> rusqlite::Result<TableOutput> {
    let provided_by_str: String = row.get(10)?;
    let provided_by = match provided_by_str.as_str() {
        "you" => StorageProvidedBy::You,
        "storage" => StorageProvidedBy::Storage,
        "you-and-storage" => StorageProvidedBy::YouAndStorage,
        _ => StorageProvidedBy::You,
    };

    let locking_script = if no_script {
        None
    } else {
        row.get::<_, Option<Vec<u8>>>(24)?
    };

    Ok(TableOutput {
        created_at: row.get(0)?,
        updated_at: row.get(1)?,
        output_id: row.get(2)?,
        user_id: row.get(3)?,
        transaction_id: row.get(4)?,
        basket_id: row.get(5)?,
        spendable: row.get::<_, i32>(6)? != 0,
        change: row.get::<_, i32>(7)? != 0,
        vout: row.get(8)?,
        satoshis: row.get(9)?,
        provided_by,
        purpose: row.get(11)?,
        output_type: row.get(12)?,
        output_description: row.get(13)?,
        txid: row.get(14)?,
        sender_identity_key: row.get(15)?,
        derivation_prefix: row.get(16)?,
        derivation_suffix: row.get(17)?,
        custom_instructions: row.get(18)?,
        spent_by: row.get(19)?,
        sequence_number: row.get(20)?,
        spending_description: row.get(21)?,
        script_length: row.get(22)?,
        script_offset: row.get(23)?,
        locking_script,
    })
}

/// Update output
///
/// Matches TypeScript `updateOutput(id: number, update: Partial<TableOutput>, trx?: TrxToken)`
/// Returns number of affected rows
pub fn update_output(
    conn: &Arc<Mutex<Connection>>,
    output_id: i64,
    output: &TableOutput,
) -> Result<usize, StorageError> {
    let conn = conn.lock().unwrap();

    let rows = conn.execute(
        "UPDATE outputs 
         SET updated_at = datetime('now'),
             basketId = ?1,
             spendable = ?2,
             `change` = ?3,
             outputDescription = ?4,
             txid = ?5,
             senderIdentityKey = ?6,
             derivationPrefix = ?7,
             derivationSuffix = ?8,
             customInstructions = ?9,
             spentBy = ?10,
             sequenceNumber = ?11,
             spendingDescription = ?12,
             scriptLength = ?13,
             scriptOffset = ?14,
             lockingScript = ?15
         WHERE outputId = ?16",
        params![
            output.basket_id,
            if output.spendable { 1 } else { 0 },
            if output.change { 1 } else { 0 },
            output.output_description,
            output.txid,
            output.sender_identity_key,
            output.derivation_prefix,
            output.derivation_suffix,
            output.custom_instructions,
            output.spent_by,
            output.sequence_number,
            output.spending_description,
            output.script_length,
            output.script_offset,
            output.locking_script.as_ref().map(|v| v.as_slice()),
            output_id,
        ],
    )
    .map_err(|e| StorageError::Database(format!("Failed to update output: {}", e)))?;

    Ok(rows)
}

/// Find outputs for transaction
pub fn find_outputs_for_transaction(
    conn: &Arc<Mutex<Connection>>,
    transaction_id: i64,
    no_script: bool,
) -> Result<Vec<TableOutput>, StorageError> {
    let conn = conn.lock().unwrap();

    let query = if no_script {
        "SELECT created_at, updated_at, outputId, userId, transactionId, basketId, spendable, `change`,
                vout, satoshis, providedBy, purpose, type, outputDescription, txid, senderIdentityKey,
                derivationPrefix, derivationSuffix, customInstructions, spentBy, sequenceNumber,
                spendingDescription, scriptLength, scriptOffset
         FROM outputs WHERE transactionId = ?1 ORDER BY vout ASC"
    } else {
        "SELECT created_at, updated_at, outputId, userId, transactionId, basketId, spendable, `change`,
                vout, satoshis, providedBy, purpose, type, outputDescription, txid, senderIdentityKey,
                derivationPrefix, derivationSuffix, customInstructions, spentBy, sequenceNumber,
                spendingDescription, scriptLength, scriptOffset, lockingScript
         FROM outputs WHERE transactionId = ?1 ORDER BY vout ASC"
    };

    let mut stmt = conn.prepare(query)
        .map_err(|e| StorageError::Database(format!("Failed to prepare query: {}", e)))?;

    let rows = stmt.query_map(params![transaction_id], |row| parse_output_row(row, no_script))
        .map_err(|e| StorageError::Database(format!("Failed to query outputs: {}", e)))?;

    let mut outputs = Vec::new();
    for row in rows {
        outputs.push(row.map_err(|e| StorageError::Database(format!("Row error: {}", e)))?);
    }

    Ok(outputs)
}

/// Find spendable outputs for user (useful for coin selection)
pub fn find_spendable_outputs_for_user(
    conn: &Arc<Mutex<Connection>>,
    user_id: i64,
    basket_id: Option<i64>,
    limit: Option<u32>,
) -> Result<Vec<TableOutput>, StorageError> {
    let conn = conn.lock().unwrap();

    let mut query = String::from(
        "SELECT created_at, updated_at, outputId, userId, transactionId, basketId, spendable, `change`,
                vout, satoshis, providedBy, purpose, type, outputDescription, txid, senderIdentityKey,
                derivationPrefix, derivationSuffix, customInstructions, spentBy, sequenceNumber,
                spendingDescription, scriptLength, scriptOffset
         FROM outputs 
         WHERE userId = ?1 AND spendable = 1 AND spentBy IS NULL"
    );

    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(user_id)];

    if let Some(basket) = basket_id {
        query.push_str(" AND basketId = ?2");
        params_vec.push(Box::new(basket));
    }

    query.push_str(" ORDER BY satoshis DESC");

    if let Some(lim) = limit {
        query.push_str(&format!(" LIMIT {}", lim));
    }

    let mut stmt = conn.prepare(&query)
        .map_err(|e| StorageError::Database(format!("Failed to prepare query: {}", e)))?;

    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(params_refs.as_slice(), |row| parse_output_row(row, true))
        .map_err(|e| StorageError::Database(format!("Failed to query outputs: {}", e)))?;

    let mut outputs = Vec::new();
    for row in rows {
        outputs.push(row.map_err(|e| StorageError::Database(format!("Row error: {}", e)))?);
    }

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use crate::migrations::apply_initial_migration;

    fn create_test_storage() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        apply_initial_migration(&conn, "test_key", "Test", "main", 100000).unwrap();
        
        // Insert test user
        conn.execute(
            "INSERT INTO users (identityKey, activeStorage) VALUES (?1, ?2)",
            params!["test_user", "test_storage"],
        ).unwrap();

        // Insert test transaction
        conn.execute(
            "INSERT INTO transactions (userId, status, reference, isOutgoing, satoshis, description)
             VALUES (1, 'completed', 'ref_test', 1, 10000, 'Test transaction')",
            params![],
        ).unwrap();

        Arc::new(Mutex::new(conn))
    }

    #[test]
    fn test_insert_and_find_output() {
        let conn = create_test_storage();
        
        let output = TableOutput::new(
            0, 1, 1, // output_id, user_id, transaction_id
            true, false, // spendable, change
            "Test output", // output_description
            0, 5000, // vout, satoshis
            StorageProvidedBy::You,
            "payment",
            "P2PKH",
        );

        let output_id = insert_output(&conn, &output).unwrap();
        assert!(output_id > 0);

        // Find without script
        let found = find_output_by_id(&conn, output_id, true).unwrap();
        assert!(found.is_some());
        
        let found = found.unwrap();
        assert_eq!(found.output_id, output_id);
        assert_eq!(found.satoshis, 5000);
        assert!(found.spendable);
        assert!(!found.change);
        assert_eq!(found.vout, 0);
        assert!(found.locking_script.is_none()); // noScript=true
    }

    #[test]
    fn test_output_with_locking_script() {
        let conn = create_test_storage();
        
        let mut output = TableOutput::new(
            0, 1, 1,
            true, false,
            "Output with script",
            0, 5000,
            StorageProvidedBy::You,
            "payment",
            "P2PKH",
        );
        output.locking_script = Some(vec![0x76, 0xA9, 0x14]); // OP_DUP OP_HASH160 OP_PUSH20

        let output_id = insert_output(&conn, &output).unwrap();

        // Find WITH script
        let found = find_output_by_id(&conn, output_id, false).unwrap().unwrap();
        assert_eq!(found.locking_script, Some(vec![0x76, 0xA9, 0x14]));

        // Find WITHOUT script
        let found_no_script = find_output_by_id(&conn, output_id, true).unwrap().unwrap();
        assert!(found_no_script.locking_script.is_none());
    }

    #[test]
    fn test_update_output() {
        let conn = create_test_storage();
        
        let mut output = TableOutput::new(
            0, 1, 1,
            true, false,
            "Original",
            0, 1000,
            StorageProvidedBy::You,
            "payment",
            "P2PKH",
        );

        let output_id = insert_output(&conn, &output).unwrap();

        // Update output
        output.output_id = output_id;
        output.spendable = false;
        output.txid = Some("abc123".to_string());
        // Don't set spent_by to avoid foreign key constraint

        let rows_affected = update_output(&conn, output_id, &output).unwrap();
        assert_eq!(rows_affected, 1);

        // Verify update
        let found = find_output_by_id(&conn, output_id, true).unwrap().unwrap();
        assert!(!found.spendable);
        assert_eq!(found.txid, Some("abc123".to_string()));
    }

    #[test]
    fn test_find_outputs_for_transaction() {
        let conn = create_test_storage();
        
        // Insert multiple outputs for same transaction
        for vout in 0..3 {
            let output = TableOutput::new(
                0, 1, 1,
                true, vout == 2, // last one is change
                &format!("Output {}", vout),
                vout, 1000 * (vout as i64 + 1),
                StorageProvidedBy::You,
                "payment",
                "P2PKH",
            );
            insert_output(&conn, &output).unwrap();
        }

        let outputs = find_outputs_for_transaction(&conn, 1, true).unwrap();
        assert_eq!(outputs.len(), 3);
        assert_eq!(outputs[0].vout, 0);
        assert_eq!(outputs[1].vout, 1);
        assert_eq!(outputs[2].vout, 2);
        assert!(outputs[2].change); // Last one is change
    }

    #[test]
    fn test_find_spendable_outputs() {
        let conn = create_test_storage();
        
        // Insert spendable output
        let output1 = TableOutput::new(
            0, 1, 1,
            true, false,
            "Spendable",
            0, 5000,
            StorageProvidedBy::You,
            "payment",
            "P2PKH",
        );
        insert_output(&conn, &output1).unwrap();

        // Insert already spent output (need to create transaction 2 first)
        conn.lock().unwrap().execute(
            "INSERT INTO transactions (userId, status, reference, isOutgoing, satoshis, description)
             VALUES (1, 'completed', 'ref_tx2', 1, 3000, 'Spending tx')",
            params![],
        ).unwrap();
        
        let mut output2 = TableOutput::new(
            0, 1, 1,
            true, false,
            "Spent",
            1, 3000,
            StorageProvidedBy::You,
            "payment",
            "P2PKH",
        );
        output2.spent_by = Some(2); // References the transaction we just created
        insert_output(&conn, &output2).unwrap();

        // Insert non-spendable output
        let output3 = TableOutput::new(
            0, 1, 1,
            false, false,
            "Not spendable",
            2, 2000,
            StorageProvidedBy::You,
            "payment",
            "P2PKH",
        );
        insert_output(&conn, &output3).unwrap();

        // Find spendable outputs
        let spendable = find_spendable_outputs_for_user(&conn, 1, None, None).unwrap();
        assert_eq!(spendable.len(), 1);
        assert_eq!(spendable[0].satoshis, 5000);
        assert!(spendable[0].spendable);
        assert!(spendable[0].spent_by.is_none());
    }

    #[test]
    fn test_output_optional_fields() {
        let conn = create_test_storage();
        
        let mut output = TableOutput::new(
            0, 1, 1,
            true, false,
            "With optional fields",
            0, 5000,
            StorageProvidedBy::YouAndStorage,
            "payment",
            "custom",
        );
        output.sender_identity_key = Some("0123456789abcdef".to_string());
        output.derivation_prefix = Some("prefix_base64".to_string());
        output.derivation_suffix = Some("suffix_base64".to_string());
        output.custom_instructions = Some("custom data".to_string());
        output.sequence_number = Some(0xFFFFFFFF);
        output.script_length = Some(25);
        output.script_offset = Some(100);

        let output_id = insert_output(&conn, &output).unwrap();
        
        let found = find_output_by_id(&conn, output_id, true).unwrap().unwrap();
        assert_eq!(found.sender_identity_key, Some("0123456789abcdef".to_string()));
        assert_eq!(found.derivation_prefix, Some("prefix_base64".to_string()));
        assert_eq!(found.custom_instructions, Some("custom data".to_string()));
        assert_eq!(found.sequence_number, Some(0xFFFFFFFF));
        assert_eq!(found.script_length, Some(25));
    }
}
