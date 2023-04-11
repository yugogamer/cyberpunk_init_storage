use juniper::FieldResult;
use juniper_compose::composable_object;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    controller::graphql::GraphqlContext,
    services::models::{
        access::{can_access_groupe, can_edit_character, can_edit_groupe},
        groupes::{Groupe, InputGroupe},
    },
};

#[derive(Default)]
pub struct GroupesMutation;

#[composable_object]
#[juniper::graphql_object(Context = GraphqlContext)]
impl GroupesMutation {
    async fn create_groupe(groupe: InputGroupe, ctx: &GraphqlContext) -> FieldResult<Groupe> {
        let groupe = entities::groupes::ActiveModel {
            name: Set(groupe.name),
            owner_id: Set(ctx.user_id),
            ..Default::default()
        }
        .insert(&ctx.db.database)
        .await?;

        entities::groupes_access::ActiveModel {
            id_user: Set(ctx.user_id),
            id_groupe: Set(groupe.id),
            admin: Set(true),
            ..Default::default()
        }
        .insert(&ctx.db.database)
        .await?;
        Ok(groupe.into())
    }

    async fn assign_character_to_groupe(
        character_id: i32,
        groupe_id: i32,
        ctx: &GraphqlContext,
    ) -> FieldResult<bool> {
        if !can_access_groupe(groupe_id, ctx.user_id, &ctx.db.database).await? {
            return Err("You can't access this groupe".into());
        }
        if !can_edit_character(character_id, ctx.user_id, &ctx.db.database).await? {
            return Err("You can't edit this character".into());
        }

        entities::active_in_groups::ActiveModel {
            id_characters: Set(character_id),
            id_groupe: Set(groupe_id),
            active: Set(true),
            ..Default::default()
        }
        .insert(&ctx.db.database)
        .await?;
        Ok(true)
    }

    async fn remove_character_from_groupe(
        character_id: i32,
        groupe_id: i32,
        ctx: &GraphqlContext,
    ) -> FieldResult<bool> {
        if !can_edit_groupe(groupe_id, ctx.user_id, &ctx.db.database).await? {
            return Err("You can't edit this groupe".into());
        }

        entities::active_in_groups::Entity::delete_many()
            .filter(entities::active_in_groups::Column::IdCharacters.eq(character_id))
            .filter(entities::active_in_groups::Column::IdGroupe.eq(groupe_id))
            .exec(&ctx.db.database)
            .await?;
        Ok(true)
    }

    async fn delete_groupe(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        entities::groupes::Entity::delete_by_id(groupe_id)
            .exec(&ctx.db.database)
            .await?;
        Ok(true)
    }
}
