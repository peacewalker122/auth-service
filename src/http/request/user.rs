use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
    pub password: String,

    pub auth_provider: Option<String>,
    pub auth_provider_user_id: Option<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}
