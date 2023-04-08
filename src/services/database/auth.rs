use sqlx::{Pool, Postgres};

use crate::{
    services::models::auth::*,
    utils::{config::Config, errors::AppErrors},
};

struct Input {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Auth {
    pub pool: Pool<Postgres>,
}

impl Auth {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait::async_trait]
impl AuthStore for Auth {
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
            if crate::utils::auth::verify_password(&login.password, &res.password)? {
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

    async fn logout(&self, _token: String) -> Result<(), AppErrors> {
        todo!()
    }
}
