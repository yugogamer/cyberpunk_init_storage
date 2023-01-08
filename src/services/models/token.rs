use crate::utils::errors::AppErrors;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    id: Uuid,
    token: String,
    user_id: Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[async_trait]
pub trait TokenStore {
    async fn get_token(&self, id: Uuid) -> Result<Token, AppErrors>;
    async fn get_token_by_user(&self, id: Uuid) -> Result<Vec<Token>, AppErrors>;
    async fn create_token(&self, user_id: Uuid, groupe_id: Uuid) -> Result<Token, AppErrors>;
    async fn update_token(&self, token: Token) -> Result<Token, AppErrors>;
    async fn delete_token(&self, id: Uuid) -> Result<(), AppErrors>;
}
