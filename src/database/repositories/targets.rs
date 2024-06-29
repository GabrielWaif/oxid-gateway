use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::{
    api::dtos::pagination::PaginationQueryDto,
    database::{
        entities::targets::{NewTarget, Target},
        errors::{adapt_infra_error, InfraError},
        get_pool_connection,
    },
    schema::targets::{self, host, name},
};

use diesel::ExpressionMethods;

pub async fn create(pool: &Pool, body: NewTarget) -> Result<Target, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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

pub async fn delete(pool: &Pool, id: i32) -> Result<Target, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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

pub async fn find_and_count(
    pool: &Pool,
    pagination: PaginationQueryDto,
) -> Result<(Vec<Target>, i64), InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let count_filter = pagination.text.clone();

    let list = manager
        .interact(move |conn| {
            let mut query = targets::table.into_boxed();

            match pagination.text {
                Some(text) => {
                    query = query.filter(targets::host.like(format!("%{text}%")));
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
            let mut query = targets::table.into_boxed();

            match count_filter {
                Some(text) => {
                    query = query.filter(targets::host.like(format!("%{text}%")));
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
