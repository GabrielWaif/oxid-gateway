use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use axum::http::StatusCode;

use crate::{
    database::{
        entities::routes::{NewRoute, Route},
        errors::{adapt_infra_error, InfraError},
        get_pool_connection,
    },
    schema::routes,
};

use diesel::ExpressionMethods;

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

pub async fn find(pool: &Pool, offset: i64, limit: i64) -> Result<Vec<Route>, InfraError> {
    let manager = get_pool_connection(pool).await;

    let res = manager
        .interact(move |conn| {
            routes::table
                .select(Route::as_select())
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
