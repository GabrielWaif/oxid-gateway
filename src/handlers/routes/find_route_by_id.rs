use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerRoute},
    },
    infra::repositories::routes_repository,
};

#[utoipa::path(
    get,
    path = "/upstreams/{upstream_id}/routes/{id}",
    operation_id = "find_route_by_id",
    tag = "Routes",
    responses (
        (status = 200, body = ResultBodyContainerRoute)
    )
)]
pub async fn find_route_by_id(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
) -> Result<Json<ResultBodyContainerRoute>, ResultErrors> {
    let response = routes_repository::find_by_id(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
