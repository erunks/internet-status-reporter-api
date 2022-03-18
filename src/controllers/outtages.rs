use super::*;

use crate::entity::prelude::*;
use crate::helpers::response::*;
use crate::helpers::server::State;

use sea_orm::{
    ConnectionTrait, Database, EntityTrait, PaginatorTrait, QueryOrder, SqlxMySqlConnector,
};
use tide::{Request, Response, Result, StatusCode};

pub async fn get(req: Request<State>) -> Result<Response> {
    let db_pool = req.state().db_pool.clone();
    let db = SqlxMySqlConnector::from_sqlx_mysql_pool(db_pool);

    let outtage_pages = Outtages::find()
        .order_by_desc(OuttagesColumn::CreatedAt)
        .paginate(&db, 10);
    let outtages = outtage_pages.fetch_page(0).await?;

    Ok(json_response::<std::vec::Vec<entity::outtages::Model>>(
        StatusCode::Ok,
        &outtages,
    ))
}
