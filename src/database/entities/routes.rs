use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(
    Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema,
)]
#[diesel(belongs_to(Upstream))]
#[diesel(table_name = crate::schema::routes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Route {
    pub id: i32,
    pub path: String,
    pub private: bool,
    pub inner_path: Option<String>,
    pub upstream_id: i32,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::routes)]
pub struct NewRoute {
    pub upstream_id: i32,
    pub path: String,
    pub private: bool,
    pub inner_path: Option<String>,
}
