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

    match outtages {
        Ok(outtages) => Ok(json_api_response(
            StatusCode::Ok,
            &vec_to_jsonapi_document(outtages),
        )),
        Err(err) => Ok(json_response(
            StatusCode::InternalServerError,
            &err.to_string(),
        )),
    }
}
