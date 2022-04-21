use crate::entity::prelude::*;
use crate::helpers::request::*;
use crate::helpers::response::*;
use crate::helpers::server::State;

use jsonapi::{
    api::{DocumentError, JsonApiDocument, JsonApiValue, Meta, PrimaryData},
    model::vec_to_jsonapi_resources,
};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, SqlxMySqlConnector,
};
use serde_json::json;
use std::collections::HashMap;
use tide::{Request, Response, Result, StatusCode};

pub async fn get(req: Request<State>) -> Result<Response> {
    let db_pool = req.state().db_pool.clone();
    let db = SqlxMySqlConnector::from_sqlx_mysql_pool(db_pool);

    let paginated_request: PaginatedRequest = req.query()?;
    println!("{:?}", paginated_request);

    let mut modem_event_select = ModemEvents::find();
    if paginated_request.filter.maintenance.is_some() {
        modem_event_select = modem_event_select.filter(
            ModemEventsColumn::Maintenance.eq(paginated_request.filter.maintenance.unwrap()),
        );
    }

    if paginated_request.date.start.is_some() {
        if paginated_request.date.method == "after" {
            modem_event_select = modem_event_select
                .filter(ModemEventsColumn::CreatedAt.gte(paginated_request.date.start.unwrap()));
        } else if paginated_request.date.method == "before" {
            modem_event_select = modem_event_select
                .filter(ModemEventsColumn::CreatedAt.lt(paginated_request.date.start.unwrap()));
        } else if paginated_request.date.end.is_some() {
            if paginated_request.date.method == "between" {
                modem_event_select =
                    modem_event_select.filter(ModemEventsColumn::CreatedAt.between(
                        paginated_request.date.start.unwrap(),
                        paginated_request.date.end.unwrap(),
                    ));
            } else if paginated_request.date.method == "not between" {
                modem_event_select =
                    modem_event_select.filter(ModemEventsColumn::CreatedAt.not_between(
                        paginated_request.date.start.unwrap(),
                        paginated_request.date.end.unwrap(),
                    ));
            }
        }
    }

    let modem_event_pages = modem_event_select
        .order_by_desc(ModemEventsColumn::CreatedAt)
        .paginate(&db, paginated_request.page.size);
    let modem_events = modem_event_pages
        .fetch_page(paginated_request.page.offset)
        .await;

    let modem_event_count: usize = modem_event_pages.num_items().await.unwrap();
    let meta: Meta = HashMap::<String, JsonApiValue>::from([(
        "totalCount".to_string(),
        json!(modem_event_count),
    )]);

    match modem_events {
        Ok(modem_events) => {
            let (resources, _) = vec_to_jsonapi_resources(modem_events);
            let document =
                create_json_api_document(PrimaryData::Multiple(resources.to_vec()), meta);

            Ok(json_api_response(
                StatusCode::Ok,
                &JsonApiDocument::Data(document),
            ))
        }
        Err(err) => {
            let status_code = StatusCode::InternalServerError;
            let document = DocumentError {
                errors: vec![create_json_api_error(status_code, &err.to_string())],
                links: None,
                meta: Some(meta),
                jsonapi: None,
            };

            Ok(json_api_response(
                status_code,
                &JsonApiDocument::Error(document),
            ))
        }
    }
}
