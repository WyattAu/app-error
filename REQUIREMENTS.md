# Requirements — app-error

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite threat-model rows.

Scope: Error classification toolkit (`error-classify`) — derive macro + traits for retryability and error classes

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-EC-001 | `#[derive(Classify)]` maps variants to declared `ErrorClass` values (retryable/fatal/…) | MUST |
| REQ-EC-002 | Classification is total: every variant receives exactly one class | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-EC-100 | Macro-generated code contains no panics on user input | MUST |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-EC-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-EC-901 | Public items carry doc comments with runnable examples where practical | SHOULD |
