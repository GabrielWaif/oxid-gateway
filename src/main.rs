use api::start_server;

pub mod api;
pub mod database;
pub mod schema;

use database::{get_pool_connection, get_postgres_pool, migrate};
use dotenvy::dotenv;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set default tracing subscriber");

    tracing::info!("Starting Logger");

    let pool = get_postgres_pool()
        .await
        .unwrap();

    let connection = get_pool_connection(&pool).await.unwrap();

    tracing::info!("Connected to connection pool");

    migrate(connection).await;

    start_server("0.0.0.0:8080", pool).await;
}
