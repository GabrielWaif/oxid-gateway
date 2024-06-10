use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::database::errors::InfraError;

#[derive(Debug)]
pub enum ResultErrors {
    InternalServerError,
    NotFound(i32),
    InfraError(InfraError),
}

impl IntoResponse for ResultErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("PostModel with id {} has not been found", id),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };

        (status, Json(json!({ "error": vec![err_msg]}))).into_response()
    }
}
