use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone)]
#[diesel(table_name = crate::schema::consumers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Consumer {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::consumers)]
pub struct NewConsumer {
    pub username: String,
    pub password: String,
}
