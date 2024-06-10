use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

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
