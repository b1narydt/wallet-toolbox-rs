//! EntityOutput - Output entity wrapper
//!
//! Translates TypeScript EntityOutput class to Rust.
//! Reference: wallet-toolbox/src/storage/schema/entities/EntityOutput.ts

use crate::schema::tables::{TableOutput, StorageProvidedBy};
use super::{EntityBase, SyncMap};

/// Output entity wrapper providing merge logic and property accessors
///
/// Matches TypeScript `EntityOutput` class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityOutput {
    api: TableOutput,
}

impl EntityOutput {
    /// Create new EntityOutput from table record
    pub fn new(api: Option<TableOutput>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            api: api.unwrap_or_else(|| TableOutput {
                created_at: now.clone(),
                updated_at: now,
                output_id: 0,
                user_id: 0,
                transaction_id: 0,
                basket_id: None,
                spendable: false,
                change: false,
                output_description: String::new(),
                vout: 0,
                satoshis: 0,
                provided_by: StorageProvidedBy::You,
                purpose: String::new(),
                output_type: String::new(),
                txid: None,
                spent_by: None,
                spending_description: None,
                derivation_prefix: None,
                derivation_suffix: None,
                sender_identity_key: None,
                custom_instructions: None,
                locking_script: None,
                script_length: None,
                script_offset: None,
                sequence_number: None,
            }),
        }
    }

    // Property accessors matching TypeScript getters/setters

    pub fn output_id(&self) -> i64 {
        self.api.output_id
    }

    pub fn set_output_id(&mut self, v: i64) {
        self.api.output_id = v;
    }

    pub fn created_at(&self) -> &str {
        &self.api.created_at
    }

    pub fn set_created_at(&mut self, v: impl Into<String>) {
        self.api.created_at = v.into();
    }

    pub fn updated_at(&self) -> &str {
        &self.api.updated_at
    }

    pub fn set_updated_at(&mut self, v: impl Into<String>) {
        self.api.updated_at = v.into();
    }

    pub fn user_id(&self) -> i64 {
        self.api.user_id
    }

    pub fn set_user_id(&mut self, v: i64) {
        self.api.user_id = v;
    }

    pub fn transaction_id(&self) -> i64 {
        self.api.transaction_id
    }

    pub fn set_transaction_id(&mut self, v: i64) {
        self.api.transaction_id = v;
    }

    pub fn basket_id(&self) -> Option<i64> {
        self.api.basket_id
    }

    pub fn set_basket_id(&mut self, v: Option<i64>) {
        self.api.basket_id = v;
    }

    pub fn spent_by(&self) -> Option<i64> {
        self.api.spent_by
    }

    pub fn set_spent_by(&mut self, v: Option<i64>) {
        self.api.spent_by = v;
    }

    pub fn vout(&self) -> u32 {
        self.api.vout
    }

    pub fn set_vout(&mut self, v: u32) {
        self.api.vout = v;
    }

    pub fn satoshis(&self) -> i64 {
        self.api.satoshis
    }

    pub fn set_satoshis(&mut self, v: i64) {
        self.api.satoshis = v;
    }

    pub fn output_description(&self) -> &str {
        &self.api.output_description
    }

    pub fn set_output_description(&mut self, v: impl Into<String>) {
        self.api.output_description = v.into();
    }

    pub fn spendable(&self) -> bool {
        self.api.spendable
    }

    pub fn set_spendable(&mut self, v: bool) {
        self.api.spendable = v;
    }

    pub fn change(&self) -> bool {
        self.api.change
    }

    pub fn set_change(&mut self, v: bool) {
        self.api.change = v;
    }

    pub fn txid(&self) -> Option<&str> {
        self.api.txid.as_deref()
    }

    pub fn set_txid(&mut self, v: Option<String>) {
        self.api.txid = v;
    }

    pub fn output_type(&self) -> &str {
        &self.api.output_type
    }

    pub fn set_output_type(&mut self, v: impl Into<String>) {
        self.api.output_type = v.into();
    }

    pub fn provided_by(&self) -> StorageProvidedBy {
        self.api.provided_by
    }

    pub fn set_provided_by(&mut self, v: StorageProvidedBy) {
        self.api.provided_by = v;
    }

    pub fn purpose(&self) -> &str {
        &self.api.purpose
    }

    pub fn set_purpose(&mut self, v: impl Into<String>) {
        self.api.purpose = v.into();
    }

    pub fn spending_description(&self) -> Option<&str> {
        self.api.spending_description.as_deref()
    }

    pub fn set_spending_description(&mut self, v: Option<String>) {
        self.api.spending_description = v;
    }

    pub fn derivation_prefix(&self) -> Option<&str> {
        self.api.derivation_prefix.as_deref()
    }

    pub fn set_derivation_prefix(&mut self, v: Option<String>) {
        self.api.derivation_prefix = v;
    }

    pub fn derivation_suffix(&self) -> Option<&str> {
        self.api.derivation_suffix.as_deref()
    }

    pub fn set_derivation_suffix(&mut self, v: Option<String>) {
        self.api.derivation_suffix = v;
    }

    pub fn sender_identity_key(&self) -> Option<&str> {
        self.api.sender_identity_key.as_deref()
    }

    pub fn set_sender_identity_key(&mut self, v: Option<String>) {
        self.api.sender_identity_key = v;
    }

    pub fn custom_instructions(&self) -> Option<&str> {
        self.api.custom_instructions.as_deref()
    }

    pub fn set_custom_instructions(&mut self, v: Option<String>) {
        self.api.custom_instructions = v;
    }

    pub fn locking_script(&self) -> Option<&Vec<u8>> {
        self.api.locking_script.as_ref()
    }

    pub fn set_locking_script(&mut self, v: Option<Vec<u8>>) {
        self.api.locking_script = v;
    }

    pub fn script_length(&self) -> Option<u32> {
        self.api.script_length
    }

    pub fn set_script_length(&mut self, v: Option<u32>) {
        self.api.script_length = v;
    }

    pub fn script_offset(&self) -> Option<u32> {
        self.api.script_offset
    }

    pub fn set_script_offset(&mut self, v: Option<u32>) {
        self.api.script_offset = v;
    }

    /// Get mutable reference to underlying API
    pub fn get_api_mut(&mut self) -> &mut TableOutput {
        &mut self.api
    }

    /// Consume entity and return API
    pub fn into_api(self) -> TableOutput {
        self.api
    }

    /// Helper to compare optional byte arrays
    fn optional_arrays_equal(a: Option<&Vec<u8>>, b: Option<&Vec<u8>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }
}

