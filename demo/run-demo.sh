#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

CARGO_BIN="${CARGO:-}"
if [ -z "$CARGO_BIN" ]; then
  if command -v cargo >/dev/null 2>&1; then
    CARGO_BIN="cargo"
  elif [ -x "$HOME/.cargo/bin/cargo" ]; then
    CARGO_BIN="$HOME/.cargo/bin/cargo"
  elif [ -x "${USERPROFILE:-$HOME}/.cargo/bin/cargo.exe" ]; then
    CARGO_BIN="${USERPROFILE:-$HOME}/.cargo/bin/cargo.exe"
  elif command -v cmd.exe >/dev/null 2>&1; then
    CARGO_WIN="$(cmd.exe /c where cargo 2>/dev/null | head -n 1 | tr -d '\r')"
    if [ -n "$CARGO_WIN" ]; then
      if command -v cygpath >/dev/null 2>&1; then
        CARGO_BIN="$(cygpath -u "$CARGO_WIN")"
      else
        CARGO_BIN="$CARGO_WIN"
      fi
    fi
  fi
fi

run() {
  if command -v stellar-foundry >/dev/null 2>&1; then
    stellar-foundry "$@"
  elif [ -n "$CARGO_BIN" ]; then
    "$CARGO_BIN" run -q -p devkit-cli -- "$@"
  else
    echo "error: neither stellar-foundry nor cargo was found on PATH" >&2
    exit 1
  fi
}

echo "== version"
run version

echo
echo "== scval (testnet event value)"
run scval --format json "$(tr -d '\n' < demo/fixtures/testnet-scval.xdr)"

echo
echo "== envelope (testnet payment)"
run envelope --format json "$(tr -d '\n' < demo/fixtures/testnet-envelope.xdr)"

echo
echo "== inspect-wasm (minimal module)"
run inspect-wasm demo/fixtures/minimal.wasm

echo
echo "== analyze-wasm (SARIF)"
run analyze-wasm --format sarif demo/fixtures/minimal.wasm

if [ "${SKIP_NETWORK:-0}" != "1" ]; then
  echo
  echo "== rpc-health (network)"
  run rpc-health https://soroban-testnet.stellar.org
else
  echo
  echo "SKIP_NETWORK=1; skipping rpc-health"
fi
