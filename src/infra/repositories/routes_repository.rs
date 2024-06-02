use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;
use utoipa::ToSchema;

use crate::{database_utils::get_pool_connection, schema::routes};

use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema)]
#[diesel(belongs_to(Upstream))]
#[diesel(table_name = crate::schema::routes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Route {
    pub id: i32,
    pub upstream_id: i32,
    pub name: String,
    pub path: String,
    pub inner_path: String,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::routes)]
pub struct NewRoute {
    pub upstream_id: i32,
    pub name: String,
    pub path: String,
    pub inner_path: String,
}

pub async fn create(pool: &Pool, body: NewRoute) -> Result<Route, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::insert_into(routes::table)
                .values(body)
                .returning(Route::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn update(pool: &Pool, id: i32, body: NewRoute) -> Result<Route, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::update(routes::dsl::routes)
                .filter(routes::id.eq(id))
                .set((
                    routes::name.eq(body.name),
                    routes::path.eq(body.path),
                    routes::inner_path.eq(body.inner_path),
                ))
                .returning(Route::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn find_by_id(pool: &Pool, id: i32) -> Result<Route, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            routes::table
                .filter(routes::id.eq(id))
                .select(Route::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}

pub async fn delete(pool: &Pool, id: i32) -> Result<Route, (StatusCode, String)> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::delete(routes::dsl::routes)
                .filter(routes::id.eq(id))
                .returning(Route::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}
