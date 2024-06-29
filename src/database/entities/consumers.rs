use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(
    Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone, ToSchema, Debug
)]
#[diesel(table_name = crate::schema::api_consumers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Consumer {
    pub id: i32,
    pub name: String,
    pub api_key: String,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::api_consumers)]
pub struct NewConsumer {
    pub name: String,
    pub api_key: String,
}
