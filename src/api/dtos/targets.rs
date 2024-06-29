use serde::Deserialize;
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize)]
pub struct TargetFormDto {
    pub host: String,
    pub port: i32,
}
