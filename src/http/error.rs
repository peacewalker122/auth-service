use std::collections::HashMap;

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct Error {
    pub status_code: StatusCode,
    pub message: String,
}

impl Error {
    pub fn new(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code,
            message,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "status_code": self.status_code.as_u16(), "message": self.message })),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub errors: HashMap<String, Vec<String>>,
}

impl ApiError {
    pub fn new(error: String) -> Self {
        let mut error_map: HashMap<String, Vec<String>> = HashMap::new();
        error_map.insert("message".to_owned(), vec![error]);

        Self { errors: error_map }
    }
}
