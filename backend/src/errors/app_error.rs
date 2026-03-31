use derive_more::derive::Display;
use diesel::result::Error as DieselError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("{_0}")]
    NotFound(String),

    #[display("{_0}")]
    BadRequest(String),

    #[display("{_0}")]
    Unauthorized(String),

    #[display("{_0}")]
    Forbidden(String),

    #[display("{_0}")]
    Conflict(String),

    #[display("{_0}")]
    Validation(String),

    #[display("{_0}")]
    Database(DieselError),

    #[display("{_0}")]
    Internal(String),
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Database(e) => Some(e),
            _ => None,
        }
    }
}
