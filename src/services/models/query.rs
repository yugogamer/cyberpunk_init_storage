use juniper::{EmptySubscription, FieldResult, RootNode};

use crate::{
    controller::graphql::GraphqlContext,
    services::models::{
        character::{Character, InputCharacter},
        database::DatabaseTrait,
        groupes::Groupe,
        roll::roll_initiative,
    },
};

use super::{character::UpdateCharacter, groupes::InputGroupe, roll::CharacterRoll};

pub struct Query;

#[juniper::graphql_object(Context = GraphqlContext)]
impl Query {
    async fn get_groupe(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<Groupe> {
        let res = ctx
            .db
            .group_service()
            .get_groupe_secured(groupe_id, ctx.user_id)
            .await?;
        Ok(res)
    }

    async fn get_groupes(ctx: &GraphqlContext) -> FieldResult<Vec<Groupe>> {
        let res = ctx
            .db
            .group_service()
            .get_groupe_by_owner(ctx.user_id)
            .await?;
        Ok(res)
    }

    async fn my_groupes(ctx: &GraphqlContext) -> FieldResult<Vec<Groupe>> {
        let res = ctx
            .db
            .group_service()
            .get_groupe_by_owner(ctx.user_id)
            .await?;
        Ok(res)
    }

    async fn get_character(character_id: i32, ctx: &GraphqlContext) -> FieldResult<Character> {
        let res = ctx
            .db
            .character_service()
            .get_character(character_id)
            .await?;
        Ok(res)
    }

    async fn make_roll(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<Vec<CharacterRoll>> {
        let character = ctx
            .db
            .character_service()
            .get_active_character_in_group(groupe_id)
            .await?;

        let rolls = roll_initiative(&character);

        Ok(rolls)
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphqlContext)]
impl Mutation {
    async fn create_groupe(groupe: InputGroupe, ctx: &GraphqlContext) -> FieldResult<Groupe> {
        let res = ctx
            .db
            .group_service()
            .create_groupe(groupe, ctx.user_id)
            .await?;
        Ok(res)
    }

    async fn create_character(
        character: InputCharacter,
        ctx: &GraphqlContext,
    ) -> FieldResult<Character> {
        let res = ctx
            .db
            .character_service()
            .create_character(character, ctx.user_id)
            .await?;
        Ok(res)
    }

    async fn update_character(
        character_id: i32,
        character: UpdateCharacter,
        ctx: &GraphqlContext,
    ) -> FieldResult<Character> {
        let res = ctx
            .db
            .character_service()
            .update_character(character, character_id)
            .await?;
        Ok(res)
    }

    async fn delete_character(character_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        ctx.db
            .character_service()
            .delete_character(character_id)
            .await?;
        Ok(true)
    }

    async fn delete_groupe(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        ctx.db.group_service().delete_groupe(groupe_id).await?;
        Ok(true)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphqlContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
