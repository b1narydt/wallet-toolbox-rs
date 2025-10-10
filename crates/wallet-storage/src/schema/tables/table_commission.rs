//! TableCommission - Commission records
//!
//! Translates TypeScript TableCommission interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableCommission.ts

use serde::{Deserialize, Serialize};

/// Commission table - stores commission payment records
///
/// Matches TypeScript `TableCommission` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableCommission {
    pub created_at: String,
    pub updated_at: String,
    
    #[serde(rename = "commissionId")]
    pub commission_id: i64,
    
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
    
    pub satoshis: i64,
    
    #[serde(rename = "keyOffset")]
    pub key_offset: String,
    
    #[serde(rename = "isRedeemed")]
    pub is_redeemed: bool,
    
    #[serde(rename = "lockingScript")]
    pub locking_script: Vec<u8>,
}

impl TableCommission {
    pub fn new(
        commission_id: i64,
        user_id: i64,
        transaction_id: i64,
        satoshis: i64,
        key_offset: impl Into<String>,
        locking_script: Vec<u8>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            commission_id,
            user_id,
            transaction_id,
            satoshis,
            key_offset: key_offset.into(),
            is_redeemed: false,
            locking_script,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn redeem(&mut self) {
        self.is_redeemed = true;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_commission_new() {
        let commission = TableCommission::new(1, 100, 200, 5000, "offset123", vec![1, 2, 3]);
        assert_eq!(commission.commission_id, 1);
        assert_eq!(commission.user_id, 100);
        assert_eq!(commission.transaction_id, 200);
        assert_eq!(commission.satoshis, 5000);
        assert_eq!(commission.key_offset, "offset123");
        assert_eq!(commission.is_redeemed, false);
        assert_eq!(commission.locking_script, vec![1, 2, 3]);
    }

    #[test]
    fn test_table_commission_redeem() {
        let mut commission = TableCommission::new(1, 100, 200, 1000, "key", vec![]);
        assert_eq!(commission.is_redeemed, false);
        commission.redeem();
        assert_eq!(commission.is_redeemed, true);
    }

    #[test]
    fn test_table_commission_serialization() {
        let commission = TableCommission::new(1, 100, 200, 5000, "key", vec![1, 2]);
        let json = serde_json::to_string(&commission).unwrap();
        assert!(json.contains("\"commissionId\":1"));
        assert!(json.contains("\"userId\":100"));
        assert!(json.contains("\"transactionId\":200"));
        assert!(json.contains("\"keyOffset\":\"key\""));
        let deserialized: TableCommission = serde_json::from_str(&json).unwrap();
        assert_eq!(commission, deserialized);
    }
}
