use crate::StorageError;

pub struct ListCertificatesResult; // TODO: refine shape

pub fn list_certificates() -> Result<ListCertificatesResult, StorageError> {
    Err(StorageError::NotImplemented("list_certificates"))
}
