use http_types::{Mime, StatusCode};
use jsonapi::api::{DocumentData, ErrorSource, JsonApiDocument, JsonApiError, Meta, PrimaryData};
use sea_orm::error::DbErr;
use serde::Serialize;
use serde_json;
use std::{error::Error, str::FromStr};
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

pub fn create_json_api_error(status: StatusCode, error: DbErr) -> JsonApiError {
    let error_source = Error::source(&error);
    let source = match error_source {
        Some(source) => Some(ErrorSource {
            pointer: Some(source.to_string()),
            parameter: None,
        }),
        None => None,
    };

    JsonApiError {
        id: None,
        links: None,
        status: Some(status.to_string()),
        code: None,
        title: None,
        detail: Some(error.to_string()),
        source: source,
        meta: None,
    }
}
