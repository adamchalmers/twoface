type Text = &'static str;
use actix_web::http::StatusCode;
use std::fmt;

pub struct Error {
    pub internal: anyhow::Error,
    pub external: (StatusCode, Text),
}

impl Error {
    pub fn describe<E: Into<anyhow::Error>>(e: E, status: StatusCode, user_msg: Text) -> Self {
        Self {
            internal: e.into(),
            external: (status, user_msg),
        }
    }
}

impl fmt::Display for Error {
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
        let err = Error::describe(io_err, StatusCode::NOT_FOUND, "invalid file path");
        println!("{}", err);
    }
}
