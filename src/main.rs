use axum::{
    routing::{delete, get, post, put},
    Router,
};

use dotenvy::dotenv;
use oxid_gateway::{
    app_state::AppState,
    database_utils::{get_pool_connection, get_postgres_pool, migrate},
    handlers::{
        routes::{create_route::create_route, delete_route::delete_route, find_route_by_id::find_route_by_id, update_route::update_route}, targets::{
            create_target::create_target, delete_target::delete_target,
            find_target_by_id::find_target_by_id, update_target::update_target,
        }, upstreams::{
            create_upstream::create_upstream, delete_upstream::delete_upstream,
            find_upstream_by_id::find_upstream_by_id, update_upstream::update_upstream,
        }
    },
    openapi::ApiDoc,
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
        .route("/upstreams/:upstream_id/targets", post(create_target))
        .route("/upstreams/:upstream_id/targets/targets/:id", get(find_target_by_id))
        .route("/upstreams/:upstream_id/targets/targets/:id", delete(delete_target))
        .route("/upstreams/:upstream_id/targets/targets/:id", put(update_target))
        .route("/upstreams", post(create_upstream))
        .route("/upstreams/:id", get(find_upstream_by_id))
        .route("/upstreams/:id", delete(delete_upstream))
        .route("/upstreams/:id", put(update_upstream))
        .route("/upstreams/:upstream_id/routes", post(create_route))
        .route("/upstreams/:upstream_id/routes/:id", get(find_route_by_id))
        .route("/upstreams/:upstream_id/routes/:id", delete(delete_route))
        .route("/upstreams/:upstream_id/routes/:id", put(update_route))
        // .route("/consumers", post(create_target))
        // .route("/consumers/:id", get(find_target_by_id))
        // .route("/consumers/:id", delete(delete_target))
        // .route("/consumers/:id", put(update_target))
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-json", ApiDoc::openapi()))
        .with_state(AppState { pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
