use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState, database_utils::get_pool_connection, models::{NewTarget, Target}, schema::target::{self, host, name}
};

use diesel::{  query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl, SelectableHelper};

pub async fn create_target(
    State(app_state): State<AppState>,
    Json(body): Json<NewTarget>,
) -> Result<Json<Target>, (StatusCode, String)> {
    let manager = get_pool_connection(&app_state.pool).await;

    let res = manager
        .interact(move |conn| {
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

pub async fn update_target(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<NewTarget>,
) -> Result<Json<Target>, (StatusCode, String)> {
    let manager = get_pool_connection(&app_state.pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::update(target::dsl::target)
                .filter(target::id.eq(id))
                .set((
                    name.eq(body.name),
                    host.eq(body.host),
                    target::port.eq(body.port)
                  ))
                .returning(Target::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    Ok(Json(res))
}

pub async fn get_target(
    State(app_state): State<AppState>,
    Path(id): Path<i32>
) -> Result<Json<Target>, (StatusCode, String)> {
    let manager = get_pool_connection(&app_state.pool).await;

    let res = manager
        .interact(move |conn| {
            target::table
                .filter(target::id.eq(id))
                .select(Target::as_select())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    Ok(Json(res))
}

pub async fn delete_target(
    State(app_state): State<AppState>,
    Path(id): Path<i32>
) -> Result<Json<Target>, (StatusCode, String)> {
    let manager = get_pool_connection(&app_state.pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::delete(target::dsl::target)
                .filter(target::id.eq(id))
                .returning(Target::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    Ok(Json(res))
}
