use crate::{Beef, StorageError, Transaction};

pub fn get_beef_for_transaction(_tx: &Transaction) -> Result<Beef, StorageError> {
    Err(StorageError::NotImplemented("get_beef_for_transaction"))
}
