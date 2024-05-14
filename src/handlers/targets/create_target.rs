use axum::{extract::State, Json};

use crate::{
    app_state::AppState,
    infra::repositories::targets_repository::{self, NewTarget, Target},
};

pub async fn create_target(
    State(app_state): State<AppState>,
    Json(body): Json<NewTarget>,
) -> Result<Json<Target>, String> {
    let response = targets_repository::create(&app_state.pool, body)
        .await
        .unwrap();

    return Ok(Json(response));
}
