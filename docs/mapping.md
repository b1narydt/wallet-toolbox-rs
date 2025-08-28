# Wallet-Toolbox TypeScript → Rust Mapping

This document maps the TypeScript source modules to their Rust crate/module paths in the `wallet-toolbox-rs` workspace. It serves as the authoritative guide for translation and API parity.

## Entrypoints
- src/index.ts → wallet-core (re-exports consolidated in crate root as needed)
- src/index.all.ts → wallet-core::index_all (internal consolidation)
- src/index.client.ts → wallet-client (re-exports from wallet-core) ✓ implemented
- src/index.mobile.ts → wallet-mobile (re-exports from wallet-core) ✓ implemented

## Core Types and Managers (wallet-core)
- src/Wallet.ts → wallet_core::wallet::Wallet
- src/SimpleWalletManager.ts → wallet_core::managers::SimpleWalletManager
- src/CWIStyleWalletManager.ts → wallet_core::managers::CWIStyleWalletManager
- src/WalletAuthenticationManager.ts → wallet_core::managers::WalletAuthenticationManager
- src/WalletPermissionsManager.ts → wallet_core::managers::WalletPermissionsManager
- src/WalletSettingsManager.ts → wallet_core::managers::WalletSettingsManager
- src/Setup.ts → wallet_core::setup::Setup
- src/SetupClient.ts → wallet_core::setup::SetupClient (re-exported in wallet-client)
- src/SetupWallet.ts → wallet_core::setup::SetupWallet (re-exported in wallet-client)

