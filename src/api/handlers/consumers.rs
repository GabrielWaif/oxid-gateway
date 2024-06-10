use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    api::{errors::ResultErrors, AppState},
    database::{
        entities::consumers::{Consumer, NewConsumer},
        repositories,
    },
};

#[utoipa::path(
    post,
    path = "/consumers",
    operation_id = "create_consumer",
    tag = "Consumers",
    responses (
        (status = 201, body = ResultBodyContainerConsumer)
    )
)]
pub async fn create_consumer(
    State(app_state): State<AppState>,
    Json(body): Json<NewConsumer>,
) -> Result<(StatusCode, Json<Consumer>), ResultErrors> {
    let response = repositories::consumers::create(&app_state.pool, body)
        .await
        .unwrap();

    return Ok((StatusCode::CREATED, Json(response)));
}

#[utoipa::path(
    delete,
    path = "/consumers/{id}",
    operation_id = "delete_consumer",
    tag = "Consumers",
    responses (
        (status = 200, body = ResultBodyContainerConsumer)
    )
)]
pub async fn delete_consumer(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Consumer>, ResultErrors> {
    let response = repositories::consumers::delete(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}

#[utoipa::path(
    get,
    path = "/consumers/{id}",
    operation_id = "find_consumer_by_id",
    tag = "Consumers",
    responses (
        (status = 200, body = ResultBodyContainerConsumer)
    )
)]
pub async fn find_consumer_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Consumer>, ResultErrors> {
    let response = repositories::consumers::find_by_id(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}

#[utoipa::path(
    put,
    path = "/consumers/{id}",
    operation_id = "update_consumer",
    tag = "Consumers",
    responses (
        (status = 200, body = ResultBodyContainerConsumer)
    )
)]
pub async fn update_consumer(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<NewConsumer>,
) -> Result<Json<Consumer>, ResultErrors> {
    let response = repositories::consumers::update(&app_state.pool, id, body)
        .await
        .unwrap();

    return Ok(Json(response));
}
