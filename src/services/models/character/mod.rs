use crate::{controller::graphql::GraphqlContext, utils::errors::AppErrors};
use async_trait::async_trait;
use juniper::{graphql_object, FieldResult};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};

pub mod mutation;
pub mod query;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    model: entities::characters::Model,
    pub group_id: Option<i32>,
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub base_ref: i32,
    pub modifier: i32,
    pub asset_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<entities::characters::Model> for Character {
    fn from(input: entities::characters::Model) -> Self {
        Self {
            model: input.clone(),
            id: input.id,
            name: input.name,
            user_id: input.user_id,
            base_ref: input.base_ref,
            modifier: input.modifier,
            asset_id: input.asset_id,
            created_at: input.created_at,
            updated_at: input.updated_at,
            group_id: None,
        }
    }
}

#[graphql_object(Context = GraphqlContext)]
impl Character {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn user_id(&self) -> i32 {
        self.user_id
    }
    fn base_ref(&self) -> i32 {
        self.base_ref
    }
    fn modifier(&self) -> i32 {
        self.modifier
    }
    async fn asset_url(&self, ctx: &GraphqlContext) -> Option<String> {
        if let Some(_asset_id) = self.asset_id {
            let asset = self
                .model
                .find_related(entities::assets::Entity)
                .one(&ctx.db.database)
                .await
                .unwrap()
                .unwrap();
            let url = ctx.storage.signe_download(&asset.bucket_name).await;
            Some(url)
        } else {
            None
        }
    }
    fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }
    fn updated_at(&self) -> chrono::NaiveDateTime {
        self.updated_at
    }

    async fn active(&self, ctx: &GraphqlContext) -> FieldResult<Option<bool>> {
        if let Some(group_id) = self.group_id {
            let active = entities::active_in_groups::Entity::find()
                .filter(entities::active_in_groups::Column::IdCharacters.eq(self.id))
                .filter(entities::active_in_groups::Column::IdGroupe.eq(group_id))
                .one(&ctx.db.database)
                .await?;
            if let Some(active) = active {
                return Ok(Some(active.active));
            }
        }
        Ok(None)
    }
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
