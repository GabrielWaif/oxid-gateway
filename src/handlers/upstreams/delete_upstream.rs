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
    delete,
    path = "/upstreams/{id}",
    operation_id = "delete_upstream",
    tag = "Upstreams",
    responses (
        (status = 200, body = ResultBodyContainerUpstream)
    )
)]
pub async fn delete_upstream(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<ResultBodyContainerUpstream>, ResultErrors> {
    let response = upstream_repository::delete(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
