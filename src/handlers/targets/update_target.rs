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
    infra::repositories::targets_repository::{self, NewTarget},
};


#[utoipa::path(
    put,
    path = "/targets/{id}",
    operation_id = "update_target",
    tag = "Targets",
    responses (
        (status = 200, body = ResultBodyContainerTarget)
    )
)]
pub async fn update_target(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<NewTarget>,
) -> Result<Json<ResultBodyContainerTarget>, ResultErrors> {
    let response = targets_repository::update(&app_state.pool, id, body)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
