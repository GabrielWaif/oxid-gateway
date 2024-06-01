use axum::{extract::State, http::StatusCode, Json};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerUpstream},
    },
    infra::repositories::{
        upstream_repository::{self, NewUpstream},
    },
};

#[utoipa::path(
    post,
    path = "/upstreams",
    operation_id = "create_upstream",
    tag = "Upstreams",
    responses (
        (status = 201, body = ResultBodyContainerUpstream)
    )
)]
pub async fn create_upstream(
    State(app_state): State<AppState>,
    Json(body): Json<NewUpstream>,
) -> Result<(StatusCode, Json<ResultBodyContainerUpstream>), ResultErrors> {
    let response = upstream_repository::create(&app_state.pool, body)
        .await
        .unwrap();

    return Ok((
        StatusCode::CREATED,
        Json(ResultBodyContainer::success(response)),
    ));
}
