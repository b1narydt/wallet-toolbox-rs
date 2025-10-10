//! Blockchain Query Operations
//!
//! Query blockchain state (height, headers, network, version).
//! Reference: wallet-toolbox SDK blockchain query methods

use crate::sdk::{GetHeaderArgs, GetHeaderResult, GetHeightResult, GetNetworkResult, GetVersionResult, WalletError, WalletResult};

/// Get current blockchain height
///
/// Reference: TypeScript `getHeight()` in SDK
pub async fn get_height() -> WalletResult<GetHeightResult> {
    // TODO: Query actual chain tracker service
    Err(WalletError::not_implemented("getHeight"))
}

/// Get block header for a specific height
///
/// Reference: TypeScript `getHeaderForHeight()` in SDK
pub async fn get_header_for_height(args: &GetHeaderArgs) -> WalletResult<GetHeaderResult> {
    let _ = args;
    // TODO: Query actual chain tracker service
    Err(WalletError::not_implemented("getHeaderForHeight"))
}

/// Get current network ("main" or "test")
///
/// Reference: TypeScript `getNetwork()` in SDK
pub async fn get_network() -> WalletResult<GetNetworkResult> {
    // TODO: Return actual configured network
    Ok(GetNetworkResult {
        network: "main".to_string(),
    })
}

/// Get wallet version
///
/// Reference: TypeScript `getVersion()` in SDK
pub async fn get_version() -> WalletResult<GetVersionResult> {
    Ok(GetVersionResult {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_network() {
        let result = get_network().await.unwrap();
        assert!(!result.network.is_empty());
    }

    #[tokio::test]
    async fn test_get_version() {
        let result = get_version().await.unwrap();
        assert!(!result.version.is_empty());
    }

    #[tokio::test]
    async fn test_get_height_not_implemented() {
        let result = get_height().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_header_not_implemented() {
        let args = GetHeaderArgs { height: 800000 };
        let result = get_header_for_height(&args).await;
        assert!(result.is_err());
    }
}
