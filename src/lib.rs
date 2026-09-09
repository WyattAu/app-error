#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Common application error variants with recovery classification.
//!
//! Provides a `CommonError` enum with standard variants (IO, NotFound, Auth,
//! etc.) and a `RecoveryClass` enum for classifying errors by retry strategy.
//! Complements `errcode` (HTTP error codes) and `http-errors` (HTTP responses).

extern crate alloc;

/// Extension traits for error conversion.
#[cfg(feature = "std")]
pub mod ext;
/// Recovery classification for errors.
pub mod recovery;
/// Common application error variants.
pub mod variants;

pub use recovery::RecoveryClass;
pub use variants::CommonError;

#[cfg(feature = "derive")]
pub use error_classify_derive::AppError;

#[cfg(feature = "errcode")]
pub use error_codes::ErrorCode;

/// Bridge trait to HTTP error codes and recovery classification.
///
/// Implement this trait on your error types to get automatic
/// HTTP status mapping and retry logic.
pub trait AppError {
    /// Get the error code for HTTP responses (from errcode crate).
    #[cfg(feature = "errcode")]
    fn code(&self) -> error_codes::ErrorCode;

    /// Get the recovery class for retry logic.
    fn recovery_class(&self) -> RecoveryClass;

    /// Get a user-facing error message.
    fn user_message(&self) -> alloc::string::String;

    /// Get the internal kind (stable identifier for logging).
    fn kind(&self) -> &'static str;

    /// Check if this error is retryable.
    fn is_retryable(&self) -> bool {
        matches!(self.recovery_class(), RecoveryClass::Retryable)
    }

    /// Check if this error requires user action.
    fn is_user_action(&self) -> bool {
        matches!(self.recovery_class(), RecoveryClass::UserAction)
    }

    /// Check if this error indicates a bug.
    fn is_bug(&self) -> bool {
        matches!(self.recovery_class(), RecoveryClass::Bug)
    }
}
