use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::{
    api::dtos::pagination::PaginationQueryDto, database::{
        entities::upstreams::{NewUpstream, Upstream},
        errors::{adapt_infra_error, InfraError},
        get_pool_connection,
    }, schema::upstreams::{self, name}
};

use diesel::ExpressionMethods;

pub async fn create(pool: &Pool, body: NewUpstream) -> Result<Upstream, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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

pub async fn delete(pool: &Pool, id: i32) -> Result<Upstream, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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

pub async fn find_and_count(
    pool: &Pool,
    pagination: PaginationQueryDto,
) -> Result<(Vec<Upstream>, i64), InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let count_filter = pagination.text.clone();

    let list = manager
        .interact(move |conn| {
            let mut query = upstreams::table.into_boxed();

            match pagination.text {
                Some(text) => {
                    query = query.filter(upstreams::name.like(format!("%{text}%")));
                }
                None => {}
            };

            query = query.offset(pagination.offset).limit(pagination.limit);

            query.load(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    let count = manager
        .interact(move |conn| {
            let mut query = upstreams::table.into_boxed();

            match count_filter {
                Some(text) => {
                    query = query.filter(upstreams::name.like(format!("%{text}%")));
                }
                None => {}
            };

            query.count().get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok((list, count));
}
