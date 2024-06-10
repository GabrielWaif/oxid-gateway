use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    api::{errors::ResultErrors, AppState},
    database::{
        entities::upstreams::{NewUpstream, Upstream},
        repositories,
    },
};

#[utoipa::path(
    post,
    path = "/upstreams",
    operation_id = "create_upstream",
    tag = "Upstreams",
    responses (
        (status = 201, body = ResultBodyContainerUpstream)
    )
)]
pub async fn create_upstream(
    State(app_state): State<AppState>,
    Json(body): Json<NewUpstream>,
) -> Result<(StatusCode, Json<Upstream>), ResultErrors> {
    let response = repositories::upstreams::create(&app_state.pool, body)
        .await
        .unwrap();

    return Ok((StatusCode::CREATED, Json(response)));
}

#[utoipa::path(
    delete,
    path = "/upstreams/{id}",
    operation_id = "delete_upstream",
    tag = "Upstreams",
    responses (
        (status = 200, body = ResultBodyContainerUpstream)
    )
)]
pub async fn delete_upstream(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Upstream>, ResultErrors> {
    let response = repositories::upstreams::delete(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/upstreams/{id}",
    operation_id = "find_upstream_by_id",
    tag = "Upstreams",
    responses (
        (status = 200, body = ResultBodyContainerUpstream)
    )
)]
pub async fn find_upstream_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Upstream>, ResultErrors> {
    let response = repositories::upstreams::find_by_id(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}

#[utoipa::path(
    put,
    path = "/upstreams/{id}",
    operation_id = "update_upstream",
    tag = "Upstreams",
    responses (
        (status = 200, body = ResultBodyContainerUpstream)
    )
)]
pub async fn update_upstream(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<NewUpstream>,
) -> Result<Json<Upstream>, ResultErrors> {
    let response = repositories::upstreams::update(&app_state.pool, id, body)
        .await
        .unwrap();

    return Ok(Json(response));
}
