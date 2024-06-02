use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;
use utoipa::ToSchema;

use crate::{database_utils::get_pool_connection, schema::consumers};

use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(
    Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema,
)]
#[diesel(table_name = crate::schema::consumers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Consumer {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::consumers)]
pub struct NewConsumer {
    pub username: String,
    pub password: String,
}

pub async fn create(pool: &Pool, body: NewConsumer) -> Result<Consumer, InfraError> {
    let manager = get_pool_connection(pool).await;

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
    let manager = get_pool_connection(pool).await;

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
    let manager = get_pool_connection(pool).await;

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

pub async fn delete(pool: &Pool, id: i32) -> Result<Consumer, (StatusCode, String)> {
    let manager = get_pool_connection(pool).await;

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
