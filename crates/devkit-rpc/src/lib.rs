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
//! * Remote endpoints may not use private, link-local, unspecified, multicast,
//!   or loopback address literals. DNS answers are filtered through the same
//!   policy before a connection is made, so a public name cannot resolve to an
//!   internal address.
//! * Redirects are followed only to the original endpoint host, and every hop
//!   is re-validated. All other redirects fail closed.
//! * Responses are parsed with structured errors; malformed responses never
//!   panic.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use devkit_core::DevkitError;
use reqwest::dns::{Addrs, Name, Resolve, Resolving};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use url::{Host, Url};

const MAX_RPC_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
const MAX_RPC_RESPONSE_DEPTH: usize = 64;

/// Fallback backoff when a rate-limit response has no usable `Retry-After`.
const DEFAULT_RATE_LIMIT_BACKOFF: Duration = Duration::from_secs(1);

/// Default cap on the total time a single call may spend waiting out limits.
const DEFAULT_MAX_RATE_LIMIT_WAIT: Duration = Duration::from_secs(60);

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
    /// * A remote endpoint may not be an IP literal in a private, link-local,
    ///   unspecified, multicast, or loopback range. Loopback literals are only
    ///   allowed because they are the local-endpoint case.
    pub fn parse(input: &str) -> Result<Self, DevkitError> {
        let url = Url::parse(input.trim())
            .map_err(|e| DevkitError::invalid(format!("invalid endpoint URL: {e}")))?;

        validate_endpoint_url(&url)?;
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

fn validate_endpoint_url(url: &Url) -> Result<(), DevkitError> {
    match url.scheme() {
        "https" => {}
        "http" => {
            if !is_loopback(url) {
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

    let host = url
        .host()
        .ok_or_else(|| DevkitError::invalid("endpoint URL has no host"))?;
    if host_is_loopback(&host) {
        return Ok(());
    }
    let blocked = match host {
        Host::Ipv4(ip) => is_blocked_ip(IpAddr::V4(ip)),
        Host::Ipv6(ip) => is_blocked_ip(IpAddr::V6(ip)),
        Host::Domain(_) => false,
    };
    if blocked {
        return Err(DevkitError::invalid(format!(
            "endpoint host `{}` is a private, link-local, or otherwise non-public address",
            url.host_str().unwrap_or("<unknown>")
        )));
    }
    Ok(())
}

fn is_loopback(url: &Url) -> bool {
    url.host().is_some_and(|host| host_is_loopback(&host))
}

fn host_is_loopback(host: &Host<&str>) -> bool {
    match host {
        Host::Domain(domain) => {
            domain.eq_ignore_ascii_case("localhost")
                || domain
                    .to_ascii_lowercase()
                    .strip_suffix(".localhost")
                    .is_some()
        }
        Host::Ipv4(ip) => ip.is_loopback(),
        Host::Ipv6(ip) => ip_is_loopback(IpAddr::V6(*ip)),
    }
}

fn ip_is_loopback(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => ip.is_loopback(),
        IpAddr::V6(ip) => {
            ip.is_loopback() || ip.to_ipv4_mapped().is_some_and(|ip| ip.is_loopback())
        }
    }
}

fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => is_blocked_ipv4(ip),
        IpAddr::V6(ip) => is_blocked_ipv6(ip),
    }
}

fn is_blocked_ipv4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    ip.is_private()
        || ip.is_link_local()
        || ip.is_unspecified()
        || ip.is_broadcast()
        || ip.is_multicast()
        || octets[0] == 0
        || (octets[0] == 100 && (64..=127).contains(&octets[1]))
        || (octets[0] == 198 && matches!(octets[1], 18 | 19))
}

