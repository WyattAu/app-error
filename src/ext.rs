use crate::CommonError;

/// Extension trait for converting standard error types to `CommonError`.
pub trait CommonErrorExt {
    /// Convert to a CommonError with context.
    fn into_common(self, context: &str) -> CommonError;
}

impl CommonErrorExt for std::io::Error {
    fn into_common(self, context: &str) -> CommonError {
        CommonError::io(context, self)
    }
}

impl CommonErrorExt for std::num::ParseIntError {
    fn into_common(self, context: &str) -> CommonError {
        CommonError::validation(format!("{}: {}", context, self))
    }
}

impl CommonErrorExt for std::num::ParseFloatError {
    fn into_common(self, context: &str) -> CommonError {
        CommonError::validation(format!("{}: {}", context, self))
    }
}
