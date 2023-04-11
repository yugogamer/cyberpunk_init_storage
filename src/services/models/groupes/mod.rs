use crate::services::models::database::DatabaseTrait;
use crate::{controller::graphql::GraphqlContext, utils::errors::AppErrors};
use async_trait::async_trait;
use juniper::graphql_object;
use sea_orm::{ModelTrait};
use serde::{Deserialize, Serialize};

use super::{character::Character, user::User};

pub mod mutation;
pub mod query;

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct InputGroupe {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Groupe {
    model: entities::groupes::Model,
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<entities::groupes::Model> for Groupe {
    fn from(input: entities::groupes::Model) -> Self {
        Self {
            model: input.clone(),
            id: input.id,
            name: input.name,
            owner_id: input.owner_id,
            created_at: input.created_at,
            updated_at: input.updated_at,
        }
    }
}

#[graphql_object(Context = GraphqlContext)]
impl Groupe {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }

    async fn owner(&self, ctx: &GraphqlContext) -> juniper::FieldResult<User> {
        let owner = self
            .model
            .find_related(entities::accounts::Entity)
            .one(&ctx.db.database)
            .await?
            .unwrap();
        Ok(owner.into())
    }

    async fn characters(&self, ctx: &GraphqlContext) -> juniper::FieldResult<Vec<Character>> {
        let characters = self
            .model
            .find_related(entities::characters::Entity)
            .all(&ctx.db.database)
            .await?
            .into_iter()
            .map(|character| character.into())
            .collect();
        Ok(characters)
    }

    fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }
    fn updated_at(&self) -> chrono::NaiveDateTime {
        self.updated_at
    }
}

#[async_trait]
pub trait GroupeStore: Sync + Send {
    async fn get_groupe(&self, id: i32) -> Result<Groupe, AppErrors>;
    async fn get_groupe_secured(&self, id: i32, owner_id: i32) -> Result<Groupe, AppErrors>;
    async fn get_groupe_by_owner(&self, id: i32) -> Result<Vec<Groupe>, AppErrors>;
    async fn create_groupe(&self, groupe: InputGroupe, owner_id: i32) -> Result<Groupe, AppErrors>;
    async fn update_groupe(&self, groupe: Groupe) -> Result<Groupe, AppErrors>;
    async fn delete_groupe(&self, id: i32) -> Result<(), AppErrors>;
}
