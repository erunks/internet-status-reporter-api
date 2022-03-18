use sqlx::{MySqlPool, Pool};

pub fn create_pool() -> MySqlPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::connect_lazy(&url).unwrap()
}
