use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::{
    database::{
        entities::consumers::{Consumer, NewConsumer},
        errors::{adapt_infra_error, InfraError},
        get_pool_connection,
    },
    schema::consumers,
};

use diesel::ExpressionMethods;

pub async fn create(pool: &Pool, body: NewConsumer) -> Result<Consumer, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let res = manager
        .interact(move |conn| {
            diesel::insert_into(consumers::table)
                .values(body)
                .returning(Consumer::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn update(pool: &Pool, id: i32, body: NewConsumer) -> Result<Consumer, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let res = manager
        .interact(move |conn| {
            diesel::update(consumers::dsl::consumers)
                .filter(consumers::id.eq(id))
                .set((
                    consumers::username.eq(body.username),
                    consumers::password.eq(body.password),
                ))
                .returning(Consumer::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    return Ok(res);
}

pub async fn find_by_id(pool: &Pool, id: i32) -> Result<Consumer, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let res = manager
        .interact(move |conn| {
            consumers::table
                .filter(consumers::id.eq(id))
                .select(Consumer::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}

pub async fn delete(pool: &Pool, id: i32) -> Result<Consumer, InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let res = manager
        .interact(move |conn| {
            diesel::delete(consumers::dsl::consumers)
                .filter(consumers::id.eq(id))
                .returning(Consumer::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok(res);
}

pub async fn find_and_count(pool: &Pool, offset: i64, limit: i64) -> Result<(Vec<Consumer>, i64), InfraError> {
    let manager =  match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let res = manager
        .interact(move |conn| {
            consumers::table
                .select(Consumer::as_select())
                .offset(offset)
                .limit(limit)
                .get_results(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    let count = manager
        .interact(move |conn| {
            consumers::table
                .count()
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)
        .unwrap()
        .map_err(adapt_infra_error)
        .unwrap();

    return Ok((res, count));
}
