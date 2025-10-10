//! Comprehensive tests for SDK types module
//! 
//! Tests functional parity with wallet-toolbox/src/sdk/types.ts

#[cfg(test)]
mod tests {
    use super::super::types::*;

    // ========================================================================
    // OutPoint Tests
    // ========================================================================

    #[test]
    fn test_outpoint_new() {
        let outpoint = OutPoint::new("abc123", 0);
        assert_eq!(outpoint.txid, "abc123");
        assert_eq!(outpoint.vout, 0);
    }

    #[test]
    fn test_outpoint_to_string_format() {
        let outpoint = OutPoint::new("abc123", 5);
        assert_eq!(outpoint.to_string_format(), "abc123:5");
    }

    #[test]
    fn test_outpoint_from_string_format() {
        let outpoint = OutPoint::from_string_format("abc123:5").unwrap();
        assert_eq!(outpoint.txid, "abc123");
        assert_eq!(outpoint.vout, 5);
    }

    #[test]
    fn test_outpoint_from_string_format_invalid() {
        assert!(OutPoint::from_string_format("invalid").is_err());
        assert!(OutPoint::from_string_format("abc:not_a_number").is_err());
        assert!(OutPoint::from_string_format("").is_err());
    }

    #[test]
    fn test_outpoint_serialization() {
        let outpoint = OutPoint::new("abc123def456", 5);
        let json = serde_json::to_string(&outpoint).unwrap();
        let deserialized: OutPoint = serde_json::from_str(&json).unwrap();
        assert_eq!(outpoint, deserialized);
    }

    #[test]
    fn test_outpoint_equality() {
        let op1 = OutPoint::new("txid1", 0);
        let op2 = OutPoint::new("txid1", 0);
        let op3 = OutPoint::new("txid2", 0);
        let op4 = OutPoint::new("txid1", 1);
        
        assert_eq!(op1, op2);
        assert_ne!(op1, op3);
        assert_ne!(op1, op4);
    }

    // ========================================================================
    // Chain Tests
    // ========================================================================

    #[test]
    fn test_chain_as_str() {
        assert_eq!(Chain::Main.as_str(), "main");
        assert_eq!(Chain::Test.as_str(), "test");
    }

    #[test]
    fn test_chain_from_str() {
        assert_eq!(Chain::from_str("main").unwrap(), Chain::Main);
        assert_eq!(Chain::from_str("test").unwrap(), Chain::Test);
        assert!(Chain::from_str("invalid").is_err());
        assert!(Chain::from_str("").is_err());
        assert!(Chain::from_str("MAIN").is_err()); // Case sensitive
    }

    #[test]
    fn test_chain_display() {
        assert_eq!(format!("{}", Chain::Main), "main");
        assert_eq!(format!("{}", Chain::Test), "test");
    }

