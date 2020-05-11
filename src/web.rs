use crate::Error;
use actix_web::{
    http::{header, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use serde_json;
use std::fmt::{Display, Error as FmtError, Formatter};

pub trait ResultExt {
    fn json_response(self) -> HttpResponse;
}

pub struct HttpError {
    pub code: StatusCode,
    pub text: &'static str,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        self.text.fmt(f)
    }
}

impl<T> ResultExt for Result<T, Error<HttpError>>
where
    T: Serialize,
{
    fn json_response(self) -> HttpResponse {
        match self.map(|s| serde_json::to_string(&s)) {
            // Result OK
            Ok(Ok(j)) => HttpResponse::Ok()
                .header(header::CONTENT_TYPE, "application/json")
                .body(j),
            // Result OK but couldn't deserialize into JSON, so
            // respond with the JSON error.
            Ok(Err(e)) => {
                eprintln!("{}", e);
                HttpResponse::InternalServerError()
                    .header(header::CONTENT_TYPE, "application/json")
                    .body("Couldn't construct the JSON response")
            }
            // Result err
            Err(err) => HttpResponse::build(err.external.code)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_json_err(&err.to_string())),
        }
    }
}

fn to_json_err(s: &str) -> String {
    format!("{{\"error\": \"{}\"}}", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_err() {
        let actual = to_json_err("page not found");
        let expected = "{\"error\": \"page not found\"}";
        assert_eq!(actual, expected);
    }
}
