# Security Policy

This policy covers the **StellarFoundry DevKit tooling itself**.

## Reporting a vulnerability

Report suspected vulnerabilities privately via GitHub's
[private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability)
for this repository. **Do not open a public issue for a security
vulnerability.**

Include: a description, impact, reproduction steps, affected version/commit,
and any suggested remediation.

## What to expect

- Acknowledgement within 5 business days.
- Assessment within 10 business days.
- Coordinated disclosure and credit (unless you prefer anonymity).

## Supported versions

Pre-1.0; fixes target the latest `main` and the most recent release once
releases begin.

| Version | Supported |
| ------- | --------- |
| 0.1.x | Yes |

## Scope

In scope:

- Unsafe handling of untrusted input (XDR, strkey, RPC responses).
- Path traversal, SSRF, or unsafe endpoint handling in any network feature.
- Denial of service from malformed input (panics, unbounded memory).
- Secret or environment leakage in output.
- Supply-chain issues introduced by this project's dependencies.

Out of scope:

- Vulnerabilities in the Stellar network, Stellar RPC, or third-party crates.
- Smart-contract bugs that this toolkit merely reports.

## Security model

- No network access is performed by default. No network features exist yet.
- Decoding is bounded; malformed input is handled with structured errors, not
  panics.
- No secrets are read, logged, or committed.
- Untrusted input is never executed.

## Threat model and expectations

See the security boundaries in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).
Every security claim in this repository must be backed by implementation and
tests; if it is not, treat it as unverified.
