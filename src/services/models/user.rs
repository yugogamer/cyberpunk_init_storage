use crate::utils::errors::AppErrors;
use crate::{utils::config::Config};
use async_trait::async_trait;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<entities::accounts::Model> for User {
    fn from(input: entities::accounts::Model) -> Self {
        Self {
            id: input.id,
            email: input.email,
            username: input.username,
            created_at: input.created_at,
            updated_at: input.updated_at,
        }
    }
}

#[async_trait]
pub trait UserStore: Sync + Send {
    async fn get_user(&self, id: i32) -> Result<User, AppErrors>;
    async fn create_user(&self, user: InputUser, config: &Config) -> Result<User, AppErrors>;
    async fn update_user(&self, user: User) -> Result<User, AppErrors>;
    async fn delete_user(&self, id: i32) -> Result<(), AppErrors>;
}
