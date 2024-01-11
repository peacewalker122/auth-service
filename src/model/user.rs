use super::ModelManager;
use crate::{
    ctx::Ctx,
    http::{request::user::CreateUserDTO, response::user::UserDTO},
};
use sqlx::{prelude::FromRow, types::time::OffsetDateTime};

use crate::model::error::{Error, Result};

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
    pub password: String,
}

pub struct UserFilter {
    pub email: String,
}

impl Into<UserDTO> for User {
    fn into(self) -> UserDTO {
        return UserDTO {
            id: self.id,
            name: self.name,
            email: self.email,
            token: None,
            refresh_token: None,
        };
    }
}

impl User {
    pub fn into_dto(self, token: String, refresh_token: String) -> UserDTO {
        UserDTO {
            id: self.id,
            name: self.name,
            email: self.email,
            token: Some(token),
            refresh_token: Some(refresh_token),
        }
    }
}

pub struct UsersBMC {}

impl UsersBMC {
    pub async fn create(_ctx: Ctx, mm: &ModelManager, data: &CreateUserDTO) -> Result<i64> {
        let db = mm.db();

        // fetch one return a tuple.
        let (id,) = sqlx::query_as(
            r#"INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING id"#,
        )
        .bind(&data.name)
        .bind(&data.email)
        .bind(&data.password)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn get_by_email(_ctx: Ctx, mm: &ModelManager, f: &UserFilter) -> Result<User> {
        let db = mm.db();

        let data: User = sqlx::query_as(r#"SELECT * FROM users WHERE lower(email) = $1"#)
            .bind(&f.email)
            .fetch_one(db)
            .await?;

        Ok(data)
    }
}
