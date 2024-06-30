use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    api::{
        dtos::{
            consumers::ConsumerFormDto,
            pagination::{ConsumersPagination, PaginationQueryDto, PaginationResponseDto},
        },
        errors::ResultErrors,
        AppState,
    },
    database::{
        entities::consumers::{ApiConsumer, NewConsumer},
        repositories,
    },
};

#[utoipa::path(
    post,
    path = "/consumers",
    operation_id = "create_consumer",
    tag = "Consumers",
    responses (
        (status = 201, body = ApiConsumer)
    )
)]
pub async fn create_consumer(
    State(app_state): State<AppState>,
    Json(body): Json<ConsumerFormDto>,
) -> Result<(StatusCode, Json<ApiConsumer>), ResultErrors> {
    let new_consumer = NewConsumer {
        name: body.name,
        api_key: Uuid::new_v4().to_string(),
    };

    let response = match repositories::consumers::create(&app_state.pool, new_consumer).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok((StatusCode::CREATED, Json(response)));
}

#[utoipa::path(
    delete,
    path = "/consumers/{id}",
    operation_id = "delete_consumer",
    tag = "Consumers",
    responses (
        (status = 200, body = ApiConsumer)
    )
)]
pub async fn delete_consumer(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiConsumer>, ResultErrors> {
    let response = match repositories::consumers::delete(&app_state.pool, id).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/consumers/{id}",
    operation_id = "find_consumer_by_id",
    tag = "Consumers",
    responses (
        (status = 200, body = ApiConsumer)
    )
)]
pub async fn find_consumer_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiConsumer>, ResultErrors> {
    let response = match repositories::consumers::find_by_id(&app_state.pool, id).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(response));
}

#[utoipa::path(
    put,
    path = "/consumers/{id}",
    operation_id = "update_consumer",
    tag = "Consumers",
    responses (
        (status = 200, body = ApiConsumer)
    )
)]
pub async fn update_consumer(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<ConsumerFormDto>,
) -> Result<Json<ApiConsumer>, ResultErrors> {
    let response = match repositories::consumers::update(&app_state.pool, id, body).await {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/consumers",
    operation_id = "find_consumers",
    tag = "Consumers",
    params (
        PaginationQueryDto
    ),
    responses (
        (status = 200, body = ConsumersPagination)
    )
)]
pub async fn find_consumers(
    State(app_state): State<AppState>,
    pagination: Query<PaginationQueryDto>,
) -> Result<Json<ConsumersPagination>, ResultErrors> {
    let pagination = pagination.0;

    let response = match repositories::consumers::find_and_count(&app_state.pool, pagination).await
    {
        Ok(response) => response,
        Err(e) => return Err(e.into()),
    };

    return Ok(Json(PaginationResponseDto {
        items: response.0,
        count: response.1,
    }));
}
