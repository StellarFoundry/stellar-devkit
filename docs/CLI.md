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

### `scval <BASE64>`

Decodes a base64 XDR `ScVal` into JSON.

### `envelope <BASE64>`

Decodes a base64 XDR `TransactionEnvelope` into JSON.

### `event <BASE64>`

Decodes a base64 XDR `ContractEvent` into JSON.

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
