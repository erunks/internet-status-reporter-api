use dotenv;

mod entity;
use entity::prelude::*;

use sea_orm::query::*;

// Import the needed modules for table creation
use sea_orm::{Database, EntityTrait};
// Handle errors using the `https://crates.io/crates/anyhow` crate
use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()>{
    dotenv::dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let db = Database::connect(database_url).await?;

    // let outtages = Outtages::find().all(&db).await;
    let outtage_pages = Outtages::find().order_by_desc(OuttagesColumn::CreatedAt).paginate(&db ,10);
    let outtages = outtage_pages.fetch_page(0).await?;

    // while let Some(outtages) = outtage_pages.fetch_and_next().await? {
    //     println!("{:?}", outtages);
    // }

    println!("{:?}", outtages);

    Ok(())
}
