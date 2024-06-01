use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;
use utoipa::ToSchema;

use crate::{
    database_utils::get_pool_connection,
    schema::targets::{self, host, name},
};

use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::infra::errors::{adapt_infra_error, InfraError};
use crate::infra::repositories::targets_repository::Target;

#[derive(
    Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema,
)]
#[diesel(table_name = crate::schema::upstreams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Upstream {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::upstreams)]
pub struct NewUpstream {
    pub name: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(Upstream))]
#[diesel(belongs_to(Target))]
#[diesel(table_name = crate::schema::target_upstream)]
#[diesel(primary_key(upstream_id, target_id))]
pub struct UpstreamTarget {
    pub target_id: i32,
    pub upstream_id: i32,
}

// pub async fn create(pool: &Pool, body: NewUpstream) -> Result<Target, InfraError> {
//     let manager = get_pool_connection(pool).await;
//
//     let res = manager
//         .interact(move |conn| {
//             diesel::insert_into(targets::table)
//                 .values(body)
//                 .returning(Target::as_returning())
//                 .get_result(conn)
//         })
//         .await
//         .unwrap()
//         .unwrap();
//
//     return Ok(res);
// }
//
// pub async fn update(pool: &Pool, id: i32, body: NewTarget) -> Result<Target, InfraError> {
//     let manager = get_pool_connection(pool).await;
//
//     let res = manager
//         .interact(move |conn| {
//             diesel::update(targets::dsl::targets)
//                 .filter(targets::id.eq(id))
//                 .set((
//                     name.eq(body.name),
//                     host.eq(body.host),
//                     targets::port.eq(body.port),
//                 ))
//                 .returning(Target::as_returning())
//                 .get_result(conn)
//         })
//         .await
//         .unwrap()
//         .unwrap();
//
//     return Ok(res);
// }
//
// pub async fn find_by_id(pool: &Pool, id: i32) -> Result<Target, InfraError> {
//     let manager = get_pool_connection(pool).await;
//
//     let res = manager
//         .interact(move |conn| {
//             targets::table
//                 .filter(targets::id.eq(id))
//                 .select(Target::as_select())
//                 .get_result(conn)
//         })
//         .await
//         .map_err(adapt_infra_error)
//         .unwrap()
//         .map_err(adapt_infra_error)
//         .unwrap();
//
//     return Ok(res);
// }
//
// pub async fn delete(pool: &Pool, id: i32) -> Result<Target, (StatusCode, String)> {
//     let manager = get_pool_connection(pool).await;
//
//     let res = manager
//         .interact(move |conn| {
//             diesel::delete(targets::dsl::targets)
//                 .filter(targets::id.eq(id))
//                 .returning(Target::as_returning())
//                 .get_result(conn)
//         })
//         .await
//         .map_err(adapt_infra_error)
//         .unwrap()
//         .map_err(adapt_infra_error)
//         .unwrap();
//
//     return Ok(res);
// }
