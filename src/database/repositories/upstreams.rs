use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use axum::http::StatusCode;

use crate::{
    database::{
        entities::upstreams::{NewUpstream, Upstream},
        errors::{adapt_infra_error, InfraError},
        get_pool_connection,
    },
    schema::upstreams::{self, name},
};

use diesel::ExpressionMethods;

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

pub async fn find(pool: &Pool, offset: i64, limit: i64) -> Result<Vec<Upstream>, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            upstreams::table
                .select(Upstream::as_select())
                .offset(offset)
                .limit(limit)
                .get_results(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}