    #[test]
    fn test_chain_serialization() {
        let main = Chain::Main;
        let json = serde_json::to_string(&main).unwrap();
        assert_eq!(json, "\"main\"");
        
        let test = Chain::Test;
        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, "\"test\"");
    }

    #[test]
    fn test_chain_deserialization() {
        let main: Chain = serde_json::from_str("\"main\"").unwrap();
        assert_eq!(main, Chain::Main);
        
        let test: Chain = serde_json::from_str("\"test\"").unwrap();
        assert_eq!(test, Chain::Test);
    }

    // ========================================================================
    // ProvenTxReqStatus Tests
    // ========================================================================

    #[test]
    fn test_proven_tx_req_status_terminal() {
        assert!(ProvenTxReqStatus::Completed.is_terminal());
        assert!(ProvenTxReqStatus::Invalid.is_terminal());
        assert!(ProvenTxReqStatus::DoubleSpend.is_terminal());
        
        assert!(!ProvenTxReqStatus::Sending.is_terminal());
        assert!(!ProvenTxReqStatus::Unsent.is_terminal());
        assert!(!ProvenTxReqStatus::Unmined.is_terminal());
    }

    #[test]
    fn test_proven_tx_req_status_non_terminal() {
        assert!(ProvenTxReqStatus::Sending.is_non_terminal());
        assert!(ProvenTxReqStatus::Unsent.is_non_terminal());
        assert!(ProvenTxReqStatus::Unmined.is_non_terminal());
        
        assert!(!ProvenTxReqStatus::Completed.is_non_terminal());
        assert!(!ProvenTxReqStatus::Invalid.is_non_terminal());
    }

    #[test]
    fn test_proven_tx_req_status_terminal_statuses() {
        let terminal = ProvenTxReqStatus::terminal_statuses();
        assert_eq!(terminal.len(), 3);
        assert!(terminal.contains(&ProvenTxReqStatus::Completed));
        assert!(terminal.contains(&ProvenTxReqStatus::Invalid));
        assert!(terminal.contains(&ProvenTxReqStatus::DoubleSpend));
    }

    #[test]
    fn test_proven_tx_req_status_non_terminal_statuses() {
        let non_terminal = ProvenTxReqStatus::non_terminal_statuses();
        assert_eq!(non_terminal.len(), 9);
        assert!(non_terminal.contains(&ProvenTxReqStatus::Sending));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Unsent));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Nosend));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Unknown));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Nonfinal));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Unprocessed));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Unmined));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Callback));
        assert!(non_terminal.contains(&ProvenTxReqStatus::Unconfirmed));
    }

    #[test]
    fn test_proven_tx_req_status_as_str() {
        assert_eq!(ProvenTxReqStatus::Sending.as_str(), "sending");
        assert_eq!(ProvenTxReqStatus::Unsent.as_str(), "unsent");
        assert_eq!(ProvenTxReqStatus::Nosend.as_str(), "nosend");
        assert_eq!(ProvenTxReqStatus::DoubleSpend.as_str(), "doubleSpend");
        assert_eq!(ProvenTxReqStatus::Completed.as_str(), "completed");
    }

    #[test]
    fn test_proven_tx_req_status_display() {
        assert_eq!(format!("{}", ProvenTxReqStatus::Sending), "sending");
        assert_eq!(format!("{}", ProvenTxReqStatus::DoubleSpend), "doubleSpend");
    }

    #[test]
    fn test_proven_tx_req_status_serialization() {
        let status = ProvenTxReqStatus::DoubleSpend;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"doubleSpend\"");
        let deserialized: ProvenTxReqStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_proven_tx_req_status_all_variants_serialize() {
        // Ensure all variants can serialize/deserialize correctly
        let statuses = vec![
            ProvenTxReqStatus::Sending,
            ProvenTxReqStatus::Unsent,
            ProvenTxReqStatus::Nosend,
            ProvenTxReqStatus::Unknown,
            ProvenTxReqStatus::Nonfinal,
            ProvenTxReqStatus::Unprocessed,
            ProvenTxReqStatus::Unmined,
            ProvenTxReqStatus::Callback,
            ProvenTxReqStatus::Unconfirmed,
            ProvenTxReqStatus::Completed,
            ProvenTxReqStatus::Invalid,
            ProvenTxReqStatus::DoubleSpend,
            ProvenTxReqStatus::Unfail,
        ];
        
        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let deserialized: ProvenTxReqStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    // ========================================================================
    // TransactionStatus Tests
    // ========================================================================

    #[test]
    fn test_transaction_status_as_str() {
        assert_eq!(TransactionStatus::Completed.as_str(), "completed");
        assert_eq!(TransactionStatus::Failed.as_str(), "failed");
        assert_eq!(TransactionStatus::Unprocessed.as_str(), "unprocessed");
        assert_eq!(TransactionStatus::Sending.as_str(), "sending");
    }

    #[test]
    fn test_transaction_status_display() {
        assert_eq!(format!("{}", TransactionStatus::Sending), "sending");
        assert_eq!(format!("{}", TransactionStatus::Failed), "failed");
    }

    #[test]
    fn test_transaction_status_serialization() {
        let status = TransactionStatus::Completed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"completed\"");
        let deserialized: TransactionStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    // ========================================================================
    // Paged Tests
    // ========================================================================

    #[test]
    fn test_paged_new() {
        let paged = Paged::new(10);
        assert_eq!(paged.limit, 10);
        assert_eq!(paged.offset, None);
    }

    #[test]
    fn test_paged_with_offset() {
        let paged = Paged::with_offset(20, 100);
        assert_eq!(paged.limit, 20);
        assert_eq!(paged.offset, Some(100));
    }

    #[test]
    fn test_paged_serialization() {
        let paged = Paged::with_offset(10, 50);
        let json = serde_json::to_string(&paged).unwrap();
        let deserialized: Paged = serde_json::from_str(&json).unwrap();
        assert_eq!(paged, deserialized);
    }

    #[test]
    fn test_paged_serialization_without_offset() {
        let paged = Paged::new(25);
        let json = serde_json::to_string(&paged).unwrap();
        // Should not include offset field when None
        assert!(!json.contains("offset"));
        assert!(json.contains("\"limit\":25"));
    }

    // ========================================================================
    // KeyPair Tests
    // ========================================================================

    #[test]
    fn test_keypair_new() {
        let kp = KeyPair::new("private", "public");
        assert_eq!(kp.private_key, "private");
        assert_eq!(kp.public_key, "public");
    }

    #[test]
    fn test_keypair_serialization() {
        let kp = KeyPair::new("priv123", "pub456");
        let json = serde_json::to_string(&kp).unwrap();
        assert!(json.contains("privateKey"));
        assert!(json.contains("publicKey"));
        let deserialized: KeyPair = serde_json::from_str(&json).unwrap();
        assert_eq!(kp, deserialized);
    }

    #[test]
    fn test_keypair_field_names() {
        let kp = KeyPair::new("priv", "pub");
        let json = serde_json::to_string(&kp).unwrap();
        // Should use camelCase for JSON compatibility with TypeScript
        assert!(json.contains("\"privateKey\":\"priv\""));
        assert!(json.contains("\"publicKey\":\"pub\""));
    }

    // ========================================================================
    // StorageIdentity Tests
    // ========================================================================

    #[test]
    fn test_storage_identity_new() {
        let si = StorageIdentity::new("key123", "MyStorage");
        assert_eq!(si.storage_identity_key, "key123");
        assert_eq!(si.storage_name, "MyStorage");
    }

    #[test]
    fn test_storage_identity_serialization() {
        let si = StorageIdentity::new("key", "name");
        let json = serde_json::to_string(&si).unwrap();
        assert!(json.contains("storageIdentityKey"));
        assert!(json.contains("storageName"));
        let deserialized: StorageIdentity = serde_json::from_str(&json).unwrap();
        assert_eq!(si, deserialized);
    }

    // ========================================================================
    // EntityTimeStamp Tests
    // ========================================================================

    #[test]
    fn test_entity_timestamp_now() {
        let ts = EntityTimeStamp::now();
        assert!(!ts.created_at.is_empty());
        assert!(!ts.updated_at.is_empty());
        // Both should be equal when created with now()
        assert_eq!(ts.created_at, ts.updated_at);
    }

    #[test]
    fn test_entity_timestamp_new() {
        let ts = EntityTimeStamp::new("2024-01-01T00:00:00Z", "2024-01-02T00:00:00Z");
        assert_eq!(ts.created_at, "2024-01-01T00:00:00Z");
        assert_eq!(ts.updated_at, "2024-01-02T00:00:00Z");
    }

    #[test]
    fn test_entity_timestamp_serialization() {
        let ts = EntityTimeStamp::new("2024-01-01T00:00:00Z", "2024-01-02T00:00:00Z");
        let json = serde_json::to_string(&ts).unwrap();
        let deserialized: EntityTimeStamp = serde_json::from_str(&json).unwrap();
        assert_eq!(ts, deserialized);
    }

    // ========================================================================
    // WalletBalance Tests
    // ========================================================================

    #[test]
    fn test_wallet_balance_empty() {
        let balance = WalletBalance::empty();
        assert_eq!(balance.total, 0);
        assert_eq!(balance.utxos.len(), 0);
    }

    #[test]
    fn test_wallet_balance_new() {
        let utxos = vec![
            UtxoBalance {
                satoshis: 1000,
                outpoint: "abc:0".to_string(),
            },
            UtxoBalance {
                satoshis: 2000,
                outpoint: "def:1".to_string(),
            },
        ];
        let balance = WalletBalance::new(3000, utxos);
        assert_eq!(balance.total, 3000);
        assert_eq!(balance.utxos.len(), 2);
        assert_eq!(balance.utxos[0].satoshis, 1000);
        assert_eq!(balance.utxos[1].satoshis, 2000);
    }

    #[test]
    fn test_wallet_balance_serialization() {
        let utxos = vec![UtxoBalance {
            satoshis: 5000,
            outpoint: "txid:0".to_string(),
        }];
        let balance = WalletBalance::new(5000, utxos);
        let json = serde_json::to_string(&balance).unwrap();
        let deserialized: WalletBalance = serde_json::from_str(&json).unwrap();
        assert_eq!(balance, deserialized);
    }

    // ========================================================================
    // ReqHistoryNote Tests
    // ========================================================================

    #[test]
    fn test_req_history_note_new() {
        let note = ReqHistoryNote::new("Transaction created");
        assert_eq!(note.what, "Transaction created");
        assert!(note.when.is_none());
        assert!(note.extra.is_empty());
    }

    #[test]
    fn test_req_history_note_with_timestamp() {
        let note = ReqHistoryNote::with_timestamp("TX sent", "2024-01-01T00:00:00Z");
        assert_eq!(note.what, "TX sent");
        assert_eq!(note.when, Some("2024-01-01T00:00:00Z".to_string()));
        assert!(note.extra.is_empty());
    }

    #[test]
    fn test_req_history_note_with_field() {
        let note = ReqHistoryNote::new("Event")
            .with_field("status", serde_json::json!("success"))
            .with_field("count", serde_json::json!(42))
            .with_field("verified", serde_json::json!(true));
        
        assert_eq!(note.extra.len(), 3);
        assert_eq!(note.extra.get("status").unwrap(), &serde_json::json!("success"));
        assert_eq!(note.extra.get("count").unwrap(), &serde_json::json!(42));
        assert_eq!(note.extra.get("verified").unwrap(), &serde_json::json!(true));
    }

    #[test]
    fn test_req_history_note_serialization() {
        let note = ReqHistoryNote::new("test")
            .with_field("custom", serde_json::json!(true))
            .with_field("value", serde_json::json!(123));
        let json = serde_json::to_string(&note).unwrap();
        let deserialized: ReqHistoryNote = serde_json::from_str(&json).unwrap();
        assert_eq!(note.what, deserialized.what);
        assert_eq!(note.extra.get("custom"), deserialized.extra.get("custom"));
        assert_eq!(note.extra.get("value"), deserialized.extra.get("value"));
    }

    #[test]
    fn test_req_history_note_flatten_extra_fields() {
        // Test that extra fields are flattened in JSON (not nested under "extra")
        let note = ReqHistoryNote::new("test")
            .with_field("myField", serde_json::json!("myValue"));
        let json = serde_json::to_string(&note).unwrap();
        
        // Should have myField at top level, not nested
        assert!(json.contains("\"myField\":\"myValue\""));
        assert!(!json.contains("\"extra\""));
    }

    // ========================================================================
    // Special Operation Constants Tests
    // ========================================================================

    #[test]
    fn test_spec_op_constants_match_typescript() {
        // Verify exact hash values match TypeScript source
        assert_eq!(
            SPEC_OP_WALLET_BALANCE,
            "893b7646de0e1c9f741bd6e9169b76a8847ae34adef7bef1e6a285371206d2e8"
        );
        assert_eq!(
            SPEC_OP_INVALID_CHANGE,
            "5a76fd430a311f8bc0553859061710a4475c19fed46e2ff95969aa918e612e57"
        );
        assert_eq!(
            SPEC_OP_SET_WALLET_CHANGE_PARAMS,
            "a4979d28ced8581e9c1c92f1001cc7cb3aabf8ea32e10888ad898f0a509a3929"
        );
        assert_eq!(
            SPEC_OP_NO_SEND_ACTIONS,
            "ac6b20a3bb320adafecd637b25c84b792ad828d3aa510d05dc841481f664277d"
        );
        assert_eq!(
            SPEC_OP_FAILED_ACTIONS,
            "97d4eb1e49215e3374cc2c1939a7c43a55e95c7427bf2d45ed63e3b4e0c88153"
        );
        assert_eq!(
            SPEC_OP_THROW_REVIEW_ACTIONS,
            "a496e747fc3ad5fabdd4ae8f91184e71f87539bd3d962aa2548942faaaf0047a"
        );
    }

    #[test]
    fn test_is_list_outputs_spec_op() {
        assert!(is_list_outputs_spec_op(SPEC_OP_WALLET_BALANCE));
        assert!(is_list_outputs_spec_op(SPEC_OP_INVALID_CHANGE));
        assert!(is_list_outputs_spec_op(SPEC_OP_SET_WALLET_CHANGE_PARAMS));
        
        assert!(!is_list_outputs_spec_op("random_basket"));
        assert!(!is_list_outputs_spec_op(""));
        assert!(!is_list_outputs_spec_op(SPEC_OP_NO_SEND_ACTIONS));
    }

    #[test]
    fn test_is_list_actions_spec_op() {
        assert!(is_list_actions_spec_op(SPEC_OP_NO_SEND_ACTIONS));
        assert!(is_list_actions_spec_op(SPEC_OP_FAILED_ACTIONS));
        
        assert!(!is_list_actions_spec_op("random_label"));
        assert!(!is_list_actions_spec_op(""));
        assert!(!is_list_actions_spec_op(SPEC_OP_WALLET_BALANCE));
    }

    #[test]
    fn test_is_create_action_spec_op() {
        assert!(is_create_action_spec_op(SPEC_OP_THROW_REVIEW_ACTIONS));
        
        assert!(!is_create_action_spec_op("random_label"));
        assert!(!is_create_action_spec_op(""));
        assert!(!is_create_action_spec_op(SPEC_OP_NO_SEND_ACTIONS));
    }

    #[test]
    fn test_spec_op_functions_exclusive() {
        // Ensure spec op identifiers are exclusive to their categories
        let all_spec_ops = vec![
            SPEC_OP_WALLET_BALANCE,
            SPEC_OP_INVALID_CHANGE,
            SPEC_OP_SET_WALLET_CHANGE_PARAMS,
            SPEC_OP_NO_SEND_ACTIONS,
            SPEC_OP_FAILED_ACTIONS,
            SPEC_OP_THROW_REVIEW_ACTIONS,
        ];
        
        for spec_op in all_spec_ops {
            let is_output = is_list_outputs_spec_op(spec_op);
            let is_action = is_list_actions_spec_op(spec_op);
            let is_create = is_create_action_spec_op(spec_op);
            
            // Each spec op should belong to exactly one category
            let count = [is_output, is_action, is_create].iter().filter(|&&x| x).count();
            assert_eq!(count, 1, "Spec op {} belongs to {} categories", spec_op, count);
        }
    }
}
