use std::env;

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::info;

use crate::{ctx::Ctx, http::Error, model::user::CustomTokenClaims};

pub async fn jwt_auth(mut request: Request, next: Next) -> Result<Response, Error> {
    #[allow(non_snake_case)]
    let JWTKEY: String = env::var("JWT_SECRET").expect("JWT_SECRET was not set in the environment");

    // get the token first
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header[7..].to_owned())
            } else {
                None
            }
        });

    let token = token.ok_or_else(|| Error {
        status_code: StatusCode::UNAUTHORIZED,
        message: String::from("Please login first"),
    })?;

    // authenticate the token
    let claims = decode::<CustomTokenClaims>(
        token.as_ref(),
        &DecodingKey::from_secret(JWTKEY.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        info!("Error decoding JWT: {}", e);
        Error {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Invalid token"),
        }
    })?
    .claims;

    // save it into the context.
    let ctx = Ctx::new(claims.sub).map_err(|_| Error {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "failed to create context".to_string(),
    })?;

    request.extensions_mut().insert(ctx);

    Ok(next.run(request).await)
}
