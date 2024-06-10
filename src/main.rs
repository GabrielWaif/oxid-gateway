use api::start_server;

pub mod api;
pub mod database;
pub mod schema;

use database::{get_pool_connection, get_postgres_pool, migrate};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = get_postgres_pool().await.unwrap();
    let connection = get_pool_connection(&pool).await;
    migrate(connection).await;

    start_server(pool).await;
}
