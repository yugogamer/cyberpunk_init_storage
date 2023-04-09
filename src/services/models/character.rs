use crate::utils::errors::AppErrors;
use async_trait::async_trait;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct InputCharacter {
    pub name: String,
    pub base_ref: i32,
    pub modifier: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateCharacter {
    pub name: Option<String>,
    pub user_id: Option<i32>,
    pub base_ref: Option<i32>,
    pub modifier: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub base_ref: i32,
    pub modifier: i32,
    pub asset_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[async_trait]
pub trait CharacterStore: Sync + Send {
    async fn get_character(&self, id: i32) -> Result<Character, AppErrors>;
    async fn get_active_character_in_group(&self, id: i32) -> Result<Vec<Character>, AppErrors>;
    async fn get_character_by_group(&self, id: i32) -> Result<Vec<Character>, AppErrors>;
    async fn get_character_by_user(&self, id: i32) -> Result<Vec<Character>, AppErrors>;
    async fn create_character(
        &self,
        character: InputCharacter,
        user_id: i32,
    ) -> Result<Character, AppErrors>;
    async fn update_character(
        &self,
        character: UpdateCharacter,
        id_character: i32,
    ) -> Result<Character, AppErrors>;
    async fn delete_character(&self, id: i32) -> Result<(), AppErrors>;
}
