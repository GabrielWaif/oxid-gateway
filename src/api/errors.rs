use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::database::errors::InfraError;

#[derive(Debug)]
pub enum ResultErrors {
    InternalServerError,
    NotFound,
    Conflict,
    InfraError(InfraError),
}

impl IntoResponse for ResultErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                String::from("PostModel has not been found"),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            Self::Conflict => (
                StatusCode::CONFLICT,
                String::from("Conflict"),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };

        (status, Json(json!({ "error": vec![err_msg]}))).into_response()
    }
}

impl From<InfraError> for ResultErrors {
    fn from(value: InfraError) -> Self {
        match value {
            InfraError::InternalServerError => Self::InfraError(value),
            InfraError::NotFound => Self::NotFound,
            InfraError::DatabaseConflict => Self::Conflict,
        }
    }
}
