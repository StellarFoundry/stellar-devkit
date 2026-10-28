# Testnet demo

A short, reproducible tour of DevKit using captured Soroban testnet data. The
decode and WASM steps are fully offline; only the final `rpc-health` step uses
the network.

## Prerequisites

- Rust 1.91 or newer, **or** a `stellar-foundry` binary on `PATH`
- Network access only for `rpc-health`

## Run

```bash
bash demo/run-demo.sh
```

or on Windows:

```powershell
pwsh demo/run-demo.ps1
```

Skip the live RPC step:

```bash
SKIP_NETWORK=1 bash demo/run-demo.sh
```

```powershell
$env:SKIP_NETWORK = "1"; pwsh demo/run-demo.ps1
```

## What it shows

1. `version` and the tool identity.
2. `scval` decoding a real testnet event value.
3. `envelope` decoding a real testnet payment transaction.
4. `inspect-wasm` on a minimal valid WASM module.
5. `analyze-wasm --format sarif` emitting SARIF 2.1.0.
6. `rpc-health` against Soroban testnet (optional).

## Expected output (abridged)

```json
{"kind":"scval","schema_version":"1.0","value":{"i128":"100"}}
```

```json
{
  "kind": "transaction_envelope",
  "schema_version": "1.0",
  "value": {
    "tx": {
      "tx": {
        "fee": 100,
        "operations": [
          {
            "body": {
              "payment": {
                "amount": "1",
                "asset": "native",
                "destination": "GD64X2WOT23F3INTH3IXP4B4YGEPGPW6ZGXWQSGTFNQPUCGRN7WWAKQS"
              }
            }
          }
        ],
        "source_account": "GD64X2WOT23F3INTH3IXP4B4YGEPGPW6ZGXWQSGTFNQPUCGRN7WWAKQS"
      }
    }
  }
}
```

```json
{
  "contract_spec_bytes": 0,
  "contract_spec_entries": [],
  "custom_sections": [],
  "exports": [],
  "has_contract_spec": false,
  "imports": [],
  "wasm_bytes": 8
}
```

`analyze-wasm` emits a SARIF document with rule `DEVKIT-WASM-001` for the
missing `contractspecv0` section. This is a structural observation, not a
vulnerability finding.

## Fixtures

| File | Source |
| ---- | ------ |
| `fixtures/testnet-envelope.xdr` | Soroban testnet ledger 5434653, tx `ff4ec67e7123f35ba33a899195b035691f4fdc4a52a740a66b9710c0ddac9d97`, captured 2026-10-28 |
| `fixtures/testnet-scval.xdr` | `getEvents` value from contract `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`, tx `1c3d38c13a4508499bc8926a1012fb46f7bdd05e79b2b6e32c76b254ae3b4538`, ledger 5434564, i128 `100` |
| `fixtures/minimal.wasm` | A minimal valid 8-byte WASM module (`\0asm` + version 1) |

These are captured inputs, not live network calls. Re-capture them if the
testnet history window no longer contains the referenced ledgers.
