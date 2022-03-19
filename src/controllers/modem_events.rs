use crate::entity::prelude::*;
use crate::helpers::request::*;
use crate::helpers::response::*;
use crate::helpers::server::State;

use jsonapi::model::vec_to_jsonapi_document;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder, SqlxMySqlConnector};
use tide::{Request, Response, Result, StatusCode};

pub async fn get(req: Request<State>) -> Result<Response> {
    let db_pool = req.state().db_pool.clone();
    let db = SqlxMySqlConnector::from_sqlx_mysql_pool(db_pool);

    let paginated_request: PaginatedRequest = req.query()?;

    let modem_event_pages = ModemEvents::find()
        .order_by_desc(ModemEventsColumn::CreatedAt)
        .paginate(&db, paginated_request.per_page);
    let modem_events = modem_event_pages
        .fetch_page(paginated_request.page_offset)
        .await?;

    Ok(json_api_response(
        StatusCode::Ok,
        &vec_to_jsonapi_document(modem_events),
    ))
}
