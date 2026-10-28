//! Core types for StellarFoundry DevKit.
//!
//! This crate is intentionally free of I/O and Stellar-specific dependencies so
//! that every other crate can depend on it without pulling in a large
//! dependency graph. It provides the shared error type, network profiles, and
//! output format selection.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// The CLI binary name.
pub const TOOL_NAME: &str = "stellar-foundry";

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Errors produced by DevKit crates.
#[derive(Debug, thiserror::Error)]
pub enum DevkitError {
    /// Input could not be decoded.
    #[error("decode error: {0}")]
    Decode(String),
    /// A value could not be encoded.
    #[error("encode error: {0}")]
    Encode(String),
    /// The caller supplied invalid input.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// Configuration was invalid.
    #[error("configuration error: {0}")]
    Config(String),
    /// An I/O operation failed.
    #[error("I/O error: {0}")]
    Io(String),
    /// The RPC endpoint rate limited the request.
    #[error("rate limited: {message}")]
    RateLimited {
        /// What happened, including any backoff decision.
        message: String,
        /// The delay the endpoint asked for via `Retry-After`, when provided.
        retry_after: Option<Duration>,
    },
}

impl DevkitError {
    /// Builds a decode error from any displayable cause.
    pub fn decode(cause: impl std::fmt::Display) -> Self {
        DevkitError::Decode(cause.to_string())
    }

    /// Builds an invalid-input error.
    pub fn invalid(cause: impl std::fmt::Display) -> Self {
        DevkitError::InvalidInput(cause.to_string())
    }

    /// Builds a rate-limit error with an optional `Retry-After` delay.
    pub fn rate_limited(message: impl Into<String>, retry_after: Option<Duration>) -> Self {
        DevkitError::RateLimited {
            message: message.into(),
            retry_after,
        }
    }

    /// The `Retry-After` delay carried by a rate-limit error, if any.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            DevkitError::RateLimited { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

/// Output format selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Human-readable text.
    Terminal,
    /// Machine-readable JSON.
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "terminal" | "text" => Ok(OutputFormat::Terminal),
            "json" => Ok(OutputFormat::Json),
            other => Err(format!("unknown output format: {other}")),
        }
    }
}

/// A Stellar network profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    /// Stellar public network (mainnet).
    Public,
    /// Stellar testnet.
    Testnet,
    /// Stellar futurenet.
    Futurenet,
    /// A user-supplied network.
    Custom {
        /// Human-readable name.
        name: String,
        /// Network passphrase.
        passphrase: String,
        /// Optional RPC endpoint.
        rpc_url: Option<String>,
    },
}

impl Network {
    /// Short lowercase name.
    pub fn name(&self) -> &str {
        match self {
            Network::Public => "public",
            Network::Testnet => "testnet",
            Network::Futurenet => "futurenet",
            Network::Custom { name, .. } => name,
        }
    }

    /// The network passphrase used for signing and hashing.
    pub fn passphrase(&self) -> &str {
        match self {
            Network::Public => "Public Global Stellar Network ; September 2015",
            Network::Testnet => "Test SDF Network ; September 2015",
            Network::Futurenet => "Test SDF Future Network ; October 2022",
            Network::Custom { passphrase, .. } => passphrase,
        }
    }

    /// A documented default RPC endpoint, if one is known.
    ///
    /// Mainnet deliberately has no default: public RPC providers change, so the
    /// endpoint must be configured explicitly rather than hardcoded.
    pub fn default_rpc_url(&self) -> Option<&str> {
        match self {
            Network::Testnet => Some("https://soroban-testnet.stellar.org"),
            Network::Public | Network::Futurenet => None,
            Network::Custom { rpc_url, .. } => rpc_url.as_deref(),
        }
    }

    /// The three well-known networks.
    pub fn known() -> [Network; 3] {
        [Network::Public, Network::Testnet, Network::Futurenet]
    }
}

impl std::str::FromStr for Network {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "public" | "mainnet" => Ok(Network::Public),
            "testnet" => Ok(Network::Testnet),
            "futurenet" => Ok(Network::Futurenet),
            other => Err(format!("unknown network: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_networks_have_distinct_passphrases() {
        let networks = Network::known();
        assert_eq!(networks.len(), 3);
        assert_ne!(networks[0].passphrase(), networks[1].passphrase());
        assert!(networks[1].passphrase().contains("Test"));
    }

    #[test]
    fn mainnet_has_no_hardcoded_rpc_endpoint() {
        assert_eq!(Network::Public.default_rpc_url(), None);
        assert!(Network::Testnet.default_rpc_url().is_some());
    }

    #[test]
    fn parses_network_and_output_names() {
        assert_eq!("mainnet".parse::<Network>().unwrap(), Network::Public);
        assert!("nope".parse::<Network>().is_err());
        assert_eq!("JSON".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
    }

    #[test]
    fn devkit_error_from_display() {
        let err = DevkitError::decode("bad base64");
        assert!(err.to_string().contains("bad base64"));
    }

    #[test]
    fn rate_limit_errors_preserve_retry_after() {
        let err = DevkitError::rate_limited("HTTP 429", Some(Duration::from_secs(5)));
        assert!(err.to_string().contains("rate limited"));
        assert_eq!(err.retry_after(), Some(Duration::from_secs(5)));
        assert_eq!(DevkitError::decode("x").retry_after(), None);
    }
}
