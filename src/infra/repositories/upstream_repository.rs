use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;
use utoipa::ToSchema;

use crate::{
    database_utils::get_pool_connection,
    schema::upstreams::{self, name},
};

use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(
    Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema,
)]
#[diesel(table_name = crate::schema::upstreams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Upstream {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::upstreams)]
pub struct NewUpstream {
    pub name: String,
}

pub async fn create(pool: &Pool, body: NewUpstream) -> Result<Upstream, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::insert_into(upstreams::table)
                .values(body)
                .returning(Upstream::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn update(pool: &Pool, id: i32, body: NewUpstream) -> Result<Upstream, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::update(upstreams::dsl::upstreams)
                .filter(upstreams::id.eq(id))
                .set((name.eq(body.name),))
                .returning(Upstream::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn find_by_id(pool: &Pool, id: i32) -> Result<Upstream, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            upstreams::table
                .filter(upstreams::id.eq(id))
                .select(Upstream::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}

pub async fn delete(pool: &Pool, id: i32) -> Result<Upstream, (StatusCode, String)> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            diesel::delete(upstreams::dsl::upstreams)
                .filter(upstreams::id.eq(id))
                .returning(Upstream::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}
