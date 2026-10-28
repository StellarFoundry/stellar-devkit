//! `stellar-foundry` command-line interface.
//!
//! Exit codes:
//! * `0` success
//! * `2` usage error
//! * `3` runtime error (decode failure, I/O)

use std::{
    fs,
    io::{self, IsTerminal, Read},
    process::ExitCode,
    time::Duration,
};

use clap::{Parser, Subcommand, ValueEnum};
use devkit_core::{Network, OutputFormat, TOOL_NAME, VERSION};
use devkit_xdr as xdr;

const EXIT_SUCCESS: u8 = 0;
const EXIT_USAGE: u8 = 2;
const EXIT_ERROR: u8 = 3;
const MAX_STDIN_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum FormatArg {
    Terminal,
    Json,
    Sarif,
}

impl From<FormatArg> for OutputFormat {
    fn from(value: FormatArg) -> Self {
        match value {
            FormatArg::Terminal => OutputFormat::Terminal,
            FormatArg::Json | FormatArg::Sarif => OutputFormat::Json,
        }
    }
}

/// StellarFoundry DevKit: developer infrastructure for Stellar and Soroban.
#[derive(Debug, Parser)]
#[command(name = "stellar-foundry", version, about, long_about = None)]
struct Cli {
    /// Optional TOML configuration file (used by commands that support configuration).
    #[arg(long, global = true)]
    config: Option<String>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = FormatArg::Terminal, global = true)]
    format: FormatArg,

    /// Include the raw base64 XDR input in decoded JSON output.
    #[arg(long, global = true)]
    raw: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print version information.
    Version,
    /// Report environment and capability information.
    Doctor,
    /// Classify a Stellar strkey (account, contract, or secret).
    Strkey {
        /// The strkey to classify.
        value: String,
    },
    /// Decode a base64 `ScVal` into JSON.
    Scval {
        /// Base64-encoded XDR `ScVal`.
        value: String,
    },
    /// Decode a base64 `TransactionEnvelope` into JSON.
    Envelope {
        /// Base64-encoded XDR `TransactionEnvelope`.
        value: String,
    },
    /// Decode a base64 `ContractEvent` into JSON.
    Event {
        /// Base64-encoded XDR `ContractEvent`.
        value: String,
    },
    /// Validate a Stellar RPC endpoint URL against the security policy.
    Endpoint {
        /// The endpoint URL to validate.
        url: String,
    },
    /// Inspect a WASM module without executing it.
    InspectWasm { path: String },
    /// Run structural checks against a WASM module; --format sarif emits SARIF 2.1.0.
    AnalyzeWasm { path: String },
    /// Query getHealth from an explicitly supplied Stellar RPC endpoint.
    RpcHealth {
        /// RPC endpoint; overrides config file and environment.
        endpoint: Option<String>,
    },
}

#[derive(Debug)]
struct CliError {
    code: u8,
    category: &'static str,
    message: String,
}

impl CliError {
    fn new(code: u8, category: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            category,
            message: message.into(),
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let json_errors = requests_json_format(&args);
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(error) => {
            use clap::error::ErrorKind;
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) {
                print!("{error}");
                return ExitCode::SUCCESS;
            }

            if json_errors {
                print_json_error(
                    "usage",
                    "invalid command-line arguments; run --help for usage",
                );
            } else {
                eprint!("{error}");
            }
            return ExitCode::from(EXIT_USAGE);
        }
    };

    let format: OutputFormat = cli.format.into();
    match run(cli) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            if format == OutputFormat::Json {
                print_json_error(error.category, &error.message);
            } else {
                eprintln!("error: {}", error.message);
            }
            ExitCode::from(error.code)
        }
    }
}

fn requests_json_format(args: &[String]) -> bool {
    args.windows(2)
        .any(|pair| pair[0] == "--format" && pair[1] == "json")
        || args.iter().any(|arg| arg == "--format=json")
}

fn print_json_error(category: &str, message: &str) {
    eprintln!(
        "{}",
        serde_json::json!({
            "error": {
                "category": category,
                "message": message
            }
        })
    );
}

