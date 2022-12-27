use super::*;

use crate::controllers::*;

use http_types::headers::HeaderValue;
use tide::{
    Result, Server,
    security::{CorsMiddleware, Origin},
};

#[derive(Clone, Debug)]
pub struct State {
    pub db_pool: sqlx::MySqlPool,
}

pub async fn create_server() -> Result<Server<State>> {
    let db_pool = database::create_pool();
    Ok(tide::with_state(State { db_pool }))
}

fn create_cors_middleware() -> CorsMiddleware {
    CorsMiddleware::new()
        .allow_origin(Origin::Any)
        .allow_methods("GET, POST, PUT, DELETE".parse::<HeaderValue>().unwrap())
        .allow_headers("Accept, Content-Type".parse::<HeaderValue>().unwrap())
}

pub async fn bind_and_listen(mut server: Server<State>) {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    server.with(create_cors_middleware());

    server.at("/").get(|_| async { Ok("Hello, world!") });
    server.at("/outtages").get(outtages::get);
    server.at("/modem_events").get(modem_events::get);

    println!("Starting server on 0.0.0.0:{}...", port);
    server
        .listen(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to port");
}