## Signer (wallet-core)
- src/signer/WalletSigner.ts → wallet_core::signer::WalletSigner
- src/signer/methods/* → wallet_core::signer::methods::{acquire_direct_certificate, build_signable_transaction, complete_signed_transaction, create_action, internalize_action, prove_certificate, sign_action}

## SDK (wallet-core)
- src/sdk/index.ts → wallet_core::sdk (module)
- src/sdk/WalletServices.interfaces.ts → wallet_core::sdk::wallet_services (Rust traits)
- src/sdk/WalletSigner.interfaces.ts → wallet_core::sdk::wallet_signer (Rust traits)
- src/sdk/WalletStorage.interfaces.ts → wallet_core::sdk::wallet_storage (Rust traits referencing wallet-storage)
- src/sdk/WERR_errors.ts → wallet_core::sdk::errors::werr
- src/sdk/WalletError.ts → wallet_core::sdk::errors::wallet_error
- src/sdk/types.ts → wallet_core::sdk::types
- src/sdk/validationHelpers.ts → wallet_core::sdk::validation
- src/sdk/CertOpsWallet.ts → wallet_core::sdk::cert_ops_wallet
- src/sdk/PrivilegedKeyManager.ts → wallet_core::sdk::PrivilegedKeyManager (re-exported in client/mobile)

## Services (wallet-core)
- src/services/Services.ts → wallet_core::services::Services
- src/services/ServiceCollection.ts → wallet_core::services::ServiceCollection
- src/services/index.ts → wallet_core::services (mod root)
- src/services/createDefaultWalletServicesOptions.ts → wallet_core::services::defaults
- src/services/providers/ARC.ts → wallet_core::services::providers::arc
- src/services/providers/Bitails.ts → wallet_core::services::providers::bitails
- src/services/providers/WhatsOnChain.ts → wallet_core::services::providers::woc
- src/services/providers/SdkWhatsOnChain.ts → wallet_core::services::providers::sdk_woc
- src/services/providers/echangeRates.ts → wallet_core::services::providers::exchange_rates
- src/services/providers/getBeefForTxid.ts → wallet_core::services::providers::get_beef_for_txid
- src/services/chaintracker/* → wallet_core::services::chaintracker::{bh_service_client, chaintracks_chain_tracker, chaintracks/*, util/*, index}

## Monitor (wallet-monitor crate)
- src/monitor/Monitor.ts → wallet_monitor::Monitor
- src/monitor/MonitorDaemon.ts → wallet_monitor::MonitorDaemon
- src/monitor/tasks/*.ts → wallet_monitor::tasks::{TaskCheckForProofs, TaskCheckNoSends, TaskClock, TaskFailAbandoned, TaskMonitorCallHistory, TaskNewHeader, TaskPurge, TaskReviewStatus, TaskSendWaiting, TaskSyncWhenIdle, TaskUnFail, WalletMonitorTask}

## WAB Client (wallet-wab-client crate)
- src/wab-client/WABClient.ts → wallet_wab_client::WABClient
- src/wab-client/auth-method-interactors/AuthMethodInteractor.ts → wallet_wab_client::auth_method_interactors::AuthMethodInteractor (trait)
- src/wab-client/auth-method-interactors/TwilioPhoneInteractor.ts → wallet_wab_client::auth_method_interactors::TwilioPhoneInteractor
- src/wab-client/auth-method-interactors/PersonaIDInteractor.ts → wallet_wab_client::auth_method_interactors::PersonaIDInteractor

Note: wallet-client and wallet-mobile re-export these via wallet-core stubs for API parity in Rust.

## Storage (wallet-storage crate + backend crates)
Shared interfaces and models (wallet-storage):
- src/storage/StorageReader.ts → wallet_storage::{traits::StorageReader}
- src/storage/StorageReaderWriter.ts → wallet_storage::{traits::StorageReaderWriter}
- src/storage/StorageSyncReader.ts → wallet_storage::{traits::StorageSyncReader}
- src/storage/StorageProvider.ts → wallet_storage::{traits::WalletStorageProvider} + associated options/models
- src/storage/WalletStorageManager.ts → wallet_storage::manager::WalletStorageManager
- src/storage/index.all.ts → wallet_storage::index_all (internal consolidation)
- src/storage/index.client.ts → wallet_core::storage::index_client (re-exported in wallet-client)
- src/storage/index.mobile.ts → wallet_core::storage::index_mobile (re-exported in wallet-mobile)
- src/storage/methods/* → wallet_storage::methods::{attempt_to_post_reqs_to_network, create_action, generate_change, get_beef_for_transaction, internalize_action, list_certificates, list_actions_spec_op, list_outputs_spec_op, process_action}
- src/storage/schema/entities/* → wallet_storage::schema::entities::{...}
- src/storage/schema/primitives/* → wallet_storage::schema::primitives::{...}
- src/storage/schema/tables/* → wallet_storage::schema::tables::{...}
- src/storage/sync/* → wallet_storage::sync::{...}

Backends:
- src/storage/IndexedDB/* → wallet-storage-indexeddb crate (WASM, feature = "indexeddb")
- src/storage/Knex/* → maps conceptually to SQL builders; in Rust, we consolidate into specific backends
- src/storage/SQLite/* → wallet-storage-sqlite crate (feature = "sqlite")
- src/storage/MySQL/* → wallet-storage-mysql crate (feature = "mysql")
- src/storage/MySQLDojo/* → wallet-storage-mysql crate (dojovariant modules)

## Utility (wallet-core)
- src/utility/index.client.ts → wallet_core::utility::index_client (re-exported to client/mobile)
- src/utility/index.all.ts → wallet_core::utility::index_all
- src/utility/Format.ts → wallet_core::utility::format
- src/utility/ScriptTemplateBRC29.ts → wallet_core::utility::script_template_brc29
- src/utility/identityUtils.ts → wallet_core::utility::identity_utils
- src/utility/parseTxScriptOffsets.ts → wallet_core::utility::parse_tx_script_offsets
- src/utility/stampLog.ts → wallet_core::utility::stamp_log
- src/utility/tscProofToMerklePath.ts → wallet_core::utility::tsc_proof_to_merkle_path
- src/utility/utilityHelpers.ts → wallet_core::utility::helpers
- src/utility/utilityHelpers.buffer.ts → wallet_core::utility::helpers_buffer
- src/utility/utilityHelpers.noBuffer.ts → wallet_core::utility::helpers_no_buffer

## Web (wallet-web crate)
- TS server/middleware analogs to be built with axum; endpoints derived from usages within WAB client auth and services providers.

## Tests Mapping
- TS Jest tests under src/**/__tests/** map to Rust unit/integration tests under the corresponding crate:
  - wallet-core/tests/*
  - wallet-storage/tests/*
  - wallet-monitor/tests/*
  - wallet-wab-client/tests/*
  - services-specific tests under wallet-core or separate crates as appropriate.

## Status
- wallet-client and wallet-mobile re-export stubs: ✓ done
- wallet-storage traits/errors placeholders: ✓ done
- Full TS inventory captured for monitor, sdk, services, signer, storage, utility, wab-client: ✓ done
- Next: generate Rust stubs/signatures per mapping and start translating core modules (Wallet, Managers, SDK interfaces)
