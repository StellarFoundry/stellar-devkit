$ErrorActionPreference = "Stop"

Set-Location (Join-Path $PSScriptRoot "..")

function Invoke-Devkit {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $Arguments
    )

    if (Get-Command stellar-foundry -ErrorAction SilentlyContinue) {
        & stellar-foundry @Arguments
    } else {
        & cargo run -q -p devkit-cli -- @Arguments
    }
}

Write-Host "== version"
Invoke-Devkit version

Write-Host "`n== scval (testnet event value)"
$scval = (Get-Content demo/fixtures/testnet-scval.xdr -Raw).Trim()
Invoke-Devkit scval --format json $scval

Write-Host "`n== envelope (testnet payment)"
$envelope = (Get-Content demo/fixtures/testnet-envelope.xdr -Raw).Trim()
Invoke-Devkit envelope --format json $envelope

Write-Host "`n== inspect-wasm (minimal module)"
Invoke-Devkit inspect-wasm demo/fixtures/minimal.wasm

Write-Host "`n== analyze-wasm (SARIF)"
Invoke-Devkit analyze-wasm --format sarif demo/fixtures/minimal.wasm

if ($env:SKIP_NETWORK -ne "1") {
    Write-Host "`n== rpc-health (network)"
    Invoke-Devkit rpc-health https://soroban-testnet.stellar.org
} else {
    Write-Host "`nSKIP_NETWORK=1; skipping rpc-health"
}
