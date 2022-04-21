use http_types::{Mime, StatusCode};
use jsonapi::api::{DocumentData, JsonApiDocument, JsonApiError, Meta, PrimaryData};
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

pub fn json_api_response(status: StatusCode, body: &JsonApiDocument) -> Response {
    let mut res = Response::new(status);
    res.set_content_type(Mime::from_str("application/json;charset=utf-8").unwrap());
    res.set_body(serde_json::to_string(&body).unwrap());
    res
}

pub fn create_json_api_document(data: PrimaryData, meta: Meta) -> DocumentData {
    DocumentData {
        data: Some(data),
        included: None,
        links: None,
        meta: Some(meta),
        jsonapi: None,
    }
}

pub fn create_json_api_error(status: StatusCode, detail: &str) -> JsonApiError {
    JsonApiError {
        id: None,
        links: None,
        status: Some(status.to_string()),
        code: None,
        title: None,
        detail: Some(detail.to_string()),
        source: None,
        meta: None,
    }
}
