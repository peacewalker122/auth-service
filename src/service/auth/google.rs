use std::env;

use ::cookie::time::Duration;
use anyhow::Context;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use tracing::info;

use crate::{
    ctx::Ctx,
    http::request::{google::AuthRequest, user::CreateUserDTO},
    model::ModelManager,
    repository::user::UserRepository,
    service::{
        self,
        constant::{COOKIE_AUTH_CODE_VERIFIER, COOKIE_AUTH_CSRF_STATE, GOOGLE_OAUTH_PROVIDER},
    },
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub sub: String,
    pub name: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub picture: String,
}

fn get_oauth_client() -> anyhow::Result<BasicClient> {
    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("Missing CLIENT_ID env var"));

    let client_secret = Some(ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET").expect("Missing CLIENT_SECRET env var"),
    ));

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?;

    let base_url = env::var("BASE_URL").context("Missing BASE_URL env var")?;
    let redirect_url = RedirectUrl::new(format!("{}/google/oauth/callback", base_url))?;

    let client = BasicClient::new(client_id, client_secret, auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    Ok(client)
}

pub async fn login() -> service::Result<impl IntoResponse> {
    let client = get_oauth_client()?;

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let csrf_cookie: Cookie = Cookie::build((COOKIE_AUTH_CSRF_STATE, csrf_state.secret().clone()))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .max_age(Duration::minutes(5))
        .into();

    let code_veifier: Cookie = Cookie::build((
        COOKIE_AUTH_CODE_VERIFIER,
        pkce_code_verifier.secret().clone(),
    ))
    .http_only(true)
    .path("/")
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(5))
    .into();

    let cookies = CookieJar::new().add(csrf_cookie).add(code_veifier);

    let response = (cookies, Redirect::to(authorize_url.as_str()));

    Ok(response)
}

pub async fn callback(
    mm: ModelManager,
    cookies: CookieJar,
    req: AuthRequest,
) -> service::Result<impl IntoResponse> {
    let stored_state = cookies.get(COOKIE_AUTH_CSRF_STATE);
    let stored_code_verifier = cookies.get(COOKIE_AUTH_CODE_VERIFIER);

    let (Some(csrf_state), Some(code_verifier)) = (stored_state, stored_code_verifier) else {
        info!("missing csrf state or code verifier");
        return Err(service::ServiceError::BadRequest(
            StatusCode::BAD_REQUEST.to_string(),
        ));
    };

    if csrf_state.value() != req.state {
        info!("csrf state not match");
        return Err(service::ServiceError::BadRequest(
            StatusCode::BAD_REQUEST.to_string(),
        ));
    }

    // this code below were logic for "token" use after user authenticate it.
    let client = get_oauth_client()?;
    let code = AuthorizationCode::new(req.code);
    let pkce_code_verifier = PkceCodeVerifier::new(code_verifier.value().to_string());

    // this code below should implement the "code" exchange
    // reference: https://developers.google.com/identity/protocols/oauth2/web-server#httprest_5
    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .context("error exchanging code for token")?;

    // this code below should implement the api fetch to get what we want
    // ie: fetch user information.
    let user_info = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .context("error fetching user info")?
        .json::<GoogleUser>()
        .await
        .context("error parsing user info")?;

    let Some(email) = user_info.email else {
        info!("user info {:?}", user_info);
        return Err(service::ServiceError::InternalServerError);
    };

    let existing_user = UserRepository::get_by_email(Ctx::root_ctx(), &mm, &email).await?;

    let user = match existing_user {
        Some(mut x) => {
            if x.auth_provider.is_none() {
                x.auth_provider = Some(GOOGLE_OAUTH_PROVIDER.to_string());
                x.auth_provider_user_id = Some(user_info.sub);
                UserRepository::update(Ctx::root_ctx(), &mm, &x.id, &x).await?;
                x
            } else {
                x
            }
        }
        None => {
            // TODO: make sure in login api, the oauth user were redirected into google account.
            let user = UserRepository::create(
                Ctx::root_ctx(),
                &mm,
                CreateUserDTO {
                    name: user_info.name,
                    email,
                    password: "".to_string(),
                    auth_provider: Some(GOOGLE_OAUTH_PROVIDER.to_string()),
                    auth_provider_user_id: Some(user_info.sub),
                },
            )
            .await?;

            user
        }
    };

    // enough for authentication proccess here's the authorization process
    let auth_code = Uuid::new_v4();

    Ok((
        StatusCode::OK,
        Json(user.into_dto(
            auth_code.to_string(),
            "supposethisisrefreshtoken".to_string(),
        )),
    ))
}
