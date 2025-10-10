// Signer methods module
//
// Reference: TypeScript src/signer/methods/

pub mod sign_message;
pub mod sign_transaction;
pub mod build_signable_transaction;
pub mod complete_signed_transaction;
pub mod acquire_direct_certificate;
pub mod prove_certificate;

// Re-exports
pub use build_signable_transaction::{
    build_signable_transaction,
    BuildSignableTransactionResult,
    PendingStorageInput,
};

pub use complete_signed_transaction::{
    complete_signed_transaction,
    verify_unlock_scripts,
    PendingSignAction,
    SignActionSpend,
};

pub use acquire_direct_certificate::{
    acquire_direct_certificate,
    AcquireCertificateResult,
    ValidAcquireDirectCertificateArgs,
    NewCertificate,
    CertificateField,
};

pub use prove_certificate::{
    prove_certificate,
    ProveCertificateResult,
    ValidProveCertificateArgs,
    ListCertificatesArgs,
    PartialCertificate,
    StorageCertificate,
};
