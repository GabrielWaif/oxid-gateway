use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;
use utoipa::ToSchema;

use crate::{
    database_utils::get_pool_connection,
    schema::targets::{self, host, name},
};

use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(
    Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema,
)]
#[diesel(table_name = crate::schema::targets)]
#[diesel(belongs_to(Upstream))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Target {
    pub id: i32,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub upstream_id: i32,
}

#[derive(Queryable, Insertable, Deserialize, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::targets)]
pub struct NewTarget {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub upstream_id: i32,
}

pub async fn create(pool: &Pool, body: NewTarget) -> Result<Target, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::insert_into(targets::table)
                .values(body)
                .returning(Target::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn update(pool: &Pool, id: i32, body: NewTarget) -> Result<Target, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::update(targets::dsl::targets)
                .filter(targets::id.eq(id))
                .set((
                    name.eq(body.name),
                    host.eq(body.host),
                    targets::port.eq(body.port),
                ))
                .returning(Target::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn find_by_id(pool: &Pool, id: i32) -> Result<Target, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            targets::table
                .filter(targets::id.eq(id))
                .select(Target::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}

pub async fn delete(pool: &Pool, id: i32) -> Result<Target, (StatusCode, String)> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::delete(targets::dsl::targets)
                .filter(targets::id.eq(id))
                .returning(Target::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}
