use crate::{
    services::models::auth::*,
    utils::{config::Config, errors::AppErrors},
};

use super::Database;

struct Input {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[async_trait::async_trait]
impl AuthStore for Database {
    async fn login(&self, login: Login, config: &Config) -> Result<String, AppErrors> {
        let res = sqlx::query_as!(
            Input,
            r#"
            SELECT accounts.id, accounts.username, accounts.email, auth.password
            FROM auth
            INNER JOIN accounts ON auth.user_id = accounts.id
            WHERE accounts.email = $1
            "#,
            login.email
        )
        .fetch_optional(&self.pool)
        .await?;
        if let Some(res) = res {
            if crate::utils::auth::verify_password(&res.password, &login.password)? {
                let token = crate::utils::auth::generate_jwt(
                    &LightUser {
                        id: res.id,
                        username: res.username,
                        email: res.email,
                    },
                    &config.jwt_secret,
                )?;
                Ok(token)
            } else {
                Err(AppErrors::Unauthorized)
            }
        } else {
            Err(AppErrors::Unauthorized)
        }
    }

    async fn logout(&self, token: String) -> Result<(), AppErrors> {
        todo!()
    }
}