impl EntityBase for EntityOutput {
    type Api = TableOutput;

    fn id(&self) -> i64 {
        self.api.output_id
    }

    fn set_id(&mut self, v: i64) {
        self.api.output_id = v;
    }

    fn entity_name(&self) -> &'static str {
        "output"
    }

    fn entity_table(&self) -> &'static str {
        "outputs"
    }

    fn update_api(&mut self) {
        // Nothing needed yet - matches TypeScript implementation
    }

    fn get_api(&self) -> &Self::Api {
        &self.api
    }

    fn equals(&self, other: &Self::Api, sync_map: Option<&SyncMap>) -> bool {
        // Match TypeScript equals logic exactly
        
        // Compare transactionId with optional sync map
        let other_transaction_id = if let Some(map) = sync_map {
            map.transaction.id_map.get(&other.transaction_id).copied().unwrap_or(other.transaction_id)
        } else {
            other.transaction_id
        };
        
        if self.transaction_id() != other_transaction_id {
            return false;
        }

        // Compare basketId with optional sync map
        let other_basket_id = if let Some(map) = sync_map {
            other.basket_id.and_then(|id| map.output_basket.id_map.get(&id).copied())
        } else {
            other.basket_id
        };
        
        if self.basket_id() != other_basket_id {
            return false;
        }

        // Compare spentBy with optional sync map
        let other_spent_by = if let Some(map) = sync_map {
            other.spent_by.and_then(|id| map.transaction.id_map.get(&id).copied())
        } else {
            other.spent_by
        };
        
        if self.spent_by() != other_spent_by {
            return false;
        }

        // Compare all other fields
        if self.vout() != other.vout
            || self.satoshis() != other.satoshis
            || self.spendable() != other.spendable
            || self.change() != other.change
            || self.txid() != other.txid.as_deref()
            || self.output_type() != other.output_type
            || self.provided_by() != other.provided_by
            || self.purpose() != other.purpose
            || self.output_description() != other.output_description
            || self.spending_description() != other.spending_description.as_deref()
            || self.derivation_prefix() != other.derivation_prefix.as_deref()
            || self.derivation_suffix() != other.derivation_suffix.as_deref()
            || self.sender_identity_key() != other.sender_identity_key.as_deref()
            || self.custom_instructions() != other.custom_instructions.as_deref()
            || !Self::optional_arrays_equal(self.locking_script(), other.locking_script.as_ref())
            || self.script_length() != other.script_length
            || self.script_offset() != other.script_offset
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_output_new_default() {
        let entity = EntityOutput::new(None);
        assert_eq!(entity.output_id(), 0);
        assert_eq!(entity.user_id(), 0);
        assert_eq!(entity.transaction_id(), 0);
        assert_eq!(entity.spendable(), false);
        assert_eq!(entity.change(), false);
        assert_eq!(entity.satoshis(), 0);
    }

    #[test]
    fn test_entity_output_new_with_api() {
        let output = TableOutput {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_id: 1,
            user_id: 100,
            transaction_id: 50,
            basket_id: Some(10),
            spendable: true,
            change: false,
            output_description: "Test output".to_string(),
            vout: 0,
            satoshis: 5000,
            provided_by: StorageProvidedBy::You,
            purpose: "payment".to_string(),
            output_type: "P2PKH".to_string(),
            txid: Some("abc123".to_string()),
            spent_by: None,
            spending_description: None,
            derivation_prefix: Some("prefix".to_string()),
            derivation_suffix: Some("suffix".to_string()),
            sender_identity_key: Some("key".to_string()),
            custom_instructions: None,
            locking_script: Some(vec![1, 2, 3]),
            script_length: Some(3),
            script_offset: Some(0),
            sequence_number: None,
        };

        let entity = EntityOutput::new(Some(output));
        assert_eq!(entity.output_id(), 1);
        assert_eq!(entity.user_id(), 100);
        assert_eq!(entity.transaction_id(), 50);
        assert_eq!(entity.basket_id(), Some(10));
        assert_eq!(entity.spendable(), true);
        assert_eq!(entity.satoshis(), 5000);
    }

    #[test]
    fn test_entity_output_property_accessors() {
        let mut entity = EntityOutput::new(None);

        entity.set_output_id(42);
        assert_eq!(entity.output_id(), 42);

        entity.set_user_id(100);
        assert_eq!(entity.user_id(), 100);

        entity.set_transaction_id(50);
        assert_eq!(entity.transaction_id(), 50);

        entity.set_basket_id(Some(10));
        assert_eq!(entity.basket_id(), Some(10));

        entity.set_spendable(true);
        assert_eq!(entity.spendable(), true);

        entity.set_change(true);
        assert_eq!(entity.change(), true);

        entity.set_vout(2);
        assert_eq!(entity.vout(), 2);

        entity.set_satoshis(10000);
        assert_eq!(entity.satoshis(), 10000);

        entity.set_output_description("My output");
        assert_eq!(entity.output_description(), "My output");

        entity.set_output_type("P2PKH");
        assert_eq!(entity.output_type(), "P2PKH");

        entity.set_provided_by(StorageProvidedBy::Storage);
        assert_eq!(entity.provided_by(), StorageProvidedBy::Storage);

        entity.set_txid(Some("txid123".to_string()));
        assert_eq!(entity.txid(), Some("txid123"));

        entity.set_spent_by(Some(99));
        assert_eq!(entity.spent_by(), Some(99));
    }

    #[test]
    fn test_entity_output_optional_fields() {
        let mut entity = EntityOutput::new(None);

        entity.set_spending_description(Some("spent on something".to_string()));
        assert_eq!(entity.spending_description(), Some("spent on something"));

        entity.set_derivation_prefix(Some("prefix".to_string()));
        assert_eq!(entity.derivation_prefix(), Some("prefix"));

        entity.set_derivation_suffix(Some("suffix".to_string()));
        assert_eq!(entity.derivation_suffix(), Some("suffix"));

        entity.set_sender_identity_key(Some("key123".to_string()));
        assert_eq!(entity.sender_identity_key(), Some("key123"));

        entity.set_custom_instructions(Some("instructions".to_string()));
        assert_eq!(entity.custom_instructions(), Some("instructions"));

        entity.set_locking_script(Some(vec![1, 2, 3]));
        assert_eq!(entity.locking_script(), Some(&vec![1, 2, 3]));

        entity.set_script_length(Some(100));
        assert_eq!(entity.script_length(), Some(100));

        entity.set_script_offset(Some(50));
        assert_eq!(entity.script_offset(), Some(50));
    }

    #[test]
    fn test_entity_output_equals_same() {
        let output = TableOutput {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_id: 1,
            user_id: 100,
            transaction_id: 50,
            basket_id: Some(10),
            spendable: true,
            change: false,
            output_description: "Test".to_string(),
            vout: 0,
            satoshis: 5000,
            provided_by: StorageProvidedBy::You,
            purpose: "payment".to_string(),
            output_type: "P2PKH".to_string(),
            txid: Some("abc".to_string()),
            spent_by: None,
            spending_description: None,
            derivation_prefix: None,
            derivation_suffix: None,
            sender_identity_key: None,
            custom_instructions: None,
            locking_script: None,
            script_length: None,
            script_offset: None,
            sequence_number: None,
        };

        let entity = EntityOutput::new(Some(output.clone()));
        assert!(entity.equals(&output, None));
    }

    #[test]
    fn test_entity_output_equals_different_satoshis() {
        let output1 = TableOutput {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_id: 1,
            user_id: 100,
            transaction_id: 50,
            basket_id: None,
            spendable: true,
            change: false,
            output_description: "Test".to_string(),
            vout: 0,
            satoshis: 5000,
            provided_by: StorageProvidedBy::You,
            purpose: "payment".to_string(),
            output_type: "P2PKH".to_string(),
            txid: None,
            spent_by: None,
            spending_description: None,
            derivation_prefix: None,
            derivation_suffix: None,
            sender_identity_key: None,
            custom_instructions: None,
            locking_script: None,
            script_length: None,
            script_offset: None,
            sequence_number: None,
        };

        let mut output2 = output1.clone();
        output2.satoshis = 10000;

        let entity = EntityOutput::new(Some(output1));
        assert!(!entity.equals(&output2, None));
    }

    #[test]
    fn test_entity_output_equals_with_locking_script() {
        let output1 = TableOutput {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_id: 1,
            user_id: 100,
            transaction_id: 50,
            basket_id: None,
            spendable: true,
            change: false,
            output_description: "Test".to_string(),
            vout: 0,
            satoshis: 5000,
            provided_by: StorageProvidedBy::You,
            purpose: "payment".to_string(),
            output_type: "P2PKH".to_string(),
            txid: None,
            spent_by: None,
            spending_description: None,
            derivation_prefix: None,
            derivation_suffix: None,
            sender_identity_key: None,
            custom_instructions: None,
            locking_script: Some(vec![1, 2, 3]),
            script_length: None,
            script_offset: None,
            sequence_number: None,
        };

        let entity = EntityOutput::new(Some(output1.clone()));
        assert!(entity.equals(&output1, None));

        let mut output2 = output1.clone();
        output2.locking_script = Some(vec![4, 5, 6]);
        assert!(!entity.equals(&output2, None));

        let mut output3 = output1;
        output3.locking_script = None;
        assert!(!entity.equals(&output3, None));
    }

    #[test]
    fn test_entity_output_entity_name() {
        let entity = EntityOutput::new(None);
        assert_eq!(entity.entity_name(), "output");
    }

    #[test]
    fn test_entity_output_entity_table() {
        let entity = EntityOutput::new(None);
        assert_eq!(entity.entity_table(), "outputs");
    }

    #[test]
    fn test_entity_output_id_methods() {
        let mut entity = EntityOutput::new(None);
        
        assert_eq!(entity.id(), 0);
        entity.set_id(999);
        assert_eq!(entity.id(), 999);
        assert_eq!(entity.output_id(), 999);
    }

    #[test]
    fn test_entity_output_clone() {
        let output = TableOutput {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            output_id: 1,
            user_id: 100,
            transaction_id: 50,
            basket_id: Some(10),
            spendable: true,
            change: false,
            output_description: "Test".to_string(),
            vout: 0,
            satoshis: 5000,
            provided_by: StorageProvidedBy::You,
            purpose: "payment".to_string(),
            output_type: "P2PKH".to_string(),
            txid: None,
            spent_by: None,
            spending_description: None,
            derivation_prefix: None,
            derivation_suffix: None,
            sender_identity_key: None,
            custom_instructions: None,
            locking_script: None,
            script_length: None,
            script_offset: None,
            sequence_number: None,
        };

        let entity1 = EntityOutput::new(Some(output));
        let entity2 = entity1.clone();
        
        assert_eq!(entity1, entity2);
        assert_eq!(entity2.output_id(), 1);
    }
}
