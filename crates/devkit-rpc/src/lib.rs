//! Stellar RPC protocol layer.
//!
//! This crate models the Stellar RPC JSON-RPC protocol and separates it from
//! transport. It does **not** perform network I/O: a [`Transport`] is injected.
//! A [`MockTransport`] makes RPC-based tooling testable and deterministic, and
//! a production transport (wrapping the official `stellar-rpc-client`) is
//! future work tracked as an issue.
//!
//! Security posture:
//! * Endpoints are validated before use; plaintext `http` is only allowed for
//!   loopback hosts, credentials in URLs are rejected, and unknown schemes are
//!   refused.
//! * Responses are parsed with structured errors; malformed responses never
//!   panic.

use std::cell::RefCell;
use std::collections::VecDeque;

use devkit_core::DevkitError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use url::Url;

/// A validated Stellar RPC endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    url: Url,
}

impl Endpoint {
    /// Parses and validates an endpoint URL.
    ///
    /// Rules:
    /// * `https` is always allowed.
    /// * `http` is allowed only for loopback hosts (`localhost`, `127.0.0.1`,
    ///   `::1`), to avoid sending traffic in plaintext to a remote host.
    /// * Any other scheme is rejected.
    /// * Embedded credentials are rejected.
    pub fn parse(input: &str) -> Result<Self, DevkitError> {
        let url = Url::parse(input.trim())
            .map_err(|e| DevkitError::invalid(format!("invalid endpoint URL: {e}")))?;

        match url.scheme() {
            "https" => {}
            "http" => {
                if !is_loopback(&url) {
                    return Err(DevkitError::invalid(
                        "plaintext http is only allowed for loopback hosts; use https",
                    ));
                }
            }
            other => {
                return Err(DevkitError::invalid(format!(
                    "unsupported endpoint scheme: {other}"
                )))
            }
        }

        if !url.username().is_empty() || url.password().is_some() {
            return Err(DevkitError::invalid(
                "credentials embedded in the endpoint URL are not allowed",
            ));
        }

        if url.host_str().is_none() {
            return Err(DevkitError::invalid("endpoint URL has no host"));
        }

        Ok(Endpoint { url })
    }

    /// The endpoint URL as a string.
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }

    /// Whether the endpoint points at a loopback host.
    pub fn is_loopback(&self) -> bool {
        is_loopback(&self.url)
    }
}

fn is_loopback(url: &Url) -> bool {
    match url.host_str() {
        Some("localhost") | Some("127.0.0.1") | Some("::1") | Some("[::1]") => true,
        Some(host) => host
            .parse::<std::net::IpAddr>()
            .map(|ip| ip.is_loopback())
            .unwrap_or(false),
        None => false,
    }
}

/// A JSON-RPC transport. Implementations may perform I/O; the rest of the crate
/// depends only on this trait.
pub trait Transport {
    /// Calls a JSON-RPC method with parameters and returns the `result` value.
    fn call(&self, method: &str, params: Value) -> Result<Value, DevkitError>;
}

/// A deterministic, in-memory transport for tests and fixtures.
#[derive(Default)]
pub struct MockTransport {
    responses: RefCell<VecDeque<Result<Value, DevkitError>>>,
    calls: RefCell<Vec<(String, Value)>>,
}

impl MockTransport {
    /// Creates a mock that returns the given responses in order.
    pub fn with_responses(responses: Vec<Value>) -> Self {
        let transport = MockTransport::default();
        for response in responses {
            transport.push_ok(response);
        }
        transport
    }

    /// Queues a successful response.
    pub fn push_ok(&self, response: Value) {
        self.responses.borrow_mut().push_back(Ok(response));
    }

    /// Queues a transport failure.
    pub fn push_err(&self, message: impl Into<String>) {
        self.responses
            .borrow_mut()
            .push_back(Err(DevkitError::Io(message.into())));
    }

    /// Returns the recorded `(method, params)` calls.
    pub fn calls(&self) -> Vec<(String, Value)> {
        self.calls.borrow().clone()
    }
}

