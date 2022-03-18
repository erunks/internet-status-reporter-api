mod controllers;
mod entity;
mod helpers;

use dotenv;
use helpers::server;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();

    let server = server::create_server().await?;
    server::bind_and_listen(server).await;

    Ok(())
}
