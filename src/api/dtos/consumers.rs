use serde::Deserialize;
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize)]
pub struct ConsumerFormDto {
    pub name: String,
}
