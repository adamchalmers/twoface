use crate::Error;
use actix_web::{
    http::{header, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use serde_json;

pub trait ResultExt {
    fn json_response(self) -> HttpResponse;
}

impl<T> ResultExt for Result<T, Error<StatusCode>>
where
    T: Serialize,
{
    fn json_response(self) -> HttpResponse {
        match self.map(|s| serde_json::to_string(&s)) {
            // Result OK
            Ok(Ok(j)) => HttpResponse::Ok()
                .header(header::CONTENT_TYPE, "application/json")
                .body(j),
            // Result OK but couldn't deserialize into JSON
            Ok(Err(e)) => HttpResponse::InternalServerError()
                .header(header::CONTENT_TYPE, "application/json")
                .body(format!("{}", e)),
            // Result err
            Err(err) => HttpResponse::build(err.external.0)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_json_err(err.external.1)),
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
