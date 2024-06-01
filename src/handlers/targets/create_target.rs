use axum::{extract::{Path, State}, http::StatusCode, Json};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerTarget}, target_form_dto::TargetFormDto,
    },
    infra::repositories::targets_repository::{self, NewTarget},
};

#[utoipa::path(
    post,
    path = "/upstream/{upstream_id}/targets",
    operation_id = "create_target",
    tag = "Targets",
    responses (
        (status = 201, body = ResultBodyContainerTarget)
    )
)]
pub async fn create_target(
    Path(upstream_id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<TargetFormDto>,
) -> Result<(StatusCode, Json<ResultBodyContainerTarget>), ResultErrors> {
    let new_target = NewTarget { 
        name: body.name, 
        host: body.host, 
        port: body.port, 
        upstream_id
    };

    let response = targets_repository::create(&app_state.pool, new_target)
        .await
        .unwrap();

    return Ok((
        StatusCode::CREATED,
        Json(ResultBodyContainer::success(response)),
    ));
}
