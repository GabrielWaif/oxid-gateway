use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone)]
#[diesel(table_name = crate::schema::target)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Target {
    pub id: i32,
    pub name: String,
    pub host: String,
    pub port: i32,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::target)]
pub struct NewTarget {
    pub name: String,
    pub host: String,
    pub port: i32,
}

// #[derive(Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone)]
// #[diesel(table_name = crate::schema::upstreams)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Upstream {
//     pub id: i32,
//     pub name: String,
//     pub host: String,
//     pub port: i32,
// }
// 
// #[derive(Queryable, Insertable, Deserialize)]
// #[diesel(table_name = crate::schema::upstreams)]
// pub struct NewUpstream {
//     pub name: String,
//     pub host: String,
//     pub port: i32,
// }
// 
// #[derive(Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone)]
// #[diesel(table_name = crate::schema::routes)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Route {
//     pub id: i32,
//     pub name: String,
//     pub path: String,
// }
// 
// #[derive(Queryable, Insertable, Deserialize)]
// #[diesel(table_name = crate::schema::routes)]
// pub struct NewRoute {
//     pub name: String,
//     pub host: String,
//     pub port: i32,
// }
