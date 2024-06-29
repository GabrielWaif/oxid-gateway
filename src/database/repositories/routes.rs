use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::{
    api::dtos::pagination::PaginationQueryDto, database::{
        entities::routes::{NewRoute, Route},
        errors::{adapt_infra_error, InfraError},
        get_pool_connection,
    }, schema::routes
};

use diesel::ExpressionMethods;

pub async fn create(pool: &Pool, body: NewRoute) -> Result<Route, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };


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
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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

pub async fn delete(pool: &Pool, id: i32) -> Result<Route, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

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

pub async fn find_and_count(
    pool: &Pool,
    pagination: PaginationQueryDto,
) -> Result<(Vec<Route>, i64), InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let count_filter = pagination.text.clone();

    // TODO: Filter by text
    let list = manager
        .interact(move |conn| {
            routes::table
                .select(Route::as_select())
                .offset(pagination.offset)
                .limit(pagination.limit)
                .get_results(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    let count = manager
        .interact(move |conn| {
            let mut query = routes::table.into_boxed();

            match count_filter {
                Some(text) => {
                    query = query.filter(routes::name.like(format!("%{text}%")));
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
