use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerTarget},
    },
    infra::repositories::targets_repository,
};

#[utoipa::path(
    get,
    path = "/upstream/{upstream_id}/targets/{id}",
    operation_id = "find_target_by_id",
    tag = "Targets",
    responses (
        (status = 200, body = ResultBodyContainerTarget)
    )
)]
pub async fn find_target_by_id(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
) -> Result<Json<ResultBodyContainerTarget>, ResultErrors> {
    let response = targets_repository::find_by_id(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
