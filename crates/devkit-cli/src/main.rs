//! `stellar-foundry` command-line interface.
//!
//! Exit codes:
//! * `0` success
//! * `2` usage error
//! * `3` runtime error (decode failure, I/O)

use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use devkit_core::{Network, OutputFormat, TOOL_NAME, VERSION};
use devkit_xdr as xdr;

const EXIT_SUCCESS: u8 = 0;
const EXIT_USAGE: u8 = 2;
const EXIT_ERROR: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum FormatArg {
    Terminal,
    Json,
}

impl From<FormatArg> for OutputFormat {
    fn from(value: FormatArg) -> Self {
        match value {
            FormatArg::Terminal => OutputFormat::Terminal,
            FormatArg::Json => OutputFormat::Json,
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
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => ExitCode::from(code),
        Err((code, message)) => {
            eprintln!("error: {message}");
            ExitCode::from(code)
        }
    }
}

fn run(cli: Cli) -> Result<u8, (u8, String)> {
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
            let json = serde_json::to_value(&info).map_err(|e| (EXIT_ERROR, e.to_string()))?;
            print_value(format, &json);
            Ok(EXIT_SUCCESS)
        }
        Command::Scval { value } => {
            let decoded = xdr::inspect_scval_base64(&value).map_err(runtime)?;
            print_value(format, &xdr::summary("scval", &decoded));
            Ok(EXIT_SUCCESS)
        }
        Command::Envelope { value } => {
            let decoded = xdr::inspect_envelope_base64(&value).map_err(runtime)?;
            print_value(format, &xdr::summary("transaction_envelope", &decoded));
            Ok(EXIT_SUCCESS)
        }
        Command::Event { value } => {
            let decoded = xdr::inspect_event_base64(&value).map_err(runtime)?;
            print_value(format, &xdr::summary("contract_event", &decoded));
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
        "capabilities": ["strkey", "scval", "transaction_envelope", "contract_event"],
        "networks": networks,
        "network_access": "none (offline decoding only)",
    });
    print_value(format, &report);
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

fn runtime(error: devkit_core::DevkitError) -> (u8, String) {
    (EXIT_ERROR, error.to_string())
}

#[allow(dead_code)]
fn usage(message: impl Into<String>) -> (u8, String) {
    (EXIT_USAGE, message.into())
}
