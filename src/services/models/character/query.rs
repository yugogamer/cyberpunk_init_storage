use juniper::FieldResult;
use juniper_compose::composable_object;
use sea_orm::{EntityTrait, ModelTrait};

use crate::controller::graphql::GraphqlContext;

#[derive(Default)]
pub struct CharacterQuery;

#[composable_object]
#[juniper::graphql_object(Context = GraphqlContext)]
impl CharacterQuery {
    async fn get_character(
        character_id: i32,
        ctx: &GraphqlContext,
    ) -> FieldResult<super::Character> {
        let res = entities::characters::Entity::find_by_id(character_id)
            .one(&ctx.db.database)
            .await?;
        if let Some(res) = res {
            return Ok(res.into());
        }
        Err("Groupe not found".into())
    }

    async fn my_character(ctx: &GraphqlContext) -> FieldResult<Vec<super::Character>> {
        let res = entities::accounts::Entity::find_by_id(ctx.user_id)
            .one(&ctx.db.database)
            .await?
            .unwrap()
            .find_related(entities::characters::Entity)
            .all(&ctx.db.database)
            .await?;
        Ok(res.into_iter().map(|x| x.into()).collect())
    }
}
