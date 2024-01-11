use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    http::{request::user::CreateUserDTO, response::user::UserDTO},
    model::ModelManager,
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
            },
        )
        .await?;

        let token = Uuid::new_v4();

        Ok(user.into_dto(token.to_string(), "supposethisisrefreshtoken".to_string()))
    }

    // TODO: adjust the logic behind to handle cases were user signup from the google oauth
    pub async fn login(mm: &ModelManager, email: String, password: String) -> Result<UserDTO> {
        let Some(user) = UserRepository::get_by_email(Ctx::root_ctx(), mm, &email).await? else {
            return Err(ServiceError::NotFound(
                "couldn't find corresponding user".to_string(),
            ));
        };

        let is_match = verify(password.as_bytes(), &user.password)?;

        if !is_match {
            return Err(ServiceError::Unauthorized);
        }

        // let's say this is the jwt token.
        let token = Uuid::new_v4();

        Ok(user.into_dto(token.to_string(), "supposethisisrefreshtoken".to_string()))
    }
}
