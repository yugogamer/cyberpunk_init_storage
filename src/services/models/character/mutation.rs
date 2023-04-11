use juniper::FieldResult;
use juniper_compose::composable_object;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

use crate::{
    controller::graphql::GraphqlContext,
    services::models::{
        access::can_edit_character,
        character::{Character, InputCharacter, UpdateCharacter},
    },
};

#[derive(Default)]
pub struct CharacterMutation;

#[composable_object]
#[juniper::graphql_object(Context = GraphqlContext)]
impl CharacterMutation {
    async fn create_character(
        character: InputCharacter,
        ctx: &GraphqlContext,
    ) -> FieldResult<Character> {
        let character = entities::characters::ActiveModel {
            name: Set(character.name),
            user_id: Set(ctx.user_id),
            base_ref: Set(character.base_ref),
            modifier: Set(character.modifier),
            ..Default::default()
        }
        .insert(&ctx.db.database)
        .await?;
        Ok(character.into())
    }

    async fn update_character(
        character_id: i32,
        input_character: UpdateCharacter,
        ctx: &GraphqlContext,
    ) -> FieldResult<Character> {
        if can_edit_character(character_id, ctx.user_id, &ctx.db.database).await? {
            return Err("You can't edit this character".into());
        }

        let character = entities::characters::Entity::find_by_id(character_id)
            .one(&ctx.db.database)
            .await?;
        if let Some(character) = character {
            let mut character = character.into_active_model();
            if let Some(name) = input_character.name {
                character.name = Set(name);
            }
            if let Some(base_ref) = input_character.base_ref {
                character.base_ref = Set(base_ref);
            }
            if let Some(modifier) = input_character.modifier {
                character.modifier = Set(modifier);
            }
            let character = character.update(&ctx.db.database).await?;
            return Ok(character.into());
        } else {
            return Err("Character not found".into());
        }
    }

    async fn delete_character(character_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        entities::characters::Entity::delete_by_id(character_id)
            .exec(&ctx.db.database)
            .await?;
        Ok(true)
    }
}
