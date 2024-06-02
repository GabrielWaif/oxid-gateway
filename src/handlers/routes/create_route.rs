use axum::{extract::{Path, State}, http::StatusCode, Json};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors, result_body_container::{ResultBodyContainer, ResultBodyContainerRoute, ResultBodyContainerTarget}, route_form_dto::RouteFormDto, target_form_dto::TargetFormDto
    },
    infra::repositories::{routes_repository::{self, NewRoute}, targets_repository::{self, NewTarget}},
};

#[utoipa::path(
    post,
    path = "/upstreams/{upstream_id}/routes",
    operation_id = "create_route",
    tag = "Routes",
    responses (
        (status = 201, body = ResultBodyContainerTarget)
    )
)]
pub async fn create_route(
    Path(upstream_id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<RouteFormDto>,
) -> Result<(StatusCode, Json<ResultBodyContainerRoute>), ResultErrors> {
    let new_route = NewRoute { 
        name: body.name,
        path: body.path,
        inner_path: body.inner_path,
        upstream_id
    };

    let response = routes_repository::create(&app_state.pool, new_route)
        .await
        .unwrap();

    return Ok((
        StatusCode::CREATED,
        Json(ResultBodyContainer::success(response)),
    ));
}
