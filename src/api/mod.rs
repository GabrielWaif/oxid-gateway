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

use rand::seq::SliceRandom;
use deadpool_diesel::postgres::Pool;
use docs::ApiDoc;
use handlers::{consumers::*, routes::*, targets::*, upstreams::*};
use hyper_tls::HttpsConnector;
use hyper_util::rt::TokioExecutor;
use tokio::net::ToSocketAddrs;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::database::repositories;
use hyper_util::client::legacy::Client;

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
        .route("/proxy/*path", any(handler))
        .route("/upstreams/:upstream_id/targets", post(create_target))
        .route(
            "/upstreams/:upstream_id/targets/:id",
            get(find_target_by_id),
        )
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
    Path(path): Path<String>,
    State(app_state): State<AppState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let https = HttpsConnector::new();
    let client: Client<_, Body> = Client::<(), ()>::builder(TokioExecutor::new()).build(https);

    let routes = match repositories::routes::find_all_routes(&app_state.pool).await {
        Ok(response) => response,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let full_path = format!("/{}", path);

    let route = match routes.iter().find(|x| full_path.contains(&x.path)) {
        Some(route) => route.clone(),
        None => return Err(StatusCode::NOT_FOUND),
    };

    let inner_path = route.inner_path.unwrap_or(String::from(""));

    let targets =
        match repositories::routes::find_all_route_targets(&app_state.pool, route.id).await {
            Ok(response) => response,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

    if targets.len() == 0 {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };

    let target = match targets.choose(&mut rand::thread_rng()) {
        Some(target) => target,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let uri = format!(
        "http://{}:{}{}",
        target.host,
        target.port,
        full_path.replace(&route.path, &inner_path)
    );

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    req.headers_mut()
        .insert("Host", target.host.parse().unwrap());

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
