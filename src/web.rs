//! Actix-web integration for Twoface
//!
//! By enabling the feature `actix_web`, Twoface errors can be used in Actix handlers.
//!
//!```rust
//! use actix_web::{web, http::StatusCode};
//! use twoface::{ResultExt};
//! use twoface::web::{HttpError, WebResult};
//!
//! async fn index() -> WebResult<web::Json<String>> {
//!     let file = std::fs::read_to_string("secret-filename-do-not-leak-to-user");
//!     file.describe_err(HttpError {
//!         code: StatusCode::NOT_FOUND,
//!         text: "page not found",
//!     })
//!     .map(web::Json)
//! }
//!```

use crate::Error;
use actix_web::{
    http::{header, StatusCode},
    HttpResponse,
};
use std::fmt;

/// Used to create HTTP responses with the given text and status code.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HttpError {
    pub code: StatusCode,
    pub text: &'static str,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.text)
    }
}

/// Use `WebResult` as the return type for your Actix handlers to ensure you never leak internal
/// errors to your users.
///
///```rust
/// use actix_web::{web, http::StatusCode};
/// use twoface::{ResultExt};
/// use twoface::web::{HttpError, WebResult};
///
/// async fn index() -> WebResult<web::Json<String>> {
///     let file = std::fs::read_to_string("secret-filename-do-not-leak-to-user");
///     file.describe_err(HttpError {
///         code: StatusCode::NOT_FOUND,
///         text: "page not found",
///     })
///     .map(web::Json)
/// }
///```
pub type WebResult<T> = Result<T, Error<HttpError>>;

impl actix_web::ResponseError for Error<HttpError> {
    fn status_code(&self) -> StatusCode {
        self.external.code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.external.code)
            .header(header::CONTENT_TYPE, "application/json")
            .body(to_json_err(&self.to_string()))
    }
}

fn to_json_err(s: &str) -> String {
    format!("{{\"error\": \"{}\"}}", s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ResultExt;
    use actix_web::{dev::Service, test, web, App, Error as ActixError};

    #[test]
    fn test_to_json_err() {
        let actual = to_json_err("page not found");
        let expected = "{\"error\": \"page not found\"}";
        assert_eq!(actual, expected);
    }

    #[actix_rt::test]
    async fn test() -> Result<(), ActixError> {
        async fn index() -> WebResult<web::Json<String>> {
            let file = std::fs::read_to_string("secret-filename-do-not-leak-to-user");
            file.describe_err(HttpError {
                code: StatusCode::NOT_FOUND,
                text: "page not found",
            })
            .map(web::Json)
        }

        let mut app =
            test::init_service(App::new().service(web::resource("/").route(web::get().to(index))))
                .await;

        // Send a request
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        let expected_body = "{\"error\": \"page not found\"}";
        if let Some(actix_web::body::Body::Bytes(bytes)) = resp.response().body().as_ref() {
            let actual_body = String::from_utf8(bytes.to_vec()).unwrap();
            assert_eq!(actual_body, expected_body);
        } else {
            panic!("wrong response type");
        }
        Ok(())
    }
}
