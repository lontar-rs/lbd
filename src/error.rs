use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("config error: {0}")]
    Config(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("search error: {0}")]
    Search(String),
    #[error("startup error: {0}")]
    Startup(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Config(msg) | AppError::Startup(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::Search(msg) => (StatusCode::BAD_GATEWAY, msg).into_response(),
        }
    }
}
