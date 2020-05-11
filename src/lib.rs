#[cfg(feature = "actix_web")]
mod web;

use std::fmt::{Display, Error as FmtError, Formatter};

pub struct Error<External: Display> {
    pub internal: anyhow::Error,
    pub external: External,
}

impl<External: Display> Error<External> {
    pub fn describe<E: Into<anyhow::Error>>(e: E, external: External) -> Self {
        Self {
            internal: e.into(),
            external,
        }
    }
}

impl<External: Display> Display for Error<External> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.external)
    }
}
