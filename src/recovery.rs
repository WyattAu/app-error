/// Recovery classification for errors.
///
/// Determines the appropriate response strategy when an error occurs.
/// Used by retry logic, circuit breakers, and monitoring systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryClass {
    /// Error is transient; retrying with backoff may succeed.
    /// Examples: network timeout, rate limit, temporary lock contention.
    Retryable,

    /// User must take action to resolve the error.
    /// Examples: invalid credentials, missing required field, permission denied.
    UserAction,

    /// Error is permanent; retrying won't help.
    /// Examples: resource not found, validation error, deleted entity.
    Permanent,

    /// Error indicates a bug in the code.
    /// Examples: null pointer, unwrap on None, unreachable code path.
    Bug,

    /// Error requires data reconciliation or repair.
    /// Examples: checksum mismatch, orphaned records, corrupted state.
    Reconciliation,
}

impl std::fmt::Display for RecoveryClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Retryable => write!(f, "retryable"),
            Self::UserAction => write!(f, "user_action"),
            Self::Permanent => write!(f, "permanent"),
            Self::Bug => write!(f, "bug"),
            Self::Reconciliation => write!(f, "reconciliation"),
        }
    }
}

impl RecoveryClass {
    /// Check if this class should trigger a retry.
    pub fn should_retry(&self) -> bool {
        matches!(self, Self::Retryable)
    }

    /// Check if this class should alert on monitoring.
    pub fn should_alert(&self) -> bool {
        matches!(self, Self::Bug | Self::Reconciliation)
    }

    /// Get a human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Retryable => "Transient error; retry with backoff",
            Self::UserAction => "User must take action to resolve",
            Self::Permanent => "Permanent error; retrying won't help",
            Self::Bug => "Indicates a bug in the code",
            Self::Reconciliation => "Requires data reconciliation",
        }
    }
}
