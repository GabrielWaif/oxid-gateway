use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    infra::repositories::targets_repository::{self, Target},
};

pub async fn find_target_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Target>, String> {
    let response = targets_repository::find_by_id(&app_state.pool, id)
        .await
        .unwrap();

    return Ok(Json(response));
}
