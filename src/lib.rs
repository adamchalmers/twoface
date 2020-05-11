#[cfg(feature = "actix_web")]
mod web;

use std::fmt;

pub struct Error<Data> {
    pub internal: anyhow::Error,
    pub user_data: Data,
    pub user_text: &'static str,
}

impl<Data> Error<Data> {
    pub fn describe<E: Into<anyhow::Error>>(
        e: E,
        user_data: Data,
        user_text: &'static str,
    ) -> Self {
        Self {
            internal: e.into(),
            user_data,
            user_text,
        }
    }
}

impl<Data> fmt::Display for Error<Data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.user_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let io_err = std::fs::read_to_string("not-a-real-file-path").unwrap_err();
        let err = Error::describe(io_err, 404, "invalid file path");
        println!("{}", err);
    }
}
