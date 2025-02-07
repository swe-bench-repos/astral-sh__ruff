use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_syntax_errors::SyntaxError;

/// Create wrapper `Violation` types for `SyntaxError`s.
macro_rules! syntax_errors {
    ($($error_type:ident$(,)*)*) => {
        $(#[derive(ViolationMetadata)]
        pub struct $error_type(pub SyntaxError);

        impl Violation for $error_type {
            #[derive_message_formats]
            fn message(&self) -> String {
                format!("{}", self.0.message())
            }
        })*
    };
}

syntax_errors! {
    MatchBeforePython310,
}
