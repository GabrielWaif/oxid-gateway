use axum::{extract::State, http::StatusCode, Json};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerTarget},
    },
    infra::repositories::targets_repository::{self, NewTarget},
};

#[utoipa::path(
    post,
    path = "/targets",
    operation_id = "create_target",
    tag = "Targets",
    responses (
        (status = 201, body = ResultBodyContainerTarget)
    )
)]
pub async fn create_target(
    State(app_state): State<AppState>,
    Json(body): Json<NewTarget>,
) -> Result<(StatusCode, Json<ResultBodyContainerTarget>), ResultErrors> {
    let response = targets_repository::create(&app_state.pool, body)
        .await
        .unwrap();

    return Ok((
        StatusCode::CREATED,
        Json(ResultBodyContainer::success(response)),
    ));
}
