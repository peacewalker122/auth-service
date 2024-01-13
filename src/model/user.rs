use crate::http::response::user::UserDTO;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::time::OffsetDateTime};

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
    pub name: String,
    pub email: String,
    pub auth_provider: Option<String>,
    pub auth_provider_user_id: Option<String>,
    pub secret: Option<String>,
    pub password: String,
}

pub struct UserFilter {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTokenClaims {
    pub sub: u64,
    pub iat: usize,
    pub exp: usize,
}

impl From<User> for UserDTO {
    fn from(val: User) -> Self {
        UserDTO {
            id: val.id,
            name: val.name,
            email: val.email,
            token: None,
            refresh_token: None,
            mfa_type: None,
        }
    }
}

impl User {
    pub fn into_dto(
        self,
        token: Option<String>,
        refresh_token: Option<String>,
        mfa_type: Option<String>,
    ) -> UserDTO {
        UserDTO {
            id: self.id,
            name: self.name,
            email: self.email,
            token,
            refresh_token,
            mfa_type,
        }
    }
}