fn run(cli: Cli) -> Result<u8, CliError> {
    let format: OutputFormat = cli.format.into();
    let options = xdr::DecodeOptions {
        include_raw: cli.raw,
    };
    let config_path = cli.config.clone();
    match cli.command {
        Command::Version => {
            println!("{TOOL_NAME} {VERSION}");
            Ok(EXIT_SUCCESS)
        }
        Command::Doctor => {
            doctor(format);
            Ok(EXIT_SUCCESS)
        }
        Command::Strkey { value } => {
            let info = xdr::describe_strkey(&value).map_err(runtime)?;
            let json = serde_json::to_value(&info)
                .map_err(|e| CliError::new(EXIT_ERROR, "internal", e.to_string()))?;
            print_value(format, &json);
            Ok(EXIT_SUCCESS)
        }
        Command::Scval { value } => {
            let value = read_value(value)?;
            let decoded =
                xdr::inspect_scval_base64_with_options(&value, options).map_err(runtime)?;
            print_value(format, &decoded);
            Ok(EXIT_SUCCESS)
        }
        Command::Envelope { value } => {
            let value = read_value(value)?;
            let decoded =
                xdr::inspect_envelope_base64_with_options(&value, options).map_err(runtime)?;
            print_value(format, &decoded);
            Ok(EXIT_SUCCESS)
        }
        Command::Event { value } => {
            let value = read_value(value)?;
            let decoded =
                xdr::inspect_event_base64_with_options(&value, options).map_err(runtime)?;
            print_value(format, &decoded);
            Ok(EXIT_SUCCESS)
        }
        Command::InspectWasm { path } => {
            let bytes = read_wasm(&path)?;
            let report = devkit_contract::inspect_wasm(&bytes).map_err(runtime)?;
            let value = serde_json::to_value(&report)
                .map_err(|error| CliError::new(EXIT_ERROR, "internal", error.to_string()))?;
            print_value(format, &value);
            Ok(EXIT_SUCCESS)
        }
        Command::AnalyzeWasm { path } => {
            let bytes = read_wasm(&path)?;
            let report = devkit_contract::inspect_wasm(&bytes).map_err(runtime)?;
            let findings = devkit_analysis::analyze(&report);
            if cli.format == FormatArg::Sarif {
                println!("{}", devkit_analysis::to_sarif(&findings, Some(&path)));
            } else {
                let count = findings.len();
                print_value(
                    format,
                    &serde_json::json!({ "findings": findings, "count": count }),
                );
            }
            Ok(EXIT_SUCCESS)
        }
        Command::RpcHealth { endpoint } => {
            let mut config = load_config(config_path.as_deref())?;
            config
                .apply_environment()
                .map_err(|message| CliError::new(EXIT_USAGE, "configuration", message))?;
            let endpoint = endpoint.or(config.rpc_url).ok_or_else(|| {
                CliError::new(
                    EXIT_USAGE,
                    "configuration",
                    "provide an RPC endpoint argument, set rpc_url in the config file, or set STELLAR_FOUNDRY_RPC_URL",
                )
            })?;
            devkit_config::diagnostic(config.verbose, "rpc_health_start");
            let endpoint = devkit_rpc::Endpoint::parse(&endpoint).map_err(runtime)?;
            let transport =
                devkit_rpc::HttpTransport::new(endpoint, Duration::from_millis(config.timeout_ms))
                    .map_err(runtime)?;
            let health = devkit_rpc::Client::new(transport)
                .health()
                .map_err(runtime)?;
            let value = serde_json::to_value(health)
                .map_err(|error| CliError::new(EXIT_ERROR, "internal", error.to_string()))?;
            print_value(format, &value);
            Ok(EXIT_SUCCESS)
        }
        Command::Endpoint { url } => {
            let endpoint = devkit_rpc::Endpoint::parse(&url).map_err(runtime)?;
            let report = serde_json::json!({
                "valid": true,
                "url": endpoint.as_str(),
                "loopback": endpoint.is_loopback(),
            });
            print_value(format, &report);
            Ok(EXIT_SUCCESS)
        }
    }
}

fn doctor(format: OutputFormat) {
    let networks: Vec<_> = Network::known()
        .iter()
        .map(|network| {
            serde_json::json!({
                "name": network.name(),
                "passphrase": network.passphrase(),
                "default_rpc_url": network.default_rpc_url(),
            })
        })
        .collect();
    let report = serde_json::json!({
        "tool": TOOL_NAME,
        "version": VERSION,
        "capabilities": ["strkey", "scval", "transaction_envelope", "contract_event", "endpoint_validation", "wasm_inspection", "wasm_analysis", "rpc_health"],
        "networks": networks,
        "network_access": "explicit only (rpc-health command)",
        "rpc_transport": "bounded HTTP JSON-RPC and deterministic mock",
    });
    print_value(format, &report);
}

