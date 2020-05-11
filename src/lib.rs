#[cfg(feature = "actix_web")]
mod web;

use std::fmt::{Display, Error as FmtError, Formatter};

pub struct Error<External: Display> {
    pub internal: anyhow::Error,
    pub external: External,
}

trait AnyhowExt {
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

impl<External: Display> Display for Error<External> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.external)
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
