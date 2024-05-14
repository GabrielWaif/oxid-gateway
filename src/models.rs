use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::infra::repositories::targets_repository::Target;

#[derive(Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone)]
#[diesel(table_name = crate::schema::upstreams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Upstream {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Insertable, Deserialize)]
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

#[derive(Queryable, Selectable, Serialize, Identifiable, AsChangeset, PartialEq, Clone)]
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
