//! TableOutput - Output (UTXO) records
//!
//! Translates TypeScript TableOutput interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableOutput.ts

use serde::{Deserialize, Serialize};

/// Storage provider type (local copy to avoid circular dependency)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageProvidedBy {
    You,
    Storage,
    YouAndStorage,
}

impl std::fmt::Display for StorageProvidedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageProvidedBy::You => write!(f, "you"),
            StorageProvidedBy::Storage => write!(f, "storage"),
            StorageProvidedBy::YouAndStorage => write!(f, "you-and-storage"),
        }
    }
}

impl std::str::FromStr for StorageProvidedBy {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "you" => Ok(StorageProvidedBy::You),
            "storage" => Ok(StorageProvidedBy::Storage),
            "you-and-storage" => Ok(StorageProvidedBy::YouAndStorage),
            _ => Err(format!("Invalid StorageProvidedBy: {}", s)),
        }
    }
}

/// Output table - stores transaction outputs (UTXOs)
///
/// Matches TypeScript `TableOutput` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableOutput {
    /// Record creation timestamp (ISO 8601 string)
    pub created_at: String,
    
    /// Record last update timestamp (ISO 8601 string)
    pub updated_at: String,
    
    /// Primary key - unique output identifier
    #[serde(rename = "outputId")]
    pub output_id: i64,
    
    /// Foreign key to user
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    /// Foreign key to transaction
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
    
    /// Optional foreign key to basket
    #[serde(rename = "basketId", skip_serializing_if = "Option::is_none")]
    pub basket_id: Option<i64>,
    
    /// Whether output is spendable
    pub spendable: bool,
    
    /// Whether output is change
    pub change: bool,
    
    /// Output description (5-50 bytes)
    #[serde(rename = "outputDescription")]
    pub output_description: String,
    
    /// Output index in transaction
    pub vout: u32,
    
    /// Satoshi amount
    pub satoshis: i64,
    
    /// Who provided this output
    #[serde(rename = "providedBy")]
    pub provided_by: StorageProvidedBy,
    
    /// Output purpose
    pub purpose: String,
    
    /// Output type
    #[serde(rename = "type")]
    pub output_type: String,
    
    /// Optional transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    
    /// Optional sender identity key (public key hex)
    #[serde(rename = "senderIdentityKey", skip_serializing_if = "Option::is_none")]
    pub sender_identity_key: Option<String>,
    
    /// Optional derivation prefix (base64)
    #[serde(rename = "derivationPrefix", skip_serializing_if = "Option::is_none")]
    pub derivation_prefix: Option<String>,
    
    /// Optional derivation suffix (base64)
    #[serde(rename = "derivationSuffix", skip_serializing_if = "Option::is_none")]
    pub derivation_suffix: Option<String>,
    
    /// Optional custom instructions
    #[serde(rename = "customInstructions", skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    
    /// Optional foreign key to spending transaction
    #[serde(rename = "spentBy", skip_serializing_if = "Option::is_none")]
    pub spent_by: Option<i64>,
    
    /// Optional sequence number
    #[serde(rename = "sequenceNumber", skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u32>,
    
    /// Optional spending description
    #[serde(rename = "spendingDescription", skip_serializing_if = "Option::is_none")]
    pub spending_description: Option<String>,
    
    /// Optional script length
    #[serde(rename = "scriptLength", skip_serializing_if = "Option::is_none")]
    pub script_length: Option<u32>,
    
    /// Optional script offset
    #[serde(rename = "scriptOffset", skip_serializing_if = "Option::is_none")]
    pub script_offset: Option<u32>,
    
    /// Optional locking script (byte array)
    #[serde(rename = "lockingScript", skip_serializing_if = "Option::is_none")]
    pub locking_script: Option<Vec<u8>>,
}

