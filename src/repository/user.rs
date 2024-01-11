use crate::{
    ctx::Ctx,
    http::request::user::CreateUserDTO,
    model::{user::User, ModelManager},
};

#[derive(Debug, Clone)]
pub struct UserRepository {}

impl UserRepository {
    pub async fn create(_ctx: Ctx, mm: &ModelManager, req: CreateUserDTO) -> anyhow::Result<User> {
        let user: User = sqlx::query_as(
            r#"INSERT INTO users (name,email,password,created_at,auth_provider,auth_provider_user_id) VALUES ($1, $2, $3, current_timestamp, $4,$5) RETURNING *"#,
        )
            .bind(req.name)
            .bind(req.email)
            .bind(req.password)
            .bind(req.auth_provider)
            .bind(req.auth_provider_user_id)
            .fetch_one(&mm.db)
            .await?;

        Ok(user)
    }

    pub async fn get_by_email(
        _ctx: Ctx,
        mm: &ModelManager,
        email: &str,
    ) -> anyhow::Result<Option<User>> {
        let user: Option<User> =
            sqlx::query_as("SELECT * FROM users where email = $1 AND deleted_at IS NULL")
                .bind(email)
                .fetch_optional(&mm.db)
                .await?;

        Ok(user)
    }

    pub async fn update(
        _ctx: Ctx,
        mm: &ModelManager,
        id: &i64,
        model: &User,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE users
                SET
                    modified_at = current_timestamp,
                    name = $2,
                    email = $3,
                    auth_provider = COALESCE(auth_provider, $4),
                    auth_provider_user_id = COALESCE(auth_provider_user_id, $5)
                WHERE id = $1;
            "#,
        )
        .bind(id)
        .bind(&model.name)
        .bind(&model.email)
        .bind(&model.auth_provider)
        .bind(&model.auth_provider_user_id)
        .execute(&mm.db)
        .await?;

        Ok(())
    }
}
