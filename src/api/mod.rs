pub mod docs;
pub mod dtos;
pub mod errors;
pub mod handlers;

use std::fmt::Display;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use deadpool_diesel::postgres::Pool;
use docs::ApiDoc;
use handlers::{consumers::*, routes::*, targets::*, upstreams::*};
use tokio::net::ToSocketAddrs;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

pub async fn start_server<A>(address: A, postgres_pool: Pool) -> ()
where
    A: ToSocketAddrs + Display,
{
    let app = Router::new()
        .route("/upstreams/:upstream_id/targets", get(find_targets))
        .route("/upstreams/:upstream_id/targets", post(create_target))
        .route(
            "/upstreams/:upstream_id/targets/:id",
            get(find_target_by_id),
        )
        .route(
            "/upstreams/:upstream_id/targets/:id",
            delete(delete_target),
        )
        .route(
            "/upstreams/:upstream_id/targets/:id",
            put(update_target),
        )
        .route("/upstreams", post(create_upstream))
        .route("/upstreams", get(find_upstreams))
        .route("/upstreams/:id", get(find_upstream_by_id))
        .route("/upstreams/:id", delete(delete_upstream))
        .route("/upstreams/:id", put(update_upstream))
        .route("/routes", get(find_routes))
        .route("/consumers/:consumer_id/routes", get(find_consumer_routes))
        .route(
            "/consumers/:consumer_id/routes/:id",
            put(link_consumer_to_route),
        )
        .route(
            "/upstreams/:upstream_id/routes",
            get(find_routes_in_upstream),
        )
        .route("/upstreams/:upstream_id/routes", post(create_route))
        .route("/upstreams/:upstream_id/routes/:id", get(find_route_by_id))
        .route("/upstreams/:upstream_id/routes/:id", delete(delete_route))
        .route("/upstreams/:upstream_id/routes/:id", put(update_route))
        .route("/consumers", get(find_consumers))
        .route("/consumers", post(create_consumer))
        .route("/consumers/:id", get(find_consumer_by_id))
        .route("/consumers/:id", delete(delete_consumer))
        .route("/consumers/:id", put(update_consumer))
        .layer(
            CorsLayer::default()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-json", ApiDoc::openapi()))
        .with_state(AppState {
            pool: postgres_pool,
        });

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    tracing::info!("Serving service connection to address: {address}");

    axum::serve(listener, app).await.unwrap();
}
