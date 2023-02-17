use crate::services::models::user::*;
use crate::utils::config::Config;

use super::Database;

#[async_trait::async_trait]
impl UserStore for Database {
    async fn get_user(&self, id: i32) -> Result<User, crate::utils::errors::AppErrors> {
        let res = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM accounts
            WHERE id = $1
            "#,
            id
        );
        let res = res.fetch_one(&self.pool).await?;
        Ok(res)
    }

    async fn create_user(
        &self,
        user: InputUser,
        config: &Config,
    ) -> Result<User, crate::utils::errors::AppErrors> {
        let mut transaction = self.pool.begin().await?;
        let res = sqlx::query_as!(
            User,
            r#"
            INSERT INTO accounts (username, email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            user.username,
            user.email
        )
        .fetch_one(&mut transaction)
        .await?;
        sqlx::query!(
            r#"
            INSERT INTO auth (user_id, password)
            VALUES ($1, $2)
            "#,
            res.id,
            crate::utils::auth::hash_password(&user.password, &config.argon2_config)?
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(res)
    }

    async fn update_user(&self, user: User) -> Result<User, crate::utils::errors::AppErrors> {
        let res = sqlx::query_as!(
            User,
            r#"
            UPDATE accounts
            SET username = $1, email = $2
            WHERE id = $3
            RETURNING *
            "#,
            user.username,
            user.email,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(res)
    }

    async fn delete_user(&self, id: i32) -> Result<(), crate::utils::errors::AppErrors> {
        sqlx::query!(
            r#"
            DELETE FROM accounts
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
