use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    infra::repositories::targets_repository::{self, NewTarget, Target},
};

pub async fn update_target(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(body): Json<NewTarget>,
) -> Result<Json<Target>, String> {
    let response = targets_repository::update(&app_state.pool, id, body)
        .await
        .unwrap();

    return Ok(Json(response));
}
