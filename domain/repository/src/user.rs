use sqlx::PgPool;
use models::user::User;
use anyhow::Result;

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<User> {
        let row = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, status)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                username,
                email,
                password_hash,
                status,
                created_at,
                updated_at
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.status
        )
            .fetch_one(self.pool)
            .await?;
        Ok(row)
    }

    pub async fn by_id(&self, id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
            *
            FROM users WHERE id = $1
            "#,
            id
        )
            .fetch_optional(self.pool)
            .await?;
        Ok(user)
    }

    pub async fn change_password(&self, id: i32, new_password_hash: String) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            "#,
            new_password_hash,
            id
        )
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
