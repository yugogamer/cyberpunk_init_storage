use crate::utils::errors::AppErrors;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    id: i32,
    token: String,
    user_id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[async_trait]
pub trait TokenStore {
    async fn get_token(&self, id: i32) -> Result<Token, AppErrors>;
    async fn get_token_by_user(&self, id: i32) -> Result<Vec<Token>, AppErrors>;
    async fn create_token(&self, user_id: i32, groupe_id: i32) -> Result<Token, AppErrors>;
    async fn update_token(&self, token: Token) -> Result<Token, AppErrors>;
    async fn delete_token(&self, id: i32) -> Result<(), AppErrors>;
}
