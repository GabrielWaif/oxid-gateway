use serde::Deserialize;
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize)]
pub struct RouteFormDto {
    pub private: bool,
    pub path: String,
    pub inner_path: Option<String>,
}
