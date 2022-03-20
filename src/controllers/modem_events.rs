use crate::entity::prelude::*;
use crate::helpers::request::*;
use crate::helpers::response::*;
use crate::helpers::server::State;

use jsonapi::model::vec_to_jsonapi_document;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, SqlxMySqlConnector,
};
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

    match modem_events {
        Ok(modem_events) => Ok(json_api_response(
            StatusCode::Ok,
            &vec_to_jsonapi_document(modem_events),
        )),
        Err(err) => Ok(json_response(
            StatusCode::InternalServerError,
            &err.to_string(),
        )),
    }
}
