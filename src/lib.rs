#[cfg(feature = "actix_web")]
mod web;

use std::fmt::{Display, Error as FmtError, Formatter};

/// Wraps a Rust error type with a user-facing description. This stops users from seeing your internal
/// errors, which might contain sensitive implementation details that should be kept private.
pub struct Error<External: Display> {
    /// The underlying error, from some function. May contain sensitive information, so it should
    /// not be shown to users.
    pub internal: anyhow::Error,
    /// A user-friendly error that doesn't contain any sensitive information.
    pub external: External,
}

/// Displaying a twoface::Error will only display the external section. The internal error remains
/// private.
impl<External: Display> Display for Error<External> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.external)
    }
}

/// Easily turn an error into a twoface::Error by describing it to your users.
trait AnyhowExt {
    /// Adds a user-facing description to an internal error.
    fn describe<External: Display>(self, external: External) -> Error<External>;
}

impl<Internal: Into<anyhow::Error>> AnyhowExt for Internal {
    fn describe<External: Display>(self, external: External) -> Error<External> {
        Error {
            internal: self.into(),
            external,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_external_part_is_shown() {
        let io_err = std::fs::read("secret-filename-do-not-leak-to-user").unwrap_err();
        let err = io_err.describe("An IO error occurred");
        assert_eq!(err.to_string(), "An IO error occurred");
    }
}
