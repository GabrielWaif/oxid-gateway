use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerUpstream},
    },
    infra::repositories::upstream_repository,
};

#[utoipa::path(
    get,
    path = "/upstreams/{id}",
    operation_id = "find_upstream_by_id",
    tag = "Upstreams",
    responses (
        (status = 200, body = ResultBodyContainerUpstream)
    )
)]
pub async fn find_upstream_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<ResultBodyContainerUpstream>, ResultErrors> {
    let response = upstream_repository::find_by_id(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
