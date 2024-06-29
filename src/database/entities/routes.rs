use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(
    Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema, Debug, Queryable
)]
#[diesel(belongs_to(Upstream))]
#[diesel(table_name = crate::schema::routes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Route {
    pub id: i32,
    pub upstream_id: i32,
    pub name: String,
    pub path: String,
    pub inner_path: String,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::routes)]
pub struct NewRoute {
    pub upstream_id: i32,
    pub name: String,
    pub path: String,
    pub inner_path: String,
}
