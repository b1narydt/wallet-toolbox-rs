//! Permission Manager Constants
//!
//! **Reference**: TypeScript `src/WalletPermissionsManager.ts` lines 205-210
//!
//! Constants for permission token storage and management

use super::types::PermissionType;

/// A map from each permission type to a special "admin basket" name used for storing
/// the tokens.
///
/// Reference: TS BASKET_MAP (WalletPermissionsManager.ts lines 205-210)
///
/// The tokens themselves are unspent transaction outputs (UTXOs) with a
/// specialized PushDrop script that references the originator, expiry, etc.
///
/// ## Admin Baskets
/// - **protocol**: Stores DPACP (Domain Protocol Access Control Protocol) tokens
/// - **basket**: Stores DBAP (Domain Basket Access Protocol) tokens
/// - **certificate**: Stores DCAP (Domain Certificate Access Protocol) tokens
/// - **spending**: Stores DSAP (Domain Spending Authorization Protocol) tokens
pub fn get_admin_basket_name(permission_type: PermissionType) -> &'static str {
    match permission_type {
        // TS line 206
        PermissionType::Protocol => "admin protocol-permission",
        // TS line 207
        PermissionType::Basket => "admin basket-access",
        // TS line 208
        PermissionType::Certificate => "admin certificate-access",
        // TS line 209
        PermissionType::Spending => "admin spending-authorization",
    }
}

/// Default token expiration time (30 days in seconds)
///
/// Reference: Based on typical permission token expiration patterns
pub const DEFAULT_TOKEN_EXPIRY_SECONDS: i64 = 30 * 24 * 60 * 60; // 30 days

/// Minimum satoshis for a permission token output
///
/// Reference: Based on dust limit and typical permission token values
pub const MIN_PERMISSION_TOKEN_SATOSHIS: i64 = 1;

/// Protocol IDs for the four permission types
///
/// Reference: TS comments describing DPACP, DBAP, DCAP, DSAP (lines 92-95)
pub mod protocol_ids {
    /// Domain Protocol Access Control Protocol
    pub const DPACP: &str = "DPACP";
    
    /// Domain Basket Access Protocol
    pub const DBAP: &str = "DBAP";
    
    /// Domain Certificate Access Protocol
    pub const DCAP: &str = "DCAP";
    
    /// Domain Spending Authorization Protocol
    pub const DSAP: &str = "DSAP";
}

/// Security level names for display
///
/// Reference: TS SecurityLevels usage throughout the file
pub mod security_level_names {
    /// Security level 0 - Public operations
    pub const LEVEL_0_PUBLIC: &str = "Public";
    
    /// Security level 1 - Shared operations
    pub const LEVEL_1_SHARED: &str = "Shared";
    
    /// Security level 2 - Private operations
    pub const LEVEL_2_PRIVATE: &str = "Private";
}

/// Special counterparty values
///
/// Reference: TS counterparty field description (line 108)
pub mod counterparty {
    /// Counterparty is the user themselves
    pub const SELF: &str = "self";
    
    /// Counterparty can be anyone
    pub const ANYONE: &str = "anyone";
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basket_names() {
        // Verify exact TypeScript parity (TS lines 206-209)
        assert_eq!(
            get_admin_basket_name(PermissionType::Protocol),
            "admin protocol-permission"
        );
        assert_eq!(
            get_admin_basket_name(PermissionType::Basket),
            "admin basket-access"
        );
        assert_eq!(
            get_admin_basket_name(PermissionType::Certificate),
            "admin certificate-access"
        );
        assert_eq!(
            get_admin_basket_name(PermissionType::Spending),
            "admin spending-authorization"
        );
    }
    
    #[test]
    fn test_protocol_ids() {
        assert_eq!(protocol_ids::DPACP, "DPACP");
        assert_eq!(protocol_ids::DBAP, "DBAP");
        assert_eq!(protocol_ids::DCAP, "DCAP");
        assert_eq!(protocol_ids::DSAP, "DSAP");
    }
    
    #[test]
    fn test_counterparty_constants() {
        assert_eq!(counterparty::SELF, "self");
        assert_eq!(counterparty::ANYONE, "anyone");
    }
    
    #[test]
    fn test_token_constants() {
        assert_eq!(DEFAULT_TOKEN_EXPIRY_SECONDS, 30 * 24 * 60 * 60);
        assert_eq!(MIN_PERMISSION_TOKEN_SATOSHIS, 1);
    }
}
