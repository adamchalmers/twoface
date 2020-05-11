//! ## Actix-web integration
//! By enabling the feature `actix_web`, Twoface errors can be easily converted into HTTP responses.
//!
//!
//!```rust
//! use twoface::AnyhowExt;
//! use twoface::web::{ResultExt, HttpError};
//! use actix_web::{Responder, web, http::StatusCode};
//!
//! fn query_db_for_username(user_id: u32) -> anyhow::Result<String> {
//!     use anyhow::anyhow;
//!     Err(anyhow!("pq: could not query relation 'users': auth error"))
//! }
//!
//! fn get_username(user_id: web::Path<u32>) -> impl Responder {
//!     let username = query_db_for_username(*user_id);
//!     username
//!         .map_err(|e| {
//!             e.describe(HttpError {
//!                 code: StatusCode::INTERNAL_SERVER_ERROR,
//!                 text: "Database was unavailable",
//!             })
//!         })
//!         .json_response()
//! }
//!```

use crate::Error;
use actix_web::{
    http::{header, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use std::fmt::{Display, Error as FmtError, Formatter};

pub trait ResultExt {
    fn json_response(self) -> HttpResponse;
}

/// Used to create HTTP responses with the given text and status code.
pub struct HttpError {
    pub code: StatusCode,
    pub text: &'static str,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.text)
    }
}

impl<T> ResultExt for Result<T, Error<HttpError>>
where
    T: Serialize,
{
    /// Ok becomes HTTP 200 and the value is serialized into JSON for the body.
    /// Err becomes whatever HTTP error was chosen, with the user-facing description set as the JSON body.
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
    use crate::AnyhowExt;

    #[test]
    fn test_to_json_err() {
        let actual = to_json_err("page not found");
        let expected = "{\"error\": \"page not found\"}";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_http_response() {
        let file = std::fs::read("secret-filename-do-not-leak-to-user");
        let resp = file
            .map_err(|e| {
                e.describe(HttpError {
                    code: StatusCode::NOT_FOUND,
                    text: "page not found",
                })
            })
            .json_response();

        assert_eq!(StatusCode::NOT_FOUND, resp.status());

        let expected_body = "{\"error\": \"page not found\"}";
        if let Some(actix_web::body::Body::Bytes(bytes)) = resp.body().as_ref() {
            let actual_body = String::from_utf8(bytes.to_vec()).unwrap();
            assert_eq!(actual_body, expected_body);
        } else {
            panic!("wrong response type");
        }
    }
}
