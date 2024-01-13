use axum::response::IntoResponse;
use serde::Serialize;

pub mod user;

#[derive(Debug, Serialize)]
pub struct BaseResponse<T> {
    pub response_code: usize,
    pub data: T,
}

impl<T> BaseResponse<T> {
    pub fn new(response_code: usize, data: T) -> Self {
        BaseResponse {
            response_code,
            data,
        }
    }
}
