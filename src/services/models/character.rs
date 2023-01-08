use crate::utils::errors::AppErrors;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputCharacter {
    pub name: String,
    pub user_id: Uuid,
    pub groupe_id: Uuid,
    pub base_ref: u32,
    pub modifier: u32,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub groupe_id: Uuid,
    pub base_ref: u32,
    pub modifier: u32,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[async_trait]
pub trait CharacterStore {
    async fn get_character(&self, id: Uuid) -> Result<Character, AppErrors>;
    async fn get_character_by_group(&self, id: Uuid) -> Result<Vec<Character>, AppErrors>;
    async fn get_character_by_user(&self, id: Uuid) -> Result<Vec<Character>, AppErrors>;
    async fn create_character(&self, character: InputCharacter) -> Result<Character, AppErrors>;
    async fn update_character(&self, character: Character) -> Result<Character, AppErrors>;
    async fn delete_character(&self, id: Uuid) -> Result<(), AppErrors>;
}
