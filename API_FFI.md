# FFI API for Metanet Native Client

Complete C API specification for `wallet-toolbox-rs` native integration.

## Core Functions

All functions return `WalletResultCode` and use JSON for complex arguments/results.

### Lifecycle
- `wallet_create()` - Create wallet with SQLite storage
- `wallet_destroy()` - Free resources
- `wallet_get_last_error()` - Get error message

### Actions
- `wallet_create_action()` - Create transaction
- `wallet_sign_action()` - Sign transaction
- `wallet_internalize_action()` - Process incoming TX
- `wallet_abort_action()` - Cancel action

### Queries
- `wallet_list_actions()` - List transactions
- `wallet_list_outputs()` - List UTXOs
- `wallet_list_certificates()` - List certificates

### Certificates
- `wallet_acquire_certificate()` - Get new certificate
- `wallet_prove_certificate()` - Prove ownership
- `wallet_relinquish_certificate()` - Revoke certificate

### Keys & Crypto
- `wallet_get_public_key()` - Derive public key
- `wallet_create_signature()` - Sign data
- `wallet_verify_signature()` - Verify signature
- `wallet_encrypt()` / `wallet_decrypt()` - Encryption

See `crates/wallet-client/include/wallet_ffi.h` for complete definitions.