fn is_blocked_ipv6(ip: Ipv6Addr) -> bool {
    if let Some(ip) = ip.to_ipv4_mapped() {
        return is_blocked_ipv4(ip);
    }
    let segments = ip.segments();
    ip.is_unspecified()
        || ip.is_multicast()
        || (segments[0] & 0xfe00) == 0xfc00
        || (segments[0] & 0xffc0) == 0xfe80
}

fn resolved_ip_is_allowed(ip: IpAddr, allow_loopback: bool) -> bool {
    if ip_is_loopback(ip) {
        allow_loopback
    } else {
        !is_blocked_ip(ip)
    }
}

/// Resolves endpoint hostnames with the same address policy as URL validation.
///
/// The guard runs before reqwest connects, so a public hostname that resolves
/// to an internal address is refused instead of being used for SSRF.
#[derive(Clone, Copy)]
struct SsrfGuardResolver {
    allow_loopback: bool,
}

impl Resolve for SsrfGuardResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let host = name.as_str().to_owned();
        let result = resolve_guarded(&host, self.allow_loopback);
        Box::pin(std::future::ready(result))
    }
}

fn resolve_guarded(
    host: &str,
    allow_loopback: bool,
) -> Result<Addrs, Box<dyn std::error::Error + Send + Sync>> {
    let addresses = (host, 0u16).to_socket_addrs().map_err(|error| {
        std::io::Error::other(format!(
            "failed to resolve RPC endpoint host `{host}`: {error}"
        ))
    })?;
    let mut allowed = Vec::new();
    let mut refused = false;
    for address in addresses {
        if resolved_ip_is_allowed(address.ip(), allow_loopback) {
            allowed.push(address);
        } else {
            refused = true;
        }
    }
    if allowed.is_empty() {
        let reason = if refused {
            "only loopback, private, link-local, or otherwise non-public addresses"
        } else {
            "no addresses"
        };
        return Err(Box::new(std::io::Error::other(format!(
            "refusing RPC endpoint `{host}`: resolved to {reason}"
        ))));
    }
    Ok(Box::new(allowed.into_iter()) as Addrs)
}

fn redirect_policy(allowed_host: String) -> reqwest::redirect::Policy {
    reqwest::redirect::Policy::custom(move |attempt| {
        if attempt.previous().len() > 5 {
            return attempt.error("too many redirects");
        }
        let next = attempt.url().clone();
        if !next
            .host_str()
            .is_some_and(|host| host.eq_ignore_ascii_case(&allowed_host))
        {
            return attempt.error(format!(
                "refusing redirect to non-allowlisted host `{}`",
                next.host_str().unwrap_or("<unknown>")
            ));
        }
        match validate_endpoint_url(&next) {
            Ok(()) => attempt.follow(),
            Err(error) => attempt.error(error.to_string()),
        }
    })
}

