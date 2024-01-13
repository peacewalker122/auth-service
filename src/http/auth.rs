use crate::{
    ctx::Ctx,
    model::ModelManager,
    service::{self, auth, user::UserService},
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};

use super::request::{
    google::AuthRequest,
    user::{CreateUserDTO, LoginDTO},
};
use axum_extra::extract::cookie::CookieJar;

// TODO: Add request validation & sanitize request validation to prevent sql injection.
// ref: https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx/blob/main/crates/conduit-api/src/extractors/validation_extractor.rs
pub async fn create_user(
    State(mm): State<ModelManager>,
    Json(payload): Json<CreateUserDTO>,
) -> service::Result<impl IntoResponse> {
    let user = UserService::create_user(
        &mm,
        &CreateUserDTO {
            name: payload.name,
            email: payload.email,
            password: payload.password,
            auth_provider: None,
            auth_provider_user_id: None,
            secret: None,
        },
    )
    .await?;

    Ok(Json(user))
}

pub async fn login(
    State(mm): State<ModelManager>,
    Json(payload): Json<LoginDTO>,
) -> service::Result<impl IntoResponse> {
    let user = UserService::login(&mm, payload.email, payload.password).await?;

    Ok(Json(user))
}

pub async fn google_oauth_login() -> service::Result<impl IntoResponse> {
    let resp = auth::google::login().await?;

    Ok(resp)
}

pub async fn google_oauth_callback(
    State(mm): State<ModelManager>,
    cookies: CookieJar,
    Query(payload): Query<AuthRequest>,
) -> service::Result<impl IntoResponse> {
    let resp = auth::google::callback(mm, cookies, payload).await?;

    Ok(resp)
}

pub async fn allow_mfa(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
) -> service::Result<impl IntoResponse> {
    let resp = UserService::set_mfa(&mm, &ctx).await?;

    Ok(Json(resp))
}
