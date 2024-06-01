use axum::{
    routing::{delete, get, post, put}, Router
};

use dotenvy::dotenv;
use oxid_gateway::{
    app_state::AppState,
    database_utils::{get_pool_connection, get_postgres_pool, migrate},
    handlers::targets::{
        create_target::create_target, delete_target::delete_target,
        find_target_by_id::find_target_by_id, update_target::update_target,
    }, openapi::ApiDoc,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = get_postgres_pool().await.unwrap();

    let connection = get_pool_connection(&pool).await;

    migrate(connection).await;

    let app = Router::new()
        .route("/targets", post(create_target))
        .route("/targets/:id", get(find_target_by_id))
        .route("/targets/:id", delete(delete_target))
        .route("/targets/:id", put(update_target))
        .route("/upstreams", post(create_target))
        .route("/upstreams/:id", get(find_target_by_id))
        .route("/upstreams/:id", delete(delete_target))
        .route("/upstreams/:id", put(update_target))
        .route("/routes", post(create_target))
        .route("/routes/:id", get(find_target_by_id))
        .route("/routes/:id", delete(delete_target))
        .route("/routes/:id", put(update_target))
        .route("/consumers", post(create_target))
        .route("/consumers/:id", get(find_target_by_id))
        .route("/consumers/:id", delete(delete_target))
        .route("/consumers/:id", put(update_target))
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-json", ApiDoc::openapi()))
        .with_state(AppState { pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
