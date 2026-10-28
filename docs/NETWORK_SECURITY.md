# Network security policy

This document describes the fail-closed policy applied by
`devkit-rpc::HttpTransport`. Network access is always explicit: the only
networked CLI command is `rpc-health`, which requires a configured endpoint.

## Endpoint URLs

`Endpoint::parse` validates a URL before any transport is created:

- `https` endpoints are allowed.
- `http` is allowed only for loopback hosts (`localhost`, `*.localhost`,
  `127.0.0.0/8`, and `::1`). Plaintext traffic is never sent to a remote host.
- Any other scheme is rejected.
- Embedded credentials (`user:password@host`) are rejected.
- IP literals in private, link-local, loopback (unless the endpoint is itself
  loopback), unspecified, multicast, broadcast, or other non-public ranges are
  rejected. This includes IPv4-mapped IPv6 forms.

## DNS resolution

`HttpTransport` installs a guarded resolver. Every address returned for the
endpoint hostname is checked against the same policy before a connection is
made. Loopback answers are accepted only for an explicitly loopback endpoint.
If a hostname resolves exclusively to internal addresses, the request fails with
a transport error. Filtering happens on the addresses actually handed to the
HTTP client, so a public name cannot be used to reach an internal service.

## Redirects

Redirects are followed only when the next URL still targets the original endpoint
host. Each hop is re-validated with the endpoint URL policy, including the
scheme and address checks. Cross-host redirects, scheme downgrades to plaintext
remote hosts, and chains longer than five hops fail closed. The final response
URL is re-validated as defense in depth before the body is read.

The allowlist therefore contains exactly one host: the configured endpoint host.
There is no API for expanding it, because an RPC client only needs to talk to the
endpoint it was configured with.

## Other transport bounds

- A finite request timeout is required; a zero timeout is rejected.
- Response bodies are capped at 4 MiB before JSON parsing.
- JSON-RPC identifiers must match the request.
- Server error messages are returned as structured errors and never panic.

## Out of scope

- Proxying or tunneling.
- Reaching Stellar RPC through an operator-managed forward proxy.
- Protecting against a compromised endpoint that legitimately resolves to a
  public address.
