use serde::Deserialize;
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize)]
pub struct RouteFormDto {
    pub path: String,
    pub private: bool,
    pub inner_path: String,
}
