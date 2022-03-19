use crate::entity::outtages::Model;

use http_types::{Mime, StatusCode};
use jsonapi::model::*;
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

pub fn json_api_response(status: StatusCode, body: &Vec<Model>) -> Response {
    let mut res = Response::new(status);
    res.set_content_type(Mime::from_str("application/vnd.api+json;charset=utf-8").unwrap());
    let json_api_doc = vec_to_jsonapi_document(body.clone());
    res.set_body(serde_json::to_string(&json_api_doc).unwrap());
    res
}
