#[cfg(feature = "actix_web")]
mod web;

use std::fmt::{Display, Error as FmtError, Formatter};

pub struct Error<Data: Display> {
    pub internal: anyhow::Error,
    pub external: Data,
}

impl<Data: Display> Error<Data> {
    pub fn describe<E: Into<anyhow::Error>>(e: E, external: Data) -> Self {
        Self {
            internal: e.into(),
            external,
        }
    }
}

impl<Data: Display> Display for Error<Data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.external)
    }
}
