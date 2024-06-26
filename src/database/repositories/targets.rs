use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::{
    api::dtos::pagination::PaginationQueryDto,
    database::{
        entities::targets::{NewTarget, Target},
        errors::InfraError,
        get_pool_connection,
    },
    schema::targets::{self, host},
};

use diesel::ExpressionMethods;

use super::extract_interact_error;

pub async fn create(pool: &Pool, body: NewTarget) -> Result<Target, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::insert_into(targets::table)
                    .values(body)
                    .returning(Target::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn update(
    pool: &Pool,
    id: i32,
    upstream_id: i32,
    body: NewTarget,
) -> Result<Target, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::update(targets::dsl::targets)
                    .filter(targets::id.eq(id).and(targets::upstream_id.eq(upstream_id)))
                    .set((host.eq(body.host), targets::port.eq(body.port)))
                    .returning(Target::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn find_by_id(pool: &Pool, id: i32, upstream_id: i32) -> Result<Target, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                targets::table
                    .filter(targets::id.eq(id).and(targets::upstream_id.eq(upstream_id)))
                    .select(Target::as_select())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn delete(pool: &Pool, id: i32, upstream_id: i32) -> Result<Target, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::delete(targets::dsl::targets)
                    .filter(targets::id.eq(id).and(targets::upstream_id.eq(upstream_id)))
                    .returning(Target::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn find_and_count(
    pool: &Pool,
    upstream_id: i32,
    pagination: PaginationQueryDto,
) -> Result<(Vec<Target>, i64), InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let count_filter = pagination.text.clone();

    let list = match extract_interact_error(
        manager
            .interact(move |conn| {
                let mut query = targets::table
                    .into_boxed()
                    .filter(targets::upstream_id.eq(upstream_id));

                match pagination.text {
                    Some(text) => {
                        query = query.filter(targets::host.like(format!("%{text}%")));
                    }
                    None => {}
                };

                query = query.offset(pagination.offset).limit(pagination.limit);

                query.load(conn)
            })
            .await,
    ) {
        Ok(list) => list,
        Err(e) => return Err(e),
    };

    match extract_interact_error(
        manager
            .interact(move |conn| {
                let mut query = targets::table
                    .into_boxed()
                    .filter(targets::upstream_id.eq(upstream_id));

                match count_filter {
                    Some(text) => {
                        query = query.filter(targets::host.like(format!("%{text}%")));
                    }
                    None => {}
                };

                query.count().get_result(conn)
            })
            .await,
    ) {
        Ok(count) => Ok((list, count)),
        Err(e) => Err(e),
    }
}
