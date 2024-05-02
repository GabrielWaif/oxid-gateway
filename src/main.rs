use axum::{
    routing::{delete, get, post, put},
    Router,
};

use dotenvy::dotenv;
use oxid_gateway::{app_state::AppState, database_utils::{get_pool_connection, get_postgres_pool, migrate}, targets::{create_target, delete_target, get_target, update_target}};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = get_postgres_pool().await.unwrap();

    let connection = get_pool_connection(&pool).await;

    migrate(connection).await;

    let app = Router::new()
        .route("/targets", post(create_target))
        .route("/targets/:id", get(get_target))
        .route("/targets/:id", delete(delete_target))
        .route("/targets/:id", put(update_target))
        .with_state(AppState { pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