impl TableOutput {
    /// Create a new TableOutput with required fields
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        output_id: i64,
        user_id: i64,
        transaction_id: i64,
        spendable: bool,
        change: bool,
        output_description: impl Into<String>,
        vout: u32,
        satoshis: i64,
        provided_by: StorageProvidedBy,
        purpose: impl Into<String>,
        output_type: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            output_id,
            user_id,
            transaction_id,
            basket_id: None,
            spendable,
            change,
            output_description: output_description.into(),
            vout,
            satoshis,
            provided_by,
            purpose: purpose.into(),
            output_type: output_type.into(),
            txid: None,
            sender_identity_key: None,
            derivation_prefix: None,
            derivation_suffix: None,
            custom_instructions: None,
            spent_by: None,
            sequence_number: None,
            spending_description: None,
            script_length: None,
            script_offset: None,
            locking_script: None,
        }
    }

    /// Builder: set basket_id
    pub fn with_basket_id(mut self, basket_id: i64) -> Self {
        self.basket_id = Some(basket_id);
        self
    }

    /// Builder: set txid
    pub fn with_txid(mut self, txid: impl Into<String>) -> Self {
        self.txid = Some(txid.into());
        self
    }

    /// Builder: set sender_identity_key
    pub fn with_sender_identity_key(mut self, key: impl Into<String>) -> Self {
        self.sender_identity_key = Some(key.into());
        self
    }

    /// Builder: set derivation_prefix
    pub fn with_derivation_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.derivation_prefix = Some(prefix.into());
        self
    }

    /// Builder: set derivation_suffix
    pub fn with_derivation_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.derivation_suffix = Some(suffix.into());
        self
    }

    /// Builder: set custom_instructions
    pub fn with_custom_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.custom_instructions = Some(instructions.into());
        self
    }

    /// Builder: set locking_script
    pub fn with_locking_script(mut self, script: Vec<u8>) -> Self {
        self.locking_script = Some(script);
        self
    }

    /// Mark output as spent
    pub fn mark_spent(&mut self, spent_by: i64, spending_description: Option<String>, sequence_number: Option<u32>) {
        self.spent_by = Some(spent_by);
        self.spending_description = spending_description;
        self.sequence_number = sequence_number;
        self.spendable = false;
        self.touch();
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Get columns without lockingScript (matches TypeScript outputColumnsWithoutLockingScript)
    pub fn columns_without_locking_script() -> &'static [&'static str] {
        &[
            "created_at",
            "updated_at",
            "outputId",
            "userId",
            "transactionId",
            "basketId",
            "spendable",
            "change",
            "vout",
            "satoshis",
            "providedBy",
            "purpose",
            "type",
            "outputDescription",
            "txid",
            "senderIdentityKey",
            "derivationPrefix",
            "derivationSuffix",
            "customInstructions",
            "spentBy",
            "sequenceNumber",
            "spendingDescription",
            "scriptLength",
            "scriptOffset",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_output_new() {
        let output = TableOutput::new(
            1, 100, 200, true, false,
            "test output", 0, 5000,
            StorageProvidedBy::You, "payment", "P2PKH"
        );
        
        assert_eq!(output.output_id, 1);
        assert_eq!(output.user_id, 100);
        assert_eq!(output.transaction_id, 200);
        assert_eq!(output.spendable, true);
        assert_eq!(output.change, false);
        assert_eq!(output.output_description, "test output");
        assert_eq!(output.vout, 0);
        assert_eq!(output.satoshis, 5000);
        assert_eq!(output.provided_by, StorageProvidedBy::You);
        assert_eq!(output.purpose, "payment");
        assert_eq!(output.output_type, "P2PKH");
        assert!(output.basket_id.is_none());
    }

    #[test]
    fn test_table_output_builder() {
        let output = TableOutput::new(
            1, 100, 200, true, false,
            "desc", 0, 1000,
            StorageProvidedBy::Storage, "test", "type"
        )
        .with_basket_id(50)
        .with_txid("abc123")
        .with_sender_identity_key("pubkey123")
        .with_custom_instructions("custom");
        
        assert_eq!(output.basket_id, Some(50));
        assert_eq!(output.txid, Some("abc123".to_string()));
        assert_eq!(output.sender_identity_key, Some("pubkey123".to_string()));
        assert_eq!(output.custom_instructions, Some("custom".to_string()));
    }

    #[test]
    fn test_table_output_mark_spent() {
        let mut output = TableOutput::new(
            1, 100, 200, true, false,
            "desc", 0, 1000,
            StorageProvidedBy::You, "test", "type"
        );
        
        assert_eq!(output.spendable, true);
        assert!(output.spent_by.is_none());
        
        output.mark_spent(300, Some("spending".to_string()), Some(1));
        
        assert_eq!(output.spendable, false);
        assert_eq!(output.spent_by, Some(300));
        assert_eq!(output.spending_description, Some("spending".to_string()));
        assert_eq!(output.sequence_number, Some(1));
    }

    #[test]
    fn test_table_output_serialization() {
        let output = TableOutput::new(
            1, 100, 200, true, false,
            "test", 0, 5000,
            StorageProvidedBy::You, "payment", "P2PKH"
        );
        
        let json = serde_json::to_string(&output).unwrap();
        
        // Check camelCase field names
        assert!(json.contains("\"outputId\":1"));
        assert!(json.contains("\"userId\":100"));
        assert!(json.contains("\"transactionId\":200"));
        assert!(json.contains("\"outputDescription\":\"test\""));
        assert!(json.contains("\"providedBy\":\"you\""));
        
        let deserialized: TableOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(output, deserialized);
    }

    #[test]
    fn test_table_output_optional_fields_not_serialized() {
        let output = TableOutput::new(
            1, 100, 200, true, false,
            "desc", 0, 1000,
            StorageProvidedBy::You, "test", "type"
        );
        
        let json = serde_json::to_string(&output).unwrap();
        
        // Optional None fields should not appear
        assert!(!json.contains("\"basketId\""));
        assert!(!json.contains("\"txid\""));
        assert!(!json.contains("\"senderIdentityKey\""));
        assert!(!json.contains("\"derivationPrefix\""));
        assert!(!json.contains("\"spentBy\""));
        assert!(!json.contains("\"lockingScript\""));
    }

    #[test]
    fn test_storage_provided_by_serialization() {
        assert_eq!(
            serde_json::to_string(&StorageProvidedBy::You).unwrap(),
            "\"you\""
        );
        assert_eq!(
            serde_json::to_string(&StorageProvidedBy::Storage).unwrap(),
            "\"storage\""
        );
        assert_eq!(
            serde_json::to_string(&StorageProvidedBy::YouAndStorage).unwrap(),
            "\"you-and-storage\""
        );
    }

    #[test]
    fn test_columns_without_locking_script() {
        let columns = TableOutput::columns_without_locking_script();
        
        assert_eq!(columns.len(), 24);
        assert!(columns.contains(&"outputId"));
        assert!(columns.contains(&"userId"));
        assert!(columns.contains(&"satoshis"));
        assert!(!columns.contains(&"lockingScript"));
    }

    #[test]
    fn test_table_output_touch() {
        let mut output = TableOutput::new(
            1, 100, 200, true, false,
            "desc", 0, 1000,
            StorageProvidedBy::You, "test", "type"
        );
        
        let original_updated = output.updated_at.clone();
        std::thread::sleep(std::time::Duration::from_millis(10));
        output.touch();
        
        assert_ne!(output.updated_at, original_updated);
    }

    #[test]
    fn test_table_output_clone() {
        let output = TableOutput::new(
            1, 100, 200, true, false,
            "desc", 0, 1000,
            StorageProvidedBy::You, "test", "type"
        );
        let cloned = output.clone();
        
        assert_eq!(output, cloned);
    }
}
