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
    /// Output format.
    #[arg(long, value_enum, default_value_t = FormatArg::Terminal, global = true)]
    format: FormatArg,

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
    RpcHealth { endpoint: String },
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
            let decoded = xdr::inspect_scval_base64(&value).map_err(runtime)?;
            print_value(format, &xdr::summary("scval", &decoded));
            Ok(EXIT_SUCCESS)
        }
        Command::Envelope { value } => {
            let value = read_value(value)?;
            let decoded = xdr::inspect_envelope_base64(&value).map_err(runtime)?;
            print_value(format, &xdr::summary("transaction_envelope", &decoded));
            Ok(EXIT_SUCCESS)
        }
        Command::Event { value } => {
            let value = read_value(value)?;
            let decoded = xdr::inspect_event_base64(&value).map_err(runtime)?;
            print_value(format, &xdr::summary("contract_event", &decoded));
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
            let endpoint = devkit_rpc::Endpoint::parse(&endpoint).map_err(runtime)?;
            let transport =
                devkit_rpc::HttpTransport::with_default_timeout(endpoint).map_err(runtime)?;
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
        return Err(CliError::new(EXIT_ERROR, "invalid_input", format!("WASM exceeds the {}-byte limit", devkit_contract::MAX_WASM_BYTES)));
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
