//! `twoface::Error` wraps a Rust error type with a user-facing description. This stops users from
//! seeing your internal errors, which might contain sensitive implementation details that should be
//! kept private.
//!
//! # Example
//!
//! ```rust
//! use twoface::{ResultExt, Error};
//!
//! fn read_private_file() -> Result<String, Error<&'static str>> {
//!     // Do not leak this path to users!
//!     let secret_path = "/secrets/user01/profile.txt";
//!     // Use `describe_err` to wrap the result's Err value into a twoface::Error.
//!     std::fs::read_to_string(secret_path).describe_err("Could not get profile")
//! }
//!
//! /// Returns the user's profile (or a user-friendly error message).
//! fn get_user_response() -> String {
//!     match read_private_file() {
//!         Ok(s) => format!("Your profile: {}", s),
//!         Err(e) => {
//!             // Log the internal error
//!             eprintln!("ERROR: {:?}", e.internal);
//!             // Return the external error to users.
//!             e.to_string()
//!         }
//!     }
//! }
//! ```
//!

#[cfg(feature = "actix_web")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub mod web;

use std::fmt::{Display, Error as FmtError, Formatter};

/// Wraps a Rust error type with a user-facing description. This stops users from seeing your internal
/// errors, which might contain sensitive implementation details that should be kept private.
#[derive(Debug)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), FmtError> {
        write!(f, "{}", self.external)
    }
}

/// Easily turn an error into a twoface::Error by describing it to your users.
pub trait AnyhowExt {
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

pub trait ResultExt<T> {
    fn describe_err<External: Display>(self, external: External) -> Result<T, Error<External>>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn describe_err<External: Display>(self, external: External) -> Result<T, Error<External>> {
        self.map_err(|e| e.describe(external))
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
