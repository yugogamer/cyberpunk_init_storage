use crate::utils::errors::AppErrors;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputGroupe {
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Groupe {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[async_trait]
pub trait GroupeStore {
    async fn get_groupe(&self, id: Uuid) -> Result<Groupe, AppErrors>;
    async fn get_groupe_by_owner(&self, id: Uuid) -> Result<Vec<Groupe>, AppErrors>;
    async fn create_groupe(&self, groupe: InputGroupe) -> Result<Groupe, AppErrors>;
    async fn update_groupe(&self, groupe: Groupe) -> Result<Groupe, AppErrors>;
    async fn delete_groupe(&self, id: Uuid) -> Result<(), AppErrors>;
}
