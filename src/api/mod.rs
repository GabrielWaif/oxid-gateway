pub mod docs;
pub mod dtos;
pub mod errors;
pub mod handlers;

use std::fmt::Display;

use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{any, delete, get, post, put},
    Router,
};
use deadpool_diesel::postgres::Pool;
use docs::ApiDoc;
use dtos::pagination::PaginationQueryDto;
use handlers::{consumers::*, routes::*, targets::*, upstreams::*};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use tokio::net::ToSocketAddrs;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::database::repositories;

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

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
        .route("/proxy/{upstream_id}", any(handler))
        .route("/upstreams/:upstream_id/targets", post(create_target))
        .route("/upstreams/:upstream_id/targets/:id", get(find_target_by_id))
        .route("/upstreams/:upstream_id/targets/:id", delete(delete_target))
        .route("/upstreams/:upstream_id/targets/:id", put(update_target))
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
            "/consumers/:consumer_id/routes/:id",
            delete(unlink_consumer_to_route),
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

async fn handler(
    Path(upstream_id): Path<i32>,
    State(app_state): State<AppState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let path = req.uri().path();

    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let response = match repositories::routes::find_and_count_in_upstream(
        &app_state.pool,
        upstream_id,
        PaginationQueryDto {
            limit: 100,
            offset: 0,
            text: 
        },
    )
    .await
    {
        Ok(response) => response,
        Err(e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // TODO: Use route path
    let base_path = "/proxy";
    let inner_path = "/consumers";

    let uri = format!(
        "http://127.0.0.1:3000{}",
        path_query.replace(base_path, inner_path)
    );

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
