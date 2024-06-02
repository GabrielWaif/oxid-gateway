use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerConsumer},
    },
    infra::repositories::consumers_repository,
};

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
) -> Result<Json<ResultBodyContainerConsumer>, ResultErrors> {
    let response = consumers_repository::delete(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
