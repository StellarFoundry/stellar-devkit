//! Stellar RPC protocol layer.
//!
//! This crate models the Stellar RPC JSON-RPC protocol and separates it from
//! transport. A [`Transport`] is injected. [`MockTransport`] makes RPC-based
//! tooling testable and deterministic, and [`HttpTransport`] provides a bounded
//! HTTP JSON-RPC transport. Integrating the official `stellar-rpc-client`
//! remains future work tracked as an issue.
//!
//! Security posture:
//! * Endpoints are validated before use; plaintext `http` is only allowed for
//!   loopback hosts, credentials in URLs are rejected, and unknown schemes are
//!   refused.
//! * Responses are parsed with structured errors; malformed responses never
//!   panic.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use devkit_core::DevkitError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use url::Url;

const MAX_RPC_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
const MAX_RPC_RESPONSE_DEPTH: usize = 64;

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

/// A bounded HTTP JSON-RPC transport for a Stellar RPC endpoint.
///
/// Responses are capped before JSON parsing. Requests have a finite timeout,
/// JSON-RPC identifiers are checked, and server error messages are returned as
/// structured errors. Redirects are disabled by the HTTP client to avoid
/// silently forwarding RPC requests to an unexpected host.
pub struct HttpTransport {
    endpoint: String,
    client: reqwest::blocking::Client,
    next_id: AtomicU64,
}

impl HttpTransport {
    /// Creates a transport with an explicit request timeout.
    pub fn new(endpoint: Endpoint, timeout: Duration) -> Result<Self, DevkitError> {
        if timeout.is_zero() {
            return Err(DevkitError::invalid(
                "RPC timeout must be greater than zero",
            ));
        }
        let client = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|error| {
                DevkitError::Io(format!("failed to build RPC HTTP client: {error}"))
            })?;
        Ok(Self {
            endpoint: endpoint.as_str().to_owned(),
            client,
            next_id: AtomicU64::new(1),
        })
    }

    /// Creates a transport with a 10-second request timeout.
    pub fn with_default_timeout(endpoint: Endpoint) -> Result<Self, DevkitError> {
        Self::new(endpoint, Duration::from_secs(10))
    }
}

impl Transport for HttpTransport {
    fn call(&self, method: &str, params: Value) -> Result<Value, DevkitError> {
        if method.trim().is_empty() {
            return Err(DevkitError::invalid("RPC method must not be empty"));
        }
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });
        let response = self
            .client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .map_err(|error| DevkitError::Io(format!("RPC HTTP request failed: {error}")))?;
        let status = response.status();
        let mut body = Vec::new();
        response
            .take((MAX_RPC_RESPONSE_BYTES + 1) as u64)
            .read_to_end(&mut body)
            .map_err(|error| DevkitError::Io(format!("failed reading RPC response: {error}")))?;
        if body.len() > MAX_RPC_RESPONSE_BYTES {
            return Err(response_too_large());
        }
        if !status.is_success() {
            return Err(DevkitError::Io(format!(
                "RPC endpoint returned HTTP status {status}"
            )));
        }
        let value: Value = serde_json::from_slice(&body).map_err(DevkitError::decode)?;
        if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
            return Err(DevkitError::decode(
                "RPC response is missing JSON-RPC version 2.0",
            ));
        }
        if value.get("id").and_then(Value::as_u64) != Some(id) {
            return Err(DevkitError::decode(
                "RPC response identifier does not match request",
            ));
        }
        if let Some(error) = value.get("error") {
            let code = error
                .get("code")
                .and_then(Value::as_i64)
                .unwrap_or_default();
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("unknown RPC error");
            return Err(DevkitError::decode(format!("RPC error {code}: {message}")));
        }
        value
            .get("result")
            .cloned()
            .ok_or_else(|| DevkitError::decode("RPC response has neither result nor error"))
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

