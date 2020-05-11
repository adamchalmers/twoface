mod web;
type Text = &'static str;
use std::fmt;

pub struct Error<Data> {
    pub internal: anyhow::Error,
    pub external: (Data, Text),
}

impl<Data> Error<Data> {
    pub fn describe<E: Into<anyhow::Error>>(e: E, data: Data, user_msg: Text) -> Self {
        Self {
            internal: e.into(),
            external: (data, user_msg),
        }
    }
}

impl<Data> fmt::Display for Error<Data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.external.1)
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
