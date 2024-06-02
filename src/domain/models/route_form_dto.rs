use serde::Deserialize;
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize)]
pub struct RouteFormDto {
    pub name: String,
    pub path: String,
    pub inner_path: String,
}
