use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    api::{dtos::{pagination::{PaginationQueryDto, PaginationResponseDto, TargetsPagination}, targets::TargetFormDto}, errors::ResultErrors, AppState},
    database::{
        entities::targets::{NewTarget, Target},
        repositories,
    },
};

#[utoipa::path(
    post,
    path = "/upstreams/{upstream_id}/targets",
    operation_id = "create_target",
    tag = "Targets",
    responses (
        (status = 201, body = Target)
    )
)]
pub async fn create_target(
    Path(upstream_id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<TargetFormDto>,
) -> Result<(StatusCode, Json<Target>), ResultErrors> {
    let new_target = NewTarget {
        name: body.name,
        host: body.host,
        port: body.port,
        upstream_id,
    };

    let response = repositories::targets::create(&app_state.pool, new_target)
        .await
        .unwrap();

    return Ok((StatusCode::CREATED, Json(response)));
}

#[utoipa::path(
    delete,
    path = "/upstreams/{upstream_id}/targets/{id}",
    operation_id = "delete_target",
    tag = "Targets",
    responses (
        (status = 200, body = Target)
    )
)]
pub async fn delete_target(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
) -> Result<Json<Target>, ResultErrors> {
    let response = repositories::targets::delete(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/upstreams/{upstream_id}/targets/{id}",
    operation_id = "find_target_by_id",
    tag = "Targets",
    responses (
        (status = 200, body = Target)
    )
)]
pub async fn find_target_by_id(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
) -> Result<Json<Target>, ResultErrors> {
    let response = repositories::targets::find_by_id(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}

#[utoipa::path(
    put,
    path = "/upstreams/{upstream_id}/targets/{id}",
    operation_id = "update_target",
    tag = "Targets",
    responses (
        (status = 200, body = Target)
    )
)]
pub async fn update_target(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
    Json(body): Json<TargetFormDto>,
) -> Result<Json<Target>, ResultErrors> {
    // TODO: Check if target is part of upstream first
    let new_target = NewTarget {
        name: body.name,
        host: body.host,
        port: body.port,
        upstream_id,
    };

    let response = repositories::targets::update(&app_state.pool, id, new_target)
        .await
        .unwrap();

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/upstreams/{upstream_id}/targets",
    operation_id = "find_targets",
    tag = "Targets",
    params (
        PaginationQueryDto
    ),
    responses (
        (status = 200, body = TargetsPagination)
    )
)]
pub async fn find_targets(
    Path(upstream_id): Path<i32>,
    State(app_state): State<AppState>,
    pagination: Query<PaginationQueryDto>,
) -> Result<Json<TargetsPagination>, ResultErrors> {
    let pagination = pagination.0;
    let response =
        repositories::targets::find(&app_state.pool, pagination.offset, pagination.limit)
            .await
            .unwrap();

    return Ok(Json(PaginationResponseDto {
        items: response,
        count: 0,
    }));
}
