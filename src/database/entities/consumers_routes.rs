use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

use super::routes::Route;
use super::consumers::ApiConsumer;

#[derive(Identifiable, Selectable, Insertable, Queryable, Associations, Serialize, Debug, ToSchema)]
#[diesel(belongs_to(Route))]
#[diesel(belongs_to(ApiConsumer))]
#[diesel(table_name = crate::schema::api_consumers_routes)]
#[diesel(primary_key(api_consumer_id, route_id))]
pub struct ConsumerRoute {
    pub api_consumer_id: i32,
    pub route_id: i32,
}
