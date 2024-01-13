use axum::{http::StatusCode, response::IntoResponse, Json};

use thiserror::Error;

use crate::http::ApiError;

pub type Result<T> = core::result::Result<T, ServiceError>;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("authentication is required to access this resource")]
    Unauthorized,
    #[error("username or password is incorrect")]
    InvalidLoginAttmpt,
    #[error("user does not have privilege to access this resource")]
    Forbidden,
    #[error("user does not have privilege to access this resource: {0}")]
    ForbiddenWithMessage(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    ApplicationStartup(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("unexpected error has occurred")]
    InternalServerError,
    #[error("{0}")]
    InternalServerErrorWithContext(String),
    #[error("{0}")]
    ObjectConflict(String),
    #[error("unprocessable request has occurred")]
    UnprocessableEntity { errors: String },
    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerErrorWithContext(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::ObjectConflict(err) => (StatusCode::CONFLICT, err),
            Self::InvalidLoginAttmpt => (
                StatusCode::BAD_REQUEST,
                Self::InvalidLoginAttmpt.to_string(),
            ),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
            Self::AnyhowError(err) => match err.to_string().contains("unique constraint") {
                true => (StatusCode::BAD_REQUEST, err.to_string()),
                false => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            },
            Self::ForbiddenWithMessage(err) => (StatusCode::FORBIDDEN, err),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("unexpected error {:?}", self),
            ),
        };

        // I'm not a fan of the error specification, so for the sake of consistency,
        // serialize singular errors as a map of vectors similar to the 422 validation responses
        let body = Json(ApiError::new(error_message));

        (status, body).into_response()
    }
}
