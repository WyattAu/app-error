# Threat Model — app-error

Reference: STRIDE. Scope: the crate's public API surface. Trust boundary:
(1) bytes/inputs entering public constructors and parsers, (2) concurrent
callers sharing interior state. app-error is an in-process library — it opens
no sockets and inherits the embedding process's trust domain.

Purpose: Error classification toolkit (`error-classify`) — derive macro + traits for retryability and error classes

## Assets

| ID | Asset | Exposed via |
|----|-------|-------------|
| A1 | correctness of retry decisions built on classes | hostile input, concurrent callers |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Residual risk |
|---|--------|----------|---------|------------|---------------|
| T1 | Wrong class causes unsafe retry of non-idempotent op | Tampering | `derive attribute parsing` | classes are explicit per variant; no inference from strings; macro tests pin output | documented |
| T2 | Macro injection via crafted identifiers | Spoofing | `derive input` | syn-parsed AST only; identifiers never string-interpolated | documented |

## Repudiation

The crate keeps no audit trail; attribution of calls to callers is out of
scope for an in-process library.

## Out of Scope

- Network transport security (the crate never opens sockets).
- Storage-host compromise: an attacker who controls the host can bypass all
  in-process mitigations.
- Denial of service via resource exhaustion of the host process beyond the
  bounds enforced above.

Reviewed: 2026-09-11