/// Load optional TOML configuration with a strict size limit. Environment
/// overrides are applied by the command so unrelated CLI commands remain
/// independent of RPC-specific environment variables.
fn load_config(path: Option<&str>) -> Result<devkit_config::Config, CliError> {
    const MAX_CONFIG_BYTES: u64 = 1024 * 1024;
    let config = if let Some(path) = path {
        let metadata = fs::metadata(path).map_err(|error| {
            CliError::new(
                EXIT_ERROR,
                "io",
                format!("failed to inspect config file: {error}"),
            )
        })?;
        if metadata.len() > MAX_CONFIG_BYTES {
            return Err(CliError::new(
                EXIT_USAGE,
                "configuration",
                "config file exceeds the 1048576-byte limit",
            ));
        }
        let content = fs::read_to_string(path).map_err(|error| {
            CliError::new(
                EXIT_ERROR,
                "io",
                format!("failed to read config file: {error}"),
            )
        })?;
        if content.len() as u64 > MAX_CONFIG_BYTES {
            return Err(CliError::new(
                EXIT_USAGE,
                "configuration",
                "config file exceeds the 1048576-byte limit",
            ));
        }
        devkit_config::Config::parse_toml(&content)
            .map_err(|message| CliError::new(EXIT_USAGE, "configuration", message))?
    } else {
        devkit_config::Config::default()
    };
    config
        .validate()
        .map_err(|message| CliError::new(EXIT_USAGE, "configuration", message))?;
    Ok(config)
}

fn read_wasm(path: &str) -> Result<Vec<u8>, CliError> {
    let metadata = fs::metadata(path).map_err(|error| {
        CliError::new(
            EXIT_ERROR,
            "io",
            format!("failed to inspect WASM file: {error}"),
        )
    })?;
    if metadata.len() > devkit_contract::MAX_WASM_BYTES as u64 {
        return Err(CliError::new(
            EXIT_ERROR,
            "invalid_input",
            format!(
                "WASM exceeds the {}-byte limit",
                devkit_contract::MAX_WASM_BYTES
            ),
        ));
    }
    let bytes = fs::read(path).map_err(|error| {
        CliError::new(
            EXIT_ERROR,
            "io",
            format!("failed to read WASM file: {error}"),
        )
    })?;
    if bytes.len() > devkit_contract::MAX_WASM_BYTES {
        return Err(CliError::new(
            EXIT_ERROR,
            "invalid_input",
            format!(
                "WASM exceeds the {}-byte limit",
                devkit_contract::MAX_WASM_BYTES
            ),
        ));
    }
    Ok(bytes)
}

/// Resolve a decode argument, reading bounded UTF-8 input only when explicitly asked.
fn read_value(value: String) -> Result<String, CliError> {
    if value != "-" {
        return Ok(value);
    }

    let stdin = io::stdin();
    if stdin.is_terminal() {
        return Err(CliError::new(
            EXIT_USAGE,
            "usage",
            "input '-' requires piped or redirected stdin",
        ));
    }

    let mut input = String::new();
    stdin
        .take(MAX_STDIN_BYTES as u64 + 1)
        .read_to_string(&mut input)
        .map_err(|error| {
            CliError::new(EXIT_ERROR, "io", format!("failed to read stdin: {error}"))
        })?;

    if input.len() > MAX_STDIN_BYTES {
        return Err(CliError::new(
            EXIT_ERROR,
            "invalid_input",
            format!("stdin input exceeds the {MAX_STDIN_BYTES}-byte limit"),
        ));
    }

    Ok(input)
}

fn print_value(format: OutputFormat, value: &serde_json::Value) {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string(value).unwrap_or_default()),
        OutputFormat::Terminal => {
            println!(
                "{}",
                serde_json::to_string_pretty(value).unwrap_or_default()
            )
        }
    }
}

fn runtime(error: devkit_core::DevkitError) -> CliError {
    use devkit_core::DevkitError;
    let category = match &error {
        DevkitError::Decode(_) => "decode",
        DevkitError::Encode(_) => "encode",
        DevkitError::InvalidInput(_) => "invalid_input",
        DevkitError::Config(_) => "configuration",
        DevkitError::Io(_) => "io",
    };
    CliError::new(EXIT_ERROR, category, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_rpc_config_file() {
        let path = std::env::temp_dir().join(format!(
            "stellar-foundry-config-{}.toml",
            std::process::id()
        ));
        fs::write(
            &path,
            "rpc_url = \"https://soroban-testnet.stellar.org\"\ntimeout_ms = 2500\nverbose = true\n",
        )
        .expect("write temporary config");
        let config = load_config(path.to_str()).expect("valid config");
        let _ = fs::remove_file(&path);
        assert_eq!(
            config.rpc_url.as_deref(),
            Some("https://soroban-testnet.stellar.org")
        );
        assert_eq!(config.timeout_ms, 2500);
        assert!(config.verbose);
    }

    #[test]
    fn rejects_unknown_config_keys() {
        let path = std::env::temp_dir().join(format!(
            "stellar-foundry-invalid-config-{}.toml",
            std::process::id()
        ));
        fs::write(&path, "unexpected = true\n").expect("write temporary config");
        let result = load_config(path.to_str());
        let _ = fs::remove_file(&path);
        assert!(result.is_err());
    }
}
