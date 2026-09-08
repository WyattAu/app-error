# app-error

Common application error variants with recovery classification.

`app-error` (package `error-classify`) gives you a `CommonError` enum with the
variants most applications need, plus a `RecoveryClass` for classifying errors
by retry strategy. It bridges application errors to HTTP status codes via
`errcode`.

## Quick Start

```rust
use error_classify::{CommonError, RecoveryClass, AppError};

let err = CommonError::not_found("user 42");
assert_eq!(err.recovery_class(), RecoveryClass::Permanent);
assert_eq!(err.kind(), "not_found");

// Internal details are hidden from clients:
assert_eq!(
    CommonError::internal("null pointer at line 42").user_message(),
    "An internal error occurred"
);
```

### Deriving `AppError`

```ignore
use error_classify::AppError;

#[derive(Debug, thiserror::Error, AppError)]
pub enum MyError {
    #[error("not found: {0}")]
    #[app_error(code = "NotFound", recovery = "Permanent")]
    NotFound(String),

    #[error("auth error: {0}")]
    #[app_error(code = "Auth", recovery = "UserAction")]
    Auth(String),
}
```

## Relationship with `errcode` and `http-errors`

These three crates cover separate layers — `app-error` is unchanged by the
merges below:

| Crate | Role |
|-------|------|
| [`errcode`](https://github.com/WyattAu/errcode) (package `error-codes`) | HTTP layer: `ErrorCode` enum, `ErrCode` / `HttpError` status-mapping traits, RFC 7807 `ProblemDetail` |
| [`http-errors`](https://github.com/WyattAu/http-error) (package `http-errors`) | Deprecated: thin re-export shim over `errcode` |
| `app-error` (this crate, package `error-classify`) | Application layer: `CommonError` variants + `RecoveryClass` retry classification |

Concretely:

- `app-error`'s `AppError::code()` returns `error_codes::ErrorCode` (behind
  the default-on `errcode` feature), so every `CommonError` maps to an HTTP
  status and can produce a `ProblemDetail`.
- `http-errors` was merged into `errcode` (its `Unauthorized` / `Forbidden`
  variants, `status_code()` alias, `as_str()` code strings, and the
  `HttpError` trait now live in `errcode`). Depending on `http-errors`
  still works via its shim, but new code should depend on `errcode`
  directly — `app-error` itself only ever talks to `errcode`.
- Recovery classification (`RecoveryClass::Retryable` / `UserAction` /
  `Permanent` / `Bug` / `Reconciliation`) exists only here; neither
  `errcode` nor `http-errors` provides it.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Standard library support (`CommonError::Io`, `CommonErrorExt`) |
| `errcode` | yes | `AppError::code()` bridging to `error-codes` |
| `alloc` | no | Explicit alloc support marker for `no_std` builds |
| `derive` | no | `AppError` derive macro (`error-classify-derive`) |
| `serde` | no | `Serialize` for `CommonError` (`{kind, message}` shape) |

## License

MIT OR Apache-2.0