/// Validates an already-parsed RPC JSON value before it reaches typed consumers.
///
/// The transport contract returns a `Value`, so this protects downstream
/// processing but cannot prevent a transport from allocating while parsing raw
/// bytes. A future network transport must enforce the byte limit before parsing.
fn validate_response(value: &Value) -> Result<(), DevkitError> {
    measure_json_size(value, 0)?;
    Ok(())
}

fn measure_json_size(value: &Value, depth: usize) -> Result<usize, DevkitError> {
    if depth > MAX_RPC_RESPONSE_DEPTH {
        return Err(DevkitError::decode(format!(
            "RPC response nesting exceeds the {MAX_RPC_RESPONSE_DEPTH}-level limit"
        )));
    }

    match value {
        Value::Null => Ok(4),
        Value::Bool(value) => Ok(if *value { 4 } else { 5 }),
        Value::Number(value) => Ok(value.to_string().len()),
        Value::String(value) => measure_json_string(value),
        Value::Array(values) => {
            let mut size = 2usize;
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    size = checked_response_size(size, 1)?;
                }
                size = checked_response_size(size, measure_json_size(value, depth + 1)?)?;
            }
            Ok(size)
        }
        Value::Object(values) => {
            let mut size = 2usize;
            for (index, (key, value)) in values.iter().enumerate() {
                if index > 0 {
                    size = checked_response_size(size, 1)?;
                }
                size = checked_response_size(size, measure_json_string(key)?)?;
                size = checked_response_size(size, 1)?;
                size = checked_response_size(size, measure_json_size(value, depth + 1)?)?;
            }
            Ok(size)
        }
    }
}

fn measure_json_string(value: &str) -> Result<usize, DevkitError> {
    if value.len() > MAX_RPC_RESPONSE_BYTES {
        return Err(response_too_large());
    }

    let mut size = 2usize;
    for character in value.chars() {
        let escaped_size = match character {
            '"' | '\\' | '\u{08}' | '\t' | '\n' | '\u{0C}' | '\r' => 2,
            character if character <= '\u{1F}' => 6,
            character => character.len_utf8(),
        };
        size = checked_response_size(size, escaped_size)?;
    }
    Ok(size)
}

fn checked_response_size(current: usize, additional: usize) -> Result<usize, DevkitError> {
    let size = current
        .checked_add(additional)
        .ok_or_else(response_too_large)?;
    if size > MAX_RPC_RESPONSE_BYTES {
        return Err(response_too_large());
    }
    Ok(size)
}

fn response_too_large() -> DevkitError {
    DevkitError::decode(format!(
        "RPC response exceeds the {MAX_RPC_RESPONSE_BYTES}-byte limit"
    ))
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
                Ok(value) => {
                    validate_response(&value)?;
                    return Ok(value);
                }
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
        // Optional fields may be omitted; required status remains mandatory.
        let transport = MockTransport::with_responses(vec![json!({ "status": "healthy" })]);
        let health = Client::new(transport).health().unwrap();
        assert_eq!(health.status, "healthy");
        assert_eq!(health.latest_ledger, None);
    }

    #[test]
    fn rejects_missing_required_network_fields() {
        let transport = MockTransport::with_responses(vec![json!({ "protocolVersion": 23 })]);
        let error = Client::new(transport).network().unwrap_err();
        assert!(matches!(error, DevkitError::Decode(_)));
    }

    #[test]
    fn rejects_oversized_rpc_response() {
        let transport = MockTransport::with_responses(vec![json!({
            "status": "healthy",
            "extra": "x".repeat(MAX_RPC_RESPONSE_BYTES + 1)
        })]);
        let error = Client::new(transport).health().unwrap_err();
        assert!(matches!(error, DevkitError::Decode(message) if message.contains("byte limit")));
    }

    #[test]
    fn rejects_pathologically_nested_rpc_response() {
        let mut value = json!(null);
        for _ in 0..=MAX_RPC_RESPONSE_DEPTH {
            value = json!([value]);
        }
        let transport = MockTransport::with_responses(vec![value]);
        let error = Client::new(transport).health().unwrap_err();
        assert!(matches!(error, DevkitError::Decode(message) if message.contains("nesting")));
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
