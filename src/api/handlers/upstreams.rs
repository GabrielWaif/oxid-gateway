use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    api::{
        dtos::pagination::{self, PaginationQueryDto, PaginationResponseDto, UpstreamsPagination},
        errors::ResultErrors,
        AppState,
    },
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
        (status = 201, body = Upstream)
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
        (status = 200, body = Upstream)
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
        (status = 200, body = Upstream)
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
    get,
    path = "/upstreams",
    operation_id = "find_upstreams",
    tag = "Upstreams",
    params (
        PaginationQueryDto
    ),
    responses (
        (status = 200, body = UpstreamsPagination)
    )
)]
pub async fn find_upstreams(
    State(app_state): State<AppState>,
    pagination: Query<PaginationQueryDto>,
) -> Result<Json<UpstreamsPagination>, ResultErrors> {
    let pagination = pagination.0;

    let response = match repositories::upstreams::find_and_count(&app_state.pool, pagination).await
    {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(PaginationResponseDto {
        items: response.0,
        count: response.1,
    }));
}

#[utoipa::path(
    put,
    path = "/upstreams/{id}",
    operation_id = "update_upstream",
    tag = "Upstreams",
    responses (
        (status = 200, body = Upstream)
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
