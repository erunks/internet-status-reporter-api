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

    let mut outtages_select = Outtages::find();
    if paginated_request.filter.maintenance.is_some() {
        outtages_select = outtages_select
            .filter(OuttagesColumn::Maintenance.eq(paginated_request.filter.maintenance.unwrap()));
    }

    if paginated_request.filter.loss.is_some() {
        outtages_select = outtages_select
            .filter(OuttagesColumn::Loss.gte(paginated_request.filter.loss.unwrap()));
    }

    if paginated_request.date.start.is_some() {
        if paginated_request.date.method == "after" {
            outtages_select = outtages_select
                .filter(OuttagesColumn::CreatedAt.gte(paginated_request.date.start.unwrap()));
        } else if paginated_request.date.method == "before" {
            outtages_select = outtages_select
                .filter(OuttagesColumn::CreatedAt.lt(paginated_request.date.start.unwrap()));
        } else if paginated_request.date.end.is_some() {
            if paginated_request.date.method == "between" {
                outtages_select = outtages_select.filter(OuttagesColumn::CreatedAt.between(
                    paginated_request.date.start.unwrap(),
                    paginated_request.date.end.unwrap(),
                ));
            } else if paginated_request.date.method == "not between" {
                outtages_select = outtages_select.filter(OuttagesColumn::CreatedAt.not_between(
                    paginated_request.date.start.unwrap(),
                    paginated_request.date.end.unwrap(),
                ));
            }
        }
    }

    let outtage_pages = outtages_select
        .order_by_desc(OuttagesColumn::CreatedAt)
        .paginate(&db, paginated_request.page.size);
    let outtages = outtage_pages
        .fetch_page(paginated_request.page.offset)
        .await;

    let outtage_count: usize = outtage_pages.num_items().await.unwrap();
    let meta: Meta =
        HashMap::<String, JsonApiValue>::from([("totalCount".to_string(), json!(outtage_count))]);

    match outtages {
        Ok(outtages) => {
            let (resources, _) = vec_to_jsonapi_resources(outtages);
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
