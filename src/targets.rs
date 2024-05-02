use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState, database_utils::get_pool_connection, models::{NewTarget, Target}, schema::target
};

use diesel::{ RunQueryDsl, SelectableHelper};

pub async fn create_target(
    State(app_state): State<AppState>,
    Json(body): Json<NewTarget>,
) -> Result<Json<Target>, (StatusCode, String)> {
    let manager = get_pool_connection(&app_state.pool).await;

    let res = manager
        .interact(|conn| {
            diesel::insert_into(target::table)
                .values(body)
                .returning(Target::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    Ok(Json(res))
}
