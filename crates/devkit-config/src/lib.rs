//! Configuration parsing and secret-safe diagnostics.
//! Configuration is loaded from TOML text; environment variables can override
//! endpoint, timeout and verbosity without printing their values.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub rpc_url: Option<String>,
    pub timeout_ms: u64,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: None,
            timeout_ms: 10_000,
            verbose: false,
        }
    }
}

impl Config {
    pub fn parse_toml(input: &str) -> Result<Self, String> {
        let config: Self =
            toml::from_str(input).map_err(|error| format!("invalid config: {error}"))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(100..=120_000).contains(&self.timeout_ms) {
            return Err("timeout_ms must be between 100 and 120000".into());
        }
        if let Some(url) = &self.rpc_url {
            if url.trim().is_empty() {
                return Err("rpc_url must not be empty".into());
            }
        }
        Ok(())
    }

    pub fn apply_environment(&mut self) -> Result<(), String> {
        if let Ok(value) = std::env::var("STELLAR_FOUNDRY_RPC_URL") {
            self.rpc_url = Some(value);
        }
        if let Ok(value) = std::env::var("STELLAR_FOUNDRY_TIMEOUT_MS") {
            self.timeout_ms = value
                .parse()
                .map_err(|_| "STELLAR_FOUNDRY_TIMEOUT_MS must be an integer".to_string())?;
        }
        if let Ok(value) = std::env::var("STELLAR_FOUNDRY_VERBOSE") {
            self.verbose = match value.to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" => true,
                "0" | "false" | "no" => false,
                _ => return Err("STELLAR_FOUNDRY_VERBOSE must be true/false".into()),
            };
        }
        self.validate()
    }
}

/// Emit a static diagnostic event only when verbose mode is enabled.
/// Callers should pass event names, never URLs, headers, payloads or secrets.
pub fn diagnostic(verbose: bool, event: &'static str) {
    if verbose {
        eprintln!("stellar-foundry: {event}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_defaults_and_explicit_values() {
        let config = Config::parse_toml("timeout_ms = 5000\nverbose = true").unwrap();
        assert_eq!(config.timeout_ms, 5000);
        assert!(config.verbose);
        assert_eq!(config.rpc_url, None);
    }

    #[test]
    fn rejects_unknown_keys_and_unsafe_timeouts() {
        assert!(Config::parse_toml("mystery = 1").is_err());
        assert!(Config::parse_toml("timeout_ms = 1").is_err());
    }
}
