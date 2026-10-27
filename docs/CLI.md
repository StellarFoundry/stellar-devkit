# CLI Reference

The `stellar-foundry` binary.

```text
stellar-foundry [--format terminal|json] <command>
```

`--format` is global and defaults to `terminal`.

## Commands

### `version`

Prints the tool name and version.

### `doctor`

Prints a JSON report: tool, version, capabilities, networks (with passphrases
and default RPC endpoints), and a statement that network access is disabled.

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

## Exit codes

| Code | Meaning |
| ---- | ------- |
| 0 | Success |
| 2 | Usage error |
| 3 | Runtime error (decode failure, I/O) |

## Output stability

- `terminal` output is human-oriented and may change.
- `json` output is machine-readable. Until a versioned schema is published
  (tracked as an issue), treat the shape as unstable.