fn validate_final_url(url: &Url, allowed_host: &str) -> Result<(), DevkitError> {
    if !url
        .host_str()
        .is_some_and(|host| host.eq_ignore_ascii_case(allowed_host))
    {
        return Err(DevkitError::Io(format!(
            "RPC transport refused a response from unexpected host `{}`",
            url.host_str().unwrap_or("<unknown>")
        )));
    }
    validate_endpoint_url(url)
        .map_err(|error| DevkitError::Io(format!("RPC transport refused final URL: {error}")))
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
        self.push_error(DevkitError::Io(message.into()));
    }

    /// Queues an arbitrary error, including a mocked rate-limit response.
    pub fn push_error(&self, error: DevkitError) {
        self.responses.borrow_mut().push_back(Err(error));
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
/// structured errors. Redirects are followed only to the endpoint host and are
/// re-validated at every hop; resolved addresses are filtered by the same
/// endpoint policy, so DNS cannot redirect a request to an internal network.
pub struct HttpTransport {
    endpoint: String,
    allowed_host: String,
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
        let allowed_host = endpoint
            .url
            .host_str()
            .ok_or_else(|| DevkitError::invalid("endpoint URL has no host"))?
            .to_owned();
        let client = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .redirect(redirect_policy(allowed_host.clone()))
            .dns_resolver(Arc::new(SsrfGuardResolver {
                allow_loopback: endpoint.is_loopback(),
            }))
            .build()
            .map_err(|error| {
                DevkitError::Io(format!("failed to build RPC HTTP client: {error}"))
            })?;
        Ok(Self {
            endpoint: endpoint.as_str().to_owned(),
            allowed_host,
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
            .map_err(|error| {
                let mut message = format!("RPC HTTP request failed: {error}");
                let mut source = std::error::Error::source(&error);
                while let Some(cause) = source {
                    message.push_str(&format!(": {cause}"));
                    source = cause.source();
                }
                DevkitError::Io(message)
            })?;
        validate_final_url(response.url(), &self.allowed_host)?;
        let status = response.status();
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|value| value.to_str().ok())
                .and_then(parse_retry_after);
            return Err(DevkitError::rate_limited(
                "RPC endpoint returned HTTP 429 Too Many Requests",
                retry_after,
            ));
        }
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

/// Parses a `Retry-After` header value in seconds.
///
/// The HTTP-date form is not parsed; callers fall back to a small bounded
/// backoff instead of guessing.
fn parse_retry_after(value: &str) -> Option<Duration> {
    value.trim().parse::<u64>().ok().map(Duration::from_secs)
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
    ///
    /// Rate-limit responses are retried only when the requested `Retry-After`
    /// fits inside the policy's total wait cap. The client never sleeps past
    /// that cap: it returns a clear rate-limit error instead.
    fn call(&self, method: &str, params: Value) -> Result<Value, DevkitError> {
        let mut attempt = 0;
        let mut waited = Duration::ZERO;
        loop {
            match self.transport.call(method, params.clone()) {
                Ok(value) => {
                    validate_response(&value)?;
                    return Ok(value);
                }
                Err(error) => {
                    if !self.retry.should_retry(attempt, &error) {
                        return Err(error);
                    }
                    if let Some(delay) = self.retry.rate_limit_delay(&error) {
                        let total = waited.saturating_add(delay);
                        if total > self.retry.max_rate_limit_wait {
                            return Err(DevkitError::rate_limited(
                                format!(
                                    "RPC rate-limit backoff of {}s would exceed the {}s cap",
                                    delay.as_secs(),
                                    self.retry.max_rate_limit_wait.as_secs()
                                ),
                                Some(delay),
                            ));
                        }
                        waited = total;
                        if !delay.is_zero() {
                            std::thread::sleep(delay);
                        }
                    }
                    attempt += 1;
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
    /// Maximum total time a single call may spend waiting out rate limits.
    pub max_rate_limit_wait: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy {
            max_retries: 2,
            max_rate_limit_wait: DEFAULT_MAX_RATE_LIMIT_WAIT,
        }
    }
}

impl RetryPolicy {
    /// Creates a policy with `max_retries` and the default wait cap.
    pub fn new(max_retries: u32) -> Self {
        RetryPolicy {
            max_retries,
            ..RetryPolicy::default()
        }
    }

    /// Sets the cap on cumulative rate-limit waiting for a single call.
    #[must_use]
    pub fn with_max_rate_limit_wait(mut self, max_rate_limit_wait: Duration) -> Self {
        self.max_rate_limit_wait = max_rate_limit_wait;
        self
    }

    /// Returns true when another attempt should be made.
    ///
    /// Transport failures and rate-limit responses are retryable. Decode and
    /// input errors are deterministic and are never retried.
    pub fn should_retry(&self, attempts_made: u32, error: &DevkitError) -> bool {
        attempts_made < self.max_retries
            && matches!(error, DevkitError::Io(_) | DevkitError::RateLimited { .. })
    }

    /// The delay to apply before retrying a rate-limit response.
    ///
    /// Uses `Retry-After` when present and a small bounded fallback otherwise.
    /// Returns `None` for errors that are not rate limits.
    pub fn rate_limit_delay(&self, error: &DevkitError) -> Option<Duration> {
        match error {
            DevkitError::RateLimited { retry_after, .. } => {
                Some(retry_after.unwrap_or(DEFAULT_RATE_LIMIT_BACKOFF))
            }
            _ => None,
        }
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
    fn rejects_private_and_link_local_ip_literals() {
        for input in [
            "https://10.0.0.1",
            "https://172.16.0.1",
            "https://192.168.1.1",
            "https://169.254.169.254",
            "https://100.64.0.1",
            "https://[fd00::1]",
            "https://[fe80::1]",
            "https://[::ffff:192.168.1.1]",
        ] {
            assert!(
                Endpoint::parse(input).is_err(),
                "{input} should be refused by the endpoint policy"
            );
        }
    }

    #[test]
    fn allows_loopback_literals_for_local_nodes() {
        assert!(Endpoint::parse("https://127.0.0.1").is_ok());
        assert!(Endpoint::parse("https://[::1]").is_ok());
        assert!(Endpoint::parse("http://localhost:8000").is_ok());
        assert!(Endpoint::parse("http://api.localhost:8000").is_ok());
        assert!(Endpoint::parse("https://[::ffff:127.0.0.1]").is_ok());
    }

    #[test]
    fn resolved_addresses_follow_the_endpoint_policy() {
        assert!(resolved_ip_is_allowed("127.0.0.1".parse().unwrap(), true));
        assert!(!resolved_ip_is_allowed("127.0.0.1".parse().unwrap(), false));
        assert!(!resolved_ip_is_allowed(
            "192.168.0.1".parse().unwrap(),
            true
        ));
        assert!(!resolved_ip_is_allowed(
            "169.254.169.254".parse().unwrap(),
            true
        ));
        assert!(!resolved_ip_is_allowed("fe80::1".parse().unwrap(), true));
        assert!(resolved_ip_is_allowed("1.1.1.1".parse().unwrap(), false));
        assert!(resolve_guarded("localhost", true).is_ok());
        assert!(resolve_guarded("localhost", false).is_err());
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
    fn retry_policy_retries_transport_errors_but_not_decode_errors() {
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

    #[test]
    fn parses_retry_after_seconds_and_ignores_http_dates() {
        assert_eq!(parse_retry_after("0"), Some(Duration::ZERO));
        assert_eq!(parse_retry_after(" 120 "), Some(Duration::from_secs(120)));
        assert_eq!(parse_retry_after("Wed, 21 Oct 2015 07:28:00 GMT"), None);
        assert_eq!(parse_retry_after("-1"), None);
        assert_eq!(parse_retry_after("later"), None);
    }

    #[test]
    fn http_transport_detects_429_and_carries_retry_after() {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("test server address");
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut request = [0u8; 1024];
            let _ = stream.read(&mut request);
            let body = "{}";
            let response = format!(
                "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 7\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            );
            stream
                .write_all(response.as_bytes())
                .expect("write response");
        });

        let endpoint = Endpoint::parse(&format!("http://{address}")).expect("loopback endpoint");
        let transport = HttpTransport::with_default_timeout(endpoint).expect("transport");
        let error = transport.call("getHealth", json!({})).unwrap_err();
        assert!(
            matches!(
                &error,
                DevkitError::RateLimited {
                    retry_after: Some(retry_after),
                    ..
                } if *retry_after == Duration::from_secs(7)
            ),
            "unexpected error: {error}"
        );
        server.join().expect("test server exits");
    }

    #[test]
    fn http_transport_follows_same_host_redirects_and_revalidates_them() {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("test server address");
        let server = std::thread::spawn(move || {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().expect("accept request");
                let mut request = [0u8; 1024];
                let _ = stream.read(&mut request);
                if request.starts_with(b"POST /final") {
                    let body = r#"{"jsonrpc":"2.0","id":1,"result":{"status":"healthy"}}"#;
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
                        body.len()
                    );
                    stream
                        .write_all(response.as_bytes())
                        .expect("write response");
                } else {
                    stream
                        .write_all(
                            b"HTTP/1.1 307 Temporary Redirect\r\nLocation: /final\r\nConnection: close\r\nContent-Length: 0\r\n\r\n",
                        )
                        .expect("write redirect");
                }
            }
        });

        let endpoint = Endpoint::parse(&format!("http://{address}")).expect("loopback endpoint");
        let transport = HttpTransport::with_default_timeout(endpoint).expect("transport");
        let health = transport
            .call("getHealth", json!({}))
            .expect("same-host redirect should be followed");
        assert_eq!(health["status"], "healthy");
        server.join().expect("test server exits");
    }

    #[test]
    fn http_transport_refuses_cross_host_redirects() {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let address = listener.local_addr().expect("test server address");
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept request");
            let mut request = [0u8; 1024];
            let _ = stream.read(&mut request);
            stream
                .write_all(
                    b"HTTP/1.1 302 Found\r\nLocation: http://example.com/rpc\r\nConnection: close\r\nContent-Length: 0\r\n\r\n",
                )
                .expect("write redirect");
        });

        let endpoint = Endpoint::parse(&format!("http://{address}")).expect("loopback endpoint");
        let transport = HttpTransport::with_default_timeout(endpoint).expect("transport");
        let error = transport.call("getHealth", json!({})).unwrap_err();
        assert!(
            error.to_string().contains("non-allowlisted"),
            "unexpected error: {error}"
        );
        server.join().expect("test server exits");
    }

    #[test]
    fn retry_policy_retries_rate_limits_and_uses_bounded_default_backoff() {
        let policy = RetryPolicy::new(2);
        let limited = DevkitError::rate_limited("429", None);
        assert!(policy.should_retry(0, &limited));
        assert!(policy.should_retry(1, &limited));
        assert!(!policy.should_retry(2, &limited));
        assert_eq!(
            policy.rate_limit_delay(&limited),
            Some(DEFAULT_RATE_LIMIT_BACKOFF)
        );
        assert_eq!(policy.rate_limit_delay(&DevkitError::decode("bad")), None);
    }

    #[test]
    fn client_retries_a_mocked_rate_limit_response_then_succeeds() {
        let transport = MockTransport::default();
        transport.push_error(DevkitError::rate_limited("HTTP 429", Some(Duration::ZERO)));
        transport.push_ok(json!({"status": "healthy"}));
        let client = Client::new(transport).with_retry(RetryPolicy::new(1));
        assert_eq!(client.health().unwrap().status, "healthy");
    }

    #[test]
    fn retry_after_beyond_the_cap_returns_a_clear_error_without_waiting() {
        let transport = MockTransport::default();
        transport.push_error(DevkitError::rate_limited(
            "HTTP 429",
            Some(Duration::from_secs(120)),
        ));
        let policy = RetryPolicy::new(2).with_max_rate_limit_wait(Duration::from_secs(30));
        let error = Client::new(transport)
            .with_retry(policy)
            .health()
            .unwrap_err();
        assert!(matches!(error, DevkitError::RateLimited { .. }));
        assert!(error.to_string().contains("cap"));
        assert!(error.to_string().contains("120s"));
    }

    #[test]
    fn client_returns_the_rate_limit_error_when_retries_are_exhausted() {
        let transport = MockTransport::default();
        transport.push_error(DevkitError::rate_limited("HTTP 429", Some(Duration::ZERO)));
        let error = Client::new(transport)
            .with_retry(RetryPolicy::new(0))
            .health()
            .unwrap_err();
        assert!(matches!(error, DevkitError::RateLimited { .. }));
        assert_eq!(error.retry_after(), Some(Duration::ZERO));
    }
}
