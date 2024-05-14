use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    infra::repositories::targets_repository::{self, Target},
};

pub async fn delete_target(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Target>, String> {
    let response = targets_repository::delete(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}
