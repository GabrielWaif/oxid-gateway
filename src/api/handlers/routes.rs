use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    api::{
        dtos::{
            pagination::{PaginationQueryDto, PaginationResponseDto, RoutesPagination},
            routes::RouteFormDto,
        },
        errors::ResultErrors,
        AppState,
    },
    database::{
        entities::routes::{NewRoute, Route},
        repositories,
    },
};

#[utoipa::path(
    post,
    path = "/upstreams/{upstream_id}/routes",
    operation_id = "create_route",
    tag = "Routes",
    responses (
        (status = 201, body = Route)
    )
)]
pub async fn create_route(
    Path(upstream_id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<RouteFormDto>,
) -> Result<(StatusCode, Json<Route>), ResultErrors> {
    let new_route = NewRoute {
        name: body.name,
        path: body.path,
        inner_path: body.inner_path,
        upstream_id,
    };

    let response = match repositories::routes::create(&app_state.pool, new_route).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok((StatusCode::CREATED, Json(response)));
}

#[utoipa::path(
    delete,
    path = "/upstreams/{upstream_id}/routes/{id}",
    operation_id = "delete_route",
    tag = "Routes",
    responses (
        (status = 200, body = Route)
    )
)]
pub async fn delete_route(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
) -> Result<Json<Route>, ResultErrors> {
    let response = match repositories::routes::delete(&app_state.pool, id).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/upstreams/{upstream_id}/routes/{id}",
    operation_id = "find_route_by_id",
    tag = "Routes",
    responses (
        (status = 200, body = Route)
    )
)]
pub async fn find_route_by_id(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
) -> Result<Json<Route>, ResultErrors> {
    let response = match repositories::routes::find_by_id(&app_state.pool, id).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(response));
}

#[utoipa::path(
    put,
    path = "/upstreams/{upstream_id}/routes/{id}",
    operation_id = "update_route",
    tag = "Routes",
    responses (
        (status = 200, body = Route)
    )
)]
pub async fn update_route(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
    Json(body): Json<RouteFormDto>,
) -> Result<Json<Route>, ResultErrors> {
    // TODO: Check if target is part of upstream first
    let new_route = NewRoute {
        name: body.name,
        path: body.path,
        inner_path: body.inner_path,
        upstream_id,
    };

    let response = match repositories::routes::update(&app_state.pool, id, new_route).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/upstreams/{upstream_id}/routes",
    operation_id = "find_routes",
    tag = "Routes",
    params (
        PaginationQueryDto
    ),
    responses (
        (status = 200, body = RoutesPagination)
    )
)]
pub async fn find_routes(
    State(app_state): State<AppState>,
    pagination: Query<PaginationQueryDto>,
) -> Result<Json<RoutesPagination>, ResultErrors> {
    let pagination = pagination.0;

    let response = match repositories::routes::find_and_count(&app_state.pool, pagination).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(PaginationResponseDto {
        items: response.0,
        count: response.1,
    }));
}
