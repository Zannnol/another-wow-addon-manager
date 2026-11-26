use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error : {0}")]
    Database(#[from] rusqlite::Error),

    #[error("I/O error : {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error : {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid path or configuration : {0}")]
    Validation(String),

    #[error("Resource Not Found : {0}")]
    NotFound(String),

    #[error("Internal error : {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct SerializedError {
    code: &'static str,
    message: String,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let code = match self {
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Io(_) => "IO_ERROR",
            AppError::Json(_) => "JSON_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Internal(_) => "INTERNAL_ERROR",
        };

        SerializedError {
            code,
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}

pub type AppResult<T> = Result<T, AppError>;