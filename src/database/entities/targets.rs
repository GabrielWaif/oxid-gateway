use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema, Queryable)]
#[diesel(belongs_to(Upstream))]
#[diesel(table_name = crate::schema::targets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Target {
    pub id: i32,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub upstream_id: i32,
}

#[derive(Queryable, Insertable, Deserialize, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::targets)]
pub struct NewTarget {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub upstream_id: i32,
}
