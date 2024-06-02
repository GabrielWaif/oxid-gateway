use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerRoute},
        route_form_dto::RouteFormDto,
    },
    infra::repositories::routes_repository::{self, NewRoute},
};

#[utoipa::path(
    put,
    path = "/upstreams/{upstream_id}/routes/{id}",
    operation_id = "update_route",
    tag = "Routes",
    responses (
        (status = 200, body = ResultBodyContainerRoute)
    )
)]
pub async fn update_route(
    Path((upstream_id, id)): Path<(i32, i32)>,
    State(app_state): State<AppState>,
    Json(body): Json<RouteFormDto>,
) -> Result<Json<ResultBodyContainerRoute>, ResultErrors> {
    // TODO: Check if target is part of upstream first
    let new_route = NewRoute {
        name: body.name,
        path: body.path,
        inner_path: body.inner_path,
        upstream_id,
    };

    let response = routes_repository::update(&app_state.pool, id, new_route)
        .await
        .unwrap();

    return Ok(Json(ResultBodyContainer::success(response)));
}