impl Transport for MockTransport {
    fn call(&self, method: &str, params: Value) -> Result<Value, DevkitError> {
        self.calls
            .borrow_mut()
            .push((method.to_string(), params.clone()));
        self.responses.borrow_mut().pop_front().unwrap_or_else(|| {
            Err(DevkitError::invalid(
                "mock transport has no queued response",
            ))
        })
    }
}

/// Well-known result of `getHealth`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// Overall status, for example `healthy`.
    pub status: String,
    /// Latest ledger known to the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_ledger: Option<u32>,
    /// Oldest ledger retained by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_ledger: Option<u32>,
    /// Size of the retention window.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ledger_retention_window: Option<u32>,
}

/// Result of `getNetwork`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    /// The network passphrase.
    pub passphrase: String,
    /// Protocol version, when reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<u32>,
}

/// Result of `getLatestLedger`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestLedger {
    /// Ledger hash identifier.
    pub id: String,
    /// Ledger sequence number.
    pub sequence: u32,
    /// Protocol version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<u32>,
    /// Close time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<u64>,
}

/// A typed RPC client over an injected [`Transport`].
pub struct Client<T: Transport> {
    transport: T,
    retry: RetryPolicy,
}

impl<T: Transport> Client<T> {
    /// Creates a client with a default retry policy.
    pub fn new(transport: T) -> Self {
        Client {
            transport,
            retry: RetryPolicy::default(),
        }
    }

    /// Sets the retry policy.
    pub fn with_retry(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    /// Calls a method, retrying retryable transport errors.
    fn call(&self, method: &str, params: Value) -> Result<Value, DevkitError> {
        let mut attempt = 0;
        loop {
            match self.transport.call(method, params.clone()) {
                Ok(value) => return Ok(value),
                Err(error) => {
                    if self.retry.should_retry(attempt, &error) {
                        attempt += 1;
                        continue;
                    }
                    return Err(error);
                }
            }
        }
    }

    /// Fetches node health.
    pub fn health(&self) -> Result<Health, DevkitError> {
        let value = self.call("getHealth", json!({}))?;
        serde_json::from_value(value).map_err(DevkitError::decode)
    }

    /// Fetches network information.
    pub fn network(&self) -> Result<NetworkInfo, DevkitError> {
        let value = self.call("getNetwork", json!({}))?;
        serde_json::from_value(value).map_err(DevkitError::decode)
    }

    /// Fetches the latest ledger.
    pub fn latest_ledger(&self) -> Result<LatestLedger, DevkitError> {
        let value = self.call("getLatestLedger", json!({}))?;
        serde_json::from_value(value).map_err(DevkitError::decode)
    }

    /// Fetches fee statistics (returned raw until modeled).
    pub fn fee_stats(&self) -> Result<Value, DevkitError> {
        self.call("getFeeStats", json!({}))
    }

    /// Fetches contract events from a start ledger.
    ///
    /// Event filters are passed through as-is; their schema is documented in the
    /// Stellar RPC reference.
    pub fn get_events(&self, start_ledger: u32, filters: Value) -> Result<Value, DevkitError> {
        self.call(
            "getEvents",
            json!({ "startLedger": start_ledger, "filters": filters }),
        )
    }
}

/// Determines whether failed calls should be retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Maximum number of retries (not counting the first attempt).
    pub max_retries: u32,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy { max_retries: 2 }
    }
}

impl RetryPolicy {
    /// Creates a policy with `max_retries`.
    pub fn new(max_retries: u32) -> Self {
        RetryPolicy { max_retries }
    }

