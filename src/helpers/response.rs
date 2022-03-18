use http_types::{Mime, StatusCode};
use serde::Serialize;
use serde_json;
use std::str::FromStr;
use tide::Response;

pub fn json_response<T: Serialize>(status: StatusCode, body: &T) -> Response {
    let mut res = Response::new(status);
    res.set_content_type(Mime::from_str("application/json;charset=utf-8").unwrap());
    res.set_body(serde_json::to_string(&body).unwrap());
    res
}
