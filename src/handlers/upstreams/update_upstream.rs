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
    infra::repositories::upstream_repository::{self, NewUpstream},
};

#[utoipa::path(
    put,
    path = "/upstreams/{id}",
    operation_id = "update_upstream",
    tag = "Upstreams",
    responses (
        (status = 200, body = ResultBodyContainerUpstream)
    )
)]
pub async fn update_upstream(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<NewUpstream>,
) -> Result<Json<ResultBodyContainerUpstream>, ResultErrors> {
    let response = upstream_repository::update(&app_state.pool, id, body)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
