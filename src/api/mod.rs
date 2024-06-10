pub mod docs;
pub mod dtos;
pub mod errors;
pub mod handlers;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use deadpool_diesel::postgres::Pool;
use docs::ApiDoc;
use handlers::{consumers::*, routes::*, targets::*, upstreams::*};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

pub async fn start_server(postgres_pool: Pool) {
    let app = Router::new()
        .route("/upstreams/:upstream_id/targets", post(create_target))
        .route(
            "/upstreams/:upstream_id/targets/targets/:id",
            get(find_target_by_id),
        )
        .route(
            "/upstreams/:upstream_id/targets/targets/:id",
            delete(delete_target),
        )
        .route(
            "/upstreams/:upstream_id/targets/targets/:id",
            put(update_target),
        )
        .route("/upstreams", post(create_upstream))
        .route("/upstreams/:id", get(find_upstream_by_id))
        .route("/upstreams/:id", delete(delete_upstream))
        .route("/upstreams/:id", put(update_upstream))
        .route("/upstreams/:upstream_id/routes", post(create_route))
        .route("/upstreams/:upstream_id/routes/:id", get(find_route_by_id))
        .route("/upstreams/:upstream_id/routes/:id", delete(delete_route))
        .route("/upstreams/:upstream_id/routes/:id", put(update_route))
        .route("/consumers", post(create_consumer))
        .route("/consumers/:id", get(find_consumer_by_id))
        .route("/consumers/:id", delete(delete_consumer))
        .route("/consumers/:id", put(update_consumer))
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-json", ApiDoc::openapi()))
        .with_state(AppState {
            pool: postgres_pool,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
