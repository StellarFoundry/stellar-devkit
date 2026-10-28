# CLI Reference

The `stellar-foundry` binary.

```text
stellar-foundry [--config PATH] [--format terminal|json|sarif] <command>
```

`--format` is global and defaults to `terminal`. `--config PATH` is also global; the TOML configuration is currently consumed by `rpc-health`.

## Commands

### `version`

Prints the tool name and version.

### `doctor`

Prints a JSON report: tool, version, capabilities, networks (with passphrases
and default RPC endpoints), and a statement that network access is explicit and
limited to `rpc-health`.

### `strkey <VALUE>`

Validates a Stellar strkey and reports its kind (`account`, `contract`, or
`other`). Invalid input is a decode error.

### `scval <BASE64|->`

Decodes a base64 XDR `ScVal` into JSON. Pass `-` to read one value from stdin, for example `printf 'AAAAAwAAACo=' | stellar-foundry --format json scval -`.

### `envelope <BASE64|->`

Decodes a base64 XDR `TransactionEnvelope` into JSON. Pass `-` to read one value from stdin.

### `event <BASE64|->`

Decodes a base64 XDR `ContractEvent` into JSON. Pass `-` to read one value from stdin.

When `-` is supplied, stdin must be piped or redirected; the CLI never waits for interactive terminal input. Input is limited to 16 MiB and must be valid UTF-8. Only one value is read; streaming multiple values is not supported.

### `rpc-health [ENDPOINT]`

Checks the health endpoint of a Stellar RPC server. The endpoint can be supplied as an argument, configured as `rpc_url` in a TOML file, or supplied through `STELLAR_FOUNDRY_RPC_URL`. Precedence is command-line endpoint, then environment override, then config file. The request timeout defaults to 10 seconds and can be set with `timeout_ms` (100–120000) or `STELLAR_FOUNDRY_TIMEOUT_MS`. Set `verbose = true` or `STELLAR_FOUNDRY_VERBOSE=true` to emit static diagnostic event names; endpoint values and response payloads are not logged.

Example config:

```toml
rpc_url = "https://soroban-testnet.stellar.org"
timeout_ms = 10000
verbose = false
```

Example invocation:

```text
stellar-foundry --config ./stellar-foundry.toml rpc-health
stellar-foundry rpc-health https://soroban-testnet.stellar.org
```

Configuration files are limited to 1 MiB. Unknown TOML keys and invalid timeout values are rejected. See [CONFIGURATION.md](CONFIGURATION.md) for precedence, fields, and environment variables.

## Exit codes

| Code | Meaning |
| ---- | ------- |
| 0 | Success |
| 2 | Usage error |
| 3 | Runtime error (decode failure, I/O) |

## Structured errors

When `--format json` is selected, errors are written to stderr as one JSON object:

```json
{
  "error": {
    "category": "decode",
    "message": "decode error: ..."
  }
}
```

The category is one of `usage`, `decode`, `encode`, `invalid_input`, `configuration`, `io`, `rate_limited`, or `internal`. Process exit codes remain unchanged (2 for usage errors and 3 for runtime errors). Command-line parse errors use a generic JSON message so supplied argument text is not echoed into logs. Successful output remains on stdout; error JSON is on stderr.

## Output stability

- `terminal` output is human-oriented and may change.
- `json` output is machine-readable. Until a versioned schema is published
  (tracked as an issue), treat the shape as unstable.
