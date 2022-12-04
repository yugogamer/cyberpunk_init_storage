use crate::utils::config::Config;
use crate::utils::errors::AppErrors;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[async_trait]
pub trait UserStore {
    async fn get_user(&self, id: Uuid) -> Result<User, AppErrors>;
    async fn create_user(&self, user: InputUser, config: &Config) -> Result<User, AppErrors>;
    async fn update_user(&self, user: User) -> Result<User, AppErrors>;
    async fn delete_user(&self, id: Uuid) -> Result<(), AppErrors>;
}
