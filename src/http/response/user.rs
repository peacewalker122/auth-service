use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: i64,
    // pub created_at: OffsetDateTime,
    // pub modified_at: Option<OffsetDateTime>,
    // pub deleted_at: Option<OffsetDateTime>,
    pub name: String,
    pub email: String,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
}
