use app_error::{AppError, CommonError};
use proptest::prelude::*;

proptest! {
    #[test]
    fn kind_always_non_empty(msg in ".*") {
        let err = CommonError::not_found(&msg);
        prop_assert!(!err.kind().is_empty());
    }

    #[test]
    fn user_message_always_non_empty(msg in ".*") {
        let err = CommonError::internal(&msg);
        prop_assert!(!err.user_message().is_empty());
    }

    #[test]
    fn display_always_contains_kind(msg in ".*") {
        let err = CommonError::validation(&msg);
        let display = err.to_string();
        prop_assert!(display.contains("validation"));
    }
}
