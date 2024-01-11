use axum::{routing, Router};

use crate::model::ModelManager;

use self::auth::{create_user, google_oauth_callback, google_oauth_login, login};

mod auth;
mod error;

pub mod request;
pub mod response;

pub use self::error::{ApiError, Error};

pub fn new_router(mm: ModelManager) -> Router {
    Router::new()
        .route("/json", routing::post(create_user))
        .route("/login", routing::post(login))
        .route("/google/oauth/login", routing::get(google_oauth_login))
        .route(
            "/google/oauth/callback",
            routing::get(google_oauth_callback),
        )
        .with_state(mm)
}
