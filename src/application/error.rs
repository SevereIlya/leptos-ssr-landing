use crate::domain::error::*;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn message_error(&self) -> String {
        match self {
            Self::Domain(domain_err) => domain_err.message_error().to_string(),
            Self::Database(_) | AppError::Config(_) | AppError::Internal(_) => {
                "На сервере технические неполадки".to_string()
            }
        }
    }
}
