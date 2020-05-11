use crate::Error;
use actix_web::{
    http::{header, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use serde_json;

pub trait ResultExt {
    fn json_response(self) -> serde_json::Result<HttpResponse>;
}

impl<T> ResultExt for Result<T, Error<StatusCode>>
where
    T: Serialize,
{
    fn json_response(self) -> serde_json::Result<HttpResponse> {
        match self {
            Ok(t) => match serde_json::to_string(&t) {
                Ok(j) => Ok(HttpResponse::Ok()
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(j)),
                Err(e) => Err(e),
            },
            Err(err) => Ok(HttpResponse::build(err.external.0)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_json_err(err.external.1))),
        }
    }
}

fn to_json_err(s: &'static str) -> String {
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
