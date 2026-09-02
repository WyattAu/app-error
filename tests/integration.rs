extern crate alloc;

use error_classify::{AppError, CommonError, RecoveryClass};

#[test]
fn common_error_variants() {
    let errors = alloc::vec![
        CommonError::serialization("bad json"),
        CommonError::not_found("user not found"),
        CommonError::auth("invalid token"),
        CommonError::forbidden("insufficient permissions"),
        CommonError::config("missing field"),
        CommonError::validation("invalid email"),
        CommonError::internal("null pointer"),
        CommonError::timeout("request timeout"),
        CommonError::rate_limited(60),
        CommonError::conflict("version mismatch"),
        CommonError::unavailable("service down"),
    ];

    for err in &errors {
        assert!(!err.kind().is_empty());
        assert!(!err.to_string().is_empty());
        assert!(!err.user_message().is_empty());
    }
}

#[cfg(feature = "std")]
#[test]
fn common_error_io_variant() {
    let err = CommonError::io("test", std::io::Error::new(std::io::ErrorKind::NotFound, "not found"));
    assert!(!err.kind().is_empty());
    assert!(!err.to_string().is_empty());
    assert!(!err.user_message().is_empty());
}

#[test]
fn recovery_class_mapping() {
    assert_eq!(CommonError::not_found("test").recovery_class(), RecoveryClass::Permanent);
    assert_eq!(CommonError::auth("test").recovery_class(), RecoveryClass::UserAction);
    assert_eq!(CommonError::internal("test").recovery_class(), RecoveryClass::Bug);
    assert_eq!(CommonError::timeout("test").recovery_class(), RecoveryClass::Retryable);

    #[cfg(feature = "std")]
    assert_eq!(CommonError::io("test", std::io::Error::new(std::io::ErrorKind::Other, "err")).recovery_class(), RecoveryClass::Retryable);
}

#[test]
fn retryable_checks() {
    assert!(!CommonError::not_found("test").is_retryable());
    assert!(CommonError::internal("test").is_bug());

    #[cfg(feature = "std")]
    assert!(CommonError::io("test", std::io::Error::new(std::io::ErrorKind::Other, "err")).is_retryable());
}

#[test]
fn user_message_hides_internal_details() {
    let err = CommonError::internal("null pointer at line 42");
    assert_eq!(err.user_message(), "An internal error occurred");

    let err = CommonError::not_found("user 123");
    assert_eq!(err.user_message(), "not found: user 123");
}
