use axum::{middleware as axum_middleware, routing, Router};

use crate::model::ModelManager;

use self::{
    auth::{allow_mfa, create_user, google_oauth_callback, google_oauth_login, login},
    middleware::jwt::jwt_auth,
};

mod auth;
mod error;

pub mod middleware;
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
        .route(
            "/auth/allow/mfa",
            routing::patch(allow_mfa).route_layer(axum_middleware::from_fn(jwt_auth)),
        )
        .with_state(mm)
}
