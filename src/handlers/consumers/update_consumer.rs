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
    infra::repositories::consumers_repository::{self, NewConsumer},
};

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
) -> Result<Json<ResultBodyContainerConsumer>, ResultErrors> {
    let response = consumers_repository::update(&app_state.pool, id, body)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
