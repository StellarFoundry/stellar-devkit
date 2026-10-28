# Configuration

DevKit reads optional TOML configuration for commands that need it. Today that
is `rpc-health`; decoding, strkey, endpoint validation, and WASM inspection work
without configuration.

## Precedence

For the RPC endpoint, the order is:

1. Command-line argument: `stellar-foundry rpc-health <URL>`
2. Environment variable: `STELLAR_FOUNDRY_RPC_URL`
3. Config file: `rpc_url`
4. No default for mainnet; testnet has a documented default only through
   `Network::default_rpc_url()` in the library

The config file is selected with the global `--config PATH` flag. Without it,
defaults are used.

## File format

```toml
rpc_url = "https://soroban-testnet.stellar.org"
timeout_ms = 10000
verbose = false
```

| Key | Type | Default | Rules |
| --- | ---- | ------- | ----- |
| `rpc_url` | string | none | Must not be empty. Validated by the endpoint policy when used. |
| `timeout_ms` | integer | `10000` | Must be between `100` and `120000`. |
| `verbose` | boolean | `false` | Emits static diagnostic event names; never endpoint values or payloads. |

Unknown keys are rejected. Configuration files are limited to 1 MiB and must be
valid UTF-8.

## Environment variables

| Variable | Effect |
| -------- | ------ |
| `STELLAR_FOUNDRY_RPC_URL` | Overrides `rpc_url`. |
| `STELLAR_FOUNDRY_TIMEOUT_MS` | Overrides `timeout_ms`; must be an integer. |
| `STELLAR_FOUNDRY_VERBOSE` | Overrides `verbose`; accepts `true`/`false`, `1`/`0`, `yes`/`no`. |

Environment overrides are applied only by commands that consume configuration.
Other commands ignore these variables.

## Examples

```bash
stellar-foundry --config ./stellar-foundry.toml rpc-health
STELLAR_FOUNDRY_RPC_URL=https://soroban-testnet.stellar.org stellar-foundry rpc-health
stellar-foundry rpc-health https://soroban-testnet.stellar.org
```

## Security

- Treat configuration files as untrusted input.
- Configuration never enables network access implicitly. `rpc-health` must still
  receive an endpoint after precedence is resolved.
- Diagnostics emit event names only; do not add URLs, headers, or payloads to
  diagnostic output.
- Endpoint policy rejects embedded credentials, permits plaintext HTTP only for
  loopback hosts, and refuses private/link-local targets and cross-host
  redirects. See [NETWORK_SECURITY.md](NETWORK_SECURITY.md).

## Not implemented yet

- Per-network profiles and a `config init/show/validate` workflow.
- Structured logging levels; only `verbose = true` static diagnostics exist.
- Configuration for commands that do not make network requests.

These are tracked in [ISSUE_BACKLOG.md](ISSUE_BACKLOG.md).
