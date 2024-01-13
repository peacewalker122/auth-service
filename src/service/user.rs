use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    http::{
        request::user::CreateUserDTO,
        response::{
            user::{MFAResponse, UserDTO},
            BaseResponse,
        },
    },
    model::ModelManager,
    pkg::{hmac::HMAC, hotp::Hotp, util::rand::generate_random_string},
    repository::user::UserRepository,
};

use super::{error::Result, ServiceError};

#[derive(Debug, Clone)]
pub struct UserService {}

impl UserService {
    pub async fn create_user(mm: &ModelManager, req: &CreateUserDTO) -> Result<UserDTO> {
        let hash_password = hash(req.password.as_bytes(), DEFAULT_COST)?;

        let user = UserRepository::create(
            Ctx::root_ctx(),
            mm,
            CreateUserDTO {
                email: req.email.to_owned(),
                name: req.name.to_owned(),
                password: hash_password,
                auth_provider: None,
                auth_provider_user_id: None,
                secret: None,
            },
        )
        .await?;

        let token = Uuid::new_v4();

        Ok(user.into_dto(
            Some(token.to_string()),
            Some("supposethisisrefreshtoken".to_string()),
            None,
        ))
    }

    // TODO: handle when user already use mfa
    pub async fn login(mm: &ModelManager, email: String, password: String) -> Result<UserDTO> {
        let Some(user) = UserRepository::get_by_email(Ctx::root_ctx(), mm, &email).await? else {
            return Err(ServiceError::NotFound(
                "couldn't find corresponding user".to_string(),
            ));
        };

        // if user.password.is_empty() {
        //     return Err(ServiceError::ForbiddenWithMessage(String::from(
        //         "wrong endpoint, should try oauth",
        //     )));
        // }

        if user.secret.is_some() {
            return Ok(user.into_dto(None, None, Some("TOTP".to_string())));
        }

        let is_match = verify(password.as_bytes(), &user.password)?;

        if !is_match {
            return Err(ServiceError::Unauthorized);
        }

        // let's say this is the jwt token.
        let token = Uuid::new_v4();

        Ok(user.into_dto(
            Some(token.to_string()),
            Some("supposethisisrefreshtoken".to_string()),
            None,
        ))
    }

    pub async fn set_mfa(mm: &ModelManager, ctx: &Ctx) -> Result<BaseResponse<MFAResponse>> {
        let mut user = UserRepository::get_by_id(Ctx::root_ctx(), mm, ctx.user_id() as i64).await?;

        user.secret = Some(generate_random_string(20));

        UserRepository::update(Ctx::root_ctx(), mm, &(ctx.user_id() as i64), &user).await?;

        let hotp = Hotp::new(
            Some(HMAC::HMACSHA256),
            "authservice",
            &user.email,
            user.secret.unwrap().as_ref(),
            6,
        );

        Ok(BaseResponse::new(
            200,
            MFAResponse {
                url: hotp.get_url(),
            },
        ))
    }
}