    /// Returns true when another attempt should be made.
    ///
    /// Only transport-level failures are retryable. Decode and input errors are
    /// deterministic and are never retried.
    pub fn should_retry(&self, attempts_made: u32, error: &DevkitError) -> bool {
        attempts_made < self.max_retries && matches!(error, DevkitError::Io(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_https_endpoints() {
        let endpoint = Endpoint::parse("https://soroban-testnet.stellar.org").unwrap();
        assert!(!endpoint.is_loopback());
        assert!(endpoint.as_str().starts_with("https://"));
    }

    #[test]
    fn rejects_plaintext_http_for_remote_hosts() {
        assert!(Endpoint::parse("http://example.com").is_err());
    }

    #[test]
    fn allows_plaintext_http_for_loopback() {
        assert!(Endpoint::parse("http://localhost:8000").is_ok());
        assert!(Endpoint::parse("http://127.0.0.1:8000").is_ok());
    }

    #[test]
    fn rejects_unsupported_schemes() {
        assert!(Endpoint::parse("ftp://example.com").is_err());
        assert!(Endpoint::parse("file:///etc/passwd").is_err());
    }

    #[test]
    fn rejects_credentials_in_url() {
        assert!(Endpoint::parse("https://user:pass@example.com").is_err());
    }

    #[test]
    fn parses_health_response() {
        let transport = MockTransport::with_responses(vec![json!({
            "status": "healthy",
            "latestLedger": 123,
            "oldestLedger": 100,
            "ledgerRetentionWindow": 23
        })]);
        let client = Client::new(transport);
        let health = client.health().unwrap();
        assert_eq!(health.status, "healthy");
        assert_eq!(health.latest_ledger, Some(123));
    }

    #[test]
    fn parses_minimal_health_response() {
        // Unknown fields must be ignored and optional fields must be optional.
        let transport = MockTransport::with_responses(vec![json!({
            "status": "healthy",
            "extraUnknownField": {"ignored": true}
        })]);
        let health = Client::new(transport).health().unwrap();
        assert_eq!(health.status, "healthy");
        assert_eq!(health.latest_ledger, None);
    }

    #[test]
    fn parses_network_and_latest_ledger() {
        let transport = MockTransport::with_responses(vec![
            json!({"passphrase": "Test SDF Network ; September 2015", "protocolVersion": 23}),
            json!({"id": "abc", "sequence": 42, "protocolVersion": 23, "closeTime": 1000}),
        ]);
        let client = Client::new(transport);
        assert_eq!(client.network().unwrap().protocol_version, Some(23));
        assert_eq!(client.latest_ledger().unwrap().sequence, 42);
    }

    #[test]
    fn malformed_response_is_a_structured_error() {
        let transport = MockTransport::with_responses(vec![json!({"unexpected": true})]);
        let err = Client::new(transport).health().unwrap_err();
        assert!(matches!(err, DevkitError::Decode(_)));
    }

    #[test]
    fn records_method_calls() {
        let transport = MockTransport::with_responses(vec![json!({"status": "healthy"})]);
        let client = Client::new(transport);
        let _ = client.health();
        // The transport is owned by the client; re-create to inspect calls in a
        // more typical pattern. Here we only assert the call succeeded.
        assert!(client.latest_ledger().is_err()); // no second response queued
    }

    #[test]
    fn mock_records_calls() {
        let transport = MockTransport::with_responses(vec![json!({"status": "healthy"})]);
        let _ = transport.call("getHealth", json!({})).unwrap();
        let calls = transport.calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "getHealth");
    }

    #[test]
    fn retry_policy_retries_only_io_errors() {
        let policy = RetryPolicy::new(2);
        assert!(policy.should_retry(0, &DevkitError::Io("timeout".into())));
        assert!(policy.should_retry(1, &DevkitError::Io("timeout".into())));
        assert!(!policy.should_retry(2, &DevkitError::Io("timeout".into())));
        assert!(!policy.should_retry(0, &DevkitError::decode("bad")));
    }

    #[test]
    fn client_retries_transport_failures_then_succeeds() {
        let transport = MockTransport::default();
        transport.push_err("temporary");
        transport.push_ok(json!({"status": "healthy"}));
        let client = Client::new(transport).with_retry(RetryPolicy::new(1));
        assert_eq!(client.health().unwrap().status, "healthy");
    }
}
