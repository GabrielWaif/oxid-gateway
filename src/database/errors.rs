use std::fmt;

use deadpool_diesel::InteractError;

pub type Result<T> = std::result::Result<T, InfraError>;

#[derive(Debug)]
pub enum InfraError {
    InternalServerError,
    NotFound,
    DatabaseConflict,
}

pub fn adapt_infra_error<T: AsInfraError>(error: T) -> InfraError {
    error.as_infra_error()
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfraError::NotFound => write!(f, "Not found"),
            InfraError::InternalServerError => write!(f, "Internal server error"),
            InfraError::DatabaseConflict => write!(f, "Conflict"),
        }
    }
}

pub trait AsInfraError {
    fn as_infra_error(&self) -> InfraError;
}

impl AsInfraError for diesel::result::Error {
    fn as_infra_error(&self) -> InfraError {
        match self {
            diesel::result::Error::NotFound => InfraError::NotFound,
            diesel::result::Error::DatabaseError(database_error, _message) => {
                match database_error {
                    diesel::result::DatabaseErrorKind::NotNullViolation
                    | diesel::result::DatabaseErrorKind::UniqueViolation
                    | diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                        InfraError::DatabaseConflict
                    }
                    _ => InfraError::InternalServerError,
                }
            }
            _ => InfraError::InternalServerError,
        }
    }
}

impl AsInfraError for deadpool_diesel::PoolError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError
    }
}

impl AsInfraError for InteractError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError
    }
}
