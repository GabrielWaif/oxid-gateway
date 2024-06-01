use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerTarget}, target_form_dto::TargetFormDto,
    },
    infra::repositories::targets_repository::{self, NewTarget},
};

#[utoipa::path(
    put,
    path = "/upstream/{upstream_id}/targets/{id}",
    operation_id = "update_target",
    tag = "Targets",
    responses (
        (status = 200, body = ResultBodyContainerTarget)
    )
)]
pub async fn update_target(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
    Json(body): Json<TargetFormDto>,
) -> Result<Json<ResultBodyContainerTarget>, ResultErrors> {
    // TODO: Check if target is part of upstream first
    let new_target = NewTarget { 
        name: body.name, 
        host: body.host, 
        port: body.port, 
        upstream_id
    };

    let response = targets_repository::update(&app_state.pool, id, new_target)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
