use crate::utils::{config::Config, errors::AppErrors};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[async_trait]
pub trait AuthStore: Sync {
    async fn login(&self, login: Login, config: &Config) -> Result<String, AppErrors>;
    async fn logout(&self, token: String) -> Result<(), AppErrors>;
}
