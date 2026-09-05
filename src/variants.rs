use crate::recovery::RecoveryClass;

/// Common application error variants.
///
/// Provides a standard set of error types that most applications need.
/// Each variant maps to an appropriate HTTP status code and recovery class.
///
/// # Example
/// ```ignore
/// use app_error::CommonError;
///
/// fn load_config(path: &str) -> Result<Config, CommonError> {
///     let content = std::fs::read_to_string(path)
///         .map_err(|e| CommonError::io("failed to read config", e))?;
///     // ...
/// }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum CommonError {
    /// I/O error (file, network, etc.)
    #[cfg(feature = "std")]
    #[error("io error: {context}")]
    Io {
        /// Context message
        context: alloc::string::String,
        /// Underlying error
        #[source]
        source: std::io::Error,
    },

    /// Serialization/deserialization error
    #[error("serialization error: {0}")]
    Serialization(alloc::string::String),

    /// Resource not found
    #[error("not found: {0}")]
    NotFound(alloc::string::String),

    /// Authentication error
    #[error("authentication error: {0}")]
    Auth(alloc::string::String),

    /// Authorization/permission error
    #[error("forbidden: {0}")]
    Forbidden(alloc::string::String),

    /// Configuration error
    #[error("configuration error: {0}")]
    Config(alloc::string::String),

    /// Validation error
    #[error("validation error: {0}")]
    Validation(alloc::string::String),

    /// Internal/bug error
    #[error("internal error: {0}")]
    Internal(alloc::string::String),

    /// Timeout error
    #[error("timeout: {0}")]
    Timeout(alloc::string::String),

    /// Rate limit exceeded
    #[error("rate limited: retry after {retry_after_secs}s")]
    RateLimited {
        /// Seconds to wait before retrying
        retry_after_secs: u64,
    },

    /// Conflict (resource state conflict)
    #[error("conflict: {0}")]
    Conflict(alloc::string::String),

    /// Service unavailable
    #[error("service unavailable: {0}")]
    Unavailable(alloc::string::String),
}

impl CommonError {
    /// Create an IO error with context.
    #[cfg(feature = "std")]
    pub fn io(context: impl Into<alloc::string::String>, source: std::io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }

    /// Create a serialization error.
    pub fn serialization(msg: impl Into<alloc::string::String>) -> Self {
        Self::Serialization(msg.into())
    }

    /// Create a not found error.
    pub fn not_found(msg: impl Into<alloc::string::String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create an auth error.
    pub fn auth(msg: impl Into<alloc::string::String>) -> Self {
        Self::Auth(msg.into())
    }

    /// Create a forbidden error.
    pub fn forbidden(msg: impl Into<alloc::string::String>) -> Self {
        Self::Forbidden(msg.into())
    }

    /// Create a config error.
    pub fn config(msg: impl Into<alloc::string::String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a validation error.
    pub fn validation(msg: impl Into<alloc::string::String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create an internal/bug error.
    pub fn internal(msg: impl Into<alloc::string::String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Create a timeout error.
    pub fn timeout(msg: impl Into<alloc::string::String>) -> Self {
        Self::Timeout(msg.into())
    }

    /// Create a rate limited error.
    pub fn rate_limited(retry_after_secs: u64) -> Self {
        Self::RateLimited { retry_after_secs }
    }

    /// Create a conflict error.
    pub fn conflict(msg: impl Into<alloc::string::String>) -> Self {
        Self::Conflict(msg.into())
    }

    /// Create a service unavailable error.
    pub fn unavailable(msg: impl Into<alloc::string::String>) -> Self {
        Self::Unavailable(msg.into())
    }
}

impl crate::AppError for CommonError {
    #[cfg(feature = "errcode")]
    fn code(&self) -> error_codes::ErrorCode {
        match self {
            #[cfg(feature = "std")]
            Self::Io { .. } => error_codes::ErrorCode::Internal,
            Self::Serialization(_) => error_codes::ErrorCode::BadRequest,
            Self::NotFound(_) => error_codes::ErrorCode::NotFound,
            Self::Auth(_) => error_codes::ErrorCode::Auth,
            Self::Forbidden(_) => error_codes::ErrorCode::Auth,
            Self::Config(_) => error_codes::ErrorCode::Internal,
            Self::Validation(_) => error_codes::ErrorCode::Validation,
            Self::Internal(_) => error_codes::ErrorCode::Internal,
            Self::Timeout(_) => error_codes::ErrorCode::Unavailable,
            Self::RateLimited { .. } => error_codes::ErrorCode::RateLimited,
            Self::Conflict(_) => error_codes::ErrorCode::Conflict,
            Self::Unavailable(_) => error_codes::ErrorCode::Unavailable,
        }
    }

    fn recovery_class(&self) -> RecoveryClass {
        match self {
            #[cfg(feature = "std")]
            Self::Io { .. } => RecoveryClass::Retryable,
            Self::Serialization(_) => RecoveryClass::Permanent,
            Self::NotFound(_) => RecoveryClass::Permanent,
            Self::Auth(_) => RecoveryClass::UserAction,
            Self::Forbidden(_) => RecoveryClass::UserAction,
            Self::Config(_) => RecoveryClass::UserAction,
            Self::Validation(_) => RecoveryClass::UserAction,
            Self::Internal(_) => RecoveryClass::Bug,
            Self::Timeout(_) => RecoveryClass::Retryable,
            Self::RateLimited { .. } => RecoveryClass::Retryable,
            Self::Conflict(_) => RecoveryClass::Permanent,
            Self::Unavailable(_) => RecoveryClass::Retryable,
        }
    }

    fn user_message(&self) -> alloc::string::String {
        match self {
            Self::Internal(_) => alloc::string::String::from("An internal error occurred"),
            _ => alloc::format!("{}", self),
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            #[cfg(feature = "std")]
            Self::Io { .. } => "io",
            Self::Serialization(_) => "serialization",
            Self::NotFound(_) => "not_found",
            Self::Auth(_) => "auth",
            Self::Forbidden(_) => "forbidden",
            Self::Config(_) => "config",
            Self::Validation(_) => "validation",
            Self::Internal(_) => "internal",
            Self::Timeout(_) => "timeout",
            Self::RateLimited { .. } => "rate_limited",
            Self::Conflict(_) => "conflict",
            Self::Unavailable(_) => "unavailable",
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CommonError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::AppError;
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("kind", self.kind())?;
        map.serialize_entry("message", &alloc::format!("{}", self))?;
        map.end()
    }
}
