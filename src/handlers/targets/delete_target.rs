use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::{error::ResultErrors, result_body_container::{ResultBodyContainer, ResultBodyContainerTarget}},
    infra::repositories::targets_repository,
};

#[utoipa::path(
    delete,
    path = "/targets/{id}",
    operation_id = "delete_target",
    tag = "Targets",
    responses (
        (status = 200, body = ResultBodyContainerTarget)
    )
)]
pub async fn delete_target(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<ResultBodyContainerTarget>, ResultErrors> {
    let response = targets_repository::delete(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
