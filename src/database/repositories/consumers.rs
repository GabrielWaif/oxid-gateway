use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::{
    api::dtos::pagination::PaginationQueryDto,
    database::{
        entities::consumers::{Consumer, NewConsumer},
        errors::InfraError,
        get_pool_connection,
    },
    schema::api_consumers,
};

use super::extract_interact_error;

pub async fn create(pool: &Pool, body: NewConsumer) -> Result<Consumer, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::insert_into(api_consumers::table)
                    .values(body)
                    .returning(Consumer::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn update(pool: &Pool, id: i32, body: NewConsumer) -> Result<Consumer, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::update(api_consumers::dsl::api_consumers)
                    .filter(api_consumers::id.eq(id))
                    .set((api_consumers::name.eq(body.name),))
                    .returning(Consumer::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn find_by_id(pool: &Pool, id: i32) -> Result<Consumer, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                api_consumers::table
                    .filter(api_consumers::id.eq(id))
                    .select(Consumer::as_select())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn delete(pool: &Pool, id: i32) -> Result<Consumer, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::delete(api_consumers::dsl::api_consumers)
                    .filter(api_consumers::id.eq(id))
                    .returning(Consumer::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn find_and_count(
    pool: &Pool,
    pagination: PaginationQueryDto,
) -> Result<(Vec<Consumer>, i64), InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let count_filter = pagination.text.clone();

    let list = match extract_interact_error(
        manager
            .interact(move |conn| {
                let mut query = api_consumers::table.into_boxed();

                match pagination.text {
                    Some(text) => {
                        query = query.filter(api_consumers::name.like(format!("%{text}%")));
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
                let mut query = api_consumers::table.into_boxed();

                match count_filter {
                    Some(text) => {
                        query = query.filter(api_consumers::name.like(format!("%{text}%")));
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
