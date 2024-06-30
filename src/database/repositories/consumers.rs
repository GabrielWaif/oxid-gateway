use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::{
    api::dtos::pagination::PaginationQueryDto,
    database::{
        entities::{
            consumers::{ApiConsumer, NewConsumer},
            consumers_routes::ConsumerRoute,
            routes::Route,
        },
        errors::InfraError,
        get_pool_connection,
    },
    schema::{self, api_consumers, api_consumers_routes},
};

use super::extract_interact_error;

pub async fn create(pool: &Pool, body: NewConsumer) -> Result<ApiConsumer, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::insert_into(api_consumers::table)
                    .values(body)
                    .returning(ApiConsumer::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn update(pool: &Pool, id: i32, body: NewConsumer) -> Result<ApiConsumer, InfraError> {
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
                    .returning(ApiConsumer::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn find_by_id(pool: &Pool, id: i32) -> Result<ApiConsumer, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                api_consumers::table
                    .filter(api_consumers::id.eq(id))
                    .select(ApiConsumer::as_select())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn delete(pool: &Pool, id: i32) -> Result<ApiConsumer, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::delete(api_consumers::dsl::api_consumers)
                    .filter(api_consumers::id.eq(id))
                    .returning(ApiConsumer::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}

pub async fn find_and_count(
    pool: &Pool,
    pagination: PaginationQueryDto,
) -> Result<(Vec<ApiConsumer>, i64), InfraError> {
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

pub async fn find_and_count_routes(
    pool: &Pool,
    id: i32,
    pagination: PaginationQueryDto,
) -> Result<(Vec<Route>, i64), InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let consumer = match find_by_id(pool, id).await {
        Ok(consumer) => consumer,
        Err(e) => return Err(e),
    };

    let count_consumer = consumer.clone();
    let count_filter = pagination.text.clone();

    let list = match extract_interact_error(
        manager
            .interact(move |conn| {
                let mut query = ConsumerRoute::belonging_to(&consumer)
                    .into_boxed()
                    .inner_join(schema::routes::table)
                    .select(Route::as_select());

                match pagination.text {
                    Some(text) => {
                        query = query.filter(schema::routes::path.like(format!("%{text}%")));
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
                let mut query = ConsumerRoute::belonging_to(&count_consumer)
                    .into_boxed()
                    .inner_join(schema::routes::table)
                    .select(Route::as_select());

                match count_filter {
                    Some(text) => {
                        query = query.filter(schema::routes::path.like(format!("%{text}%")));
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

pub async fn link_consumer_to_route(
    pool: &Pool,
    consumer_id: i32,
    route_id: i32,
) -> Result<ConsumerRoute, InfraError> {
    let manager = match get_pool_connection(pool).await {
        Ok(manager) => manager,
        Err(e) => return Err(e),
    };

    let relation = ConsumerRoute {
        api_consumer_id: consumer_id,
        route_id,
    };

    extract_interact_error(
        manager
            .interact(move |conn| {
                diesel::insert_into(api_consumers_routes::table)
                    .values(relation)
                    .returning(ConsumerRoute::as_returning())
                    .get_result(conn)
            })
            .await,
    )
}
