use juniper::{EmptySubscription, FieldResult, RootNode};

use crate::{
    controller::graphql::GraphqlContext,
    services::models::{
        character::{Character, InputCharacter},
        database::DatabaseTrait,
        groupes::Groupe,
        roll::roll_initiative,
    },
    utils::errors::AppErrors,
};
use sea_orm::{
    entity, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    ModelTrait, QueryFilter, QuerySelect, Related, RelationTrait, Set,
};

use super::{character::UpdateCharacter, groupes::InputGroupe, roll::CharacterRoll};

pub struct Query;

#[juniper::graphql_object(Context = GraphqlContext)]
impl Query {
    async fn get_groupe(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<Groupe> {
        if !can_access_groupe(groupe_id, ctx.user_id, &ctx.db.database).await? {
            return Err("You can't access this groupe".into());
        }

        let text = entities::groupes::Entity::find_by_id(groupe_id)
            .one(&ctx.db.database)
            .await?;
        if let Some(res) = text {
            return Ok(res.into());
        }
        Err("Groupe not found".into())
    }

    async fn get_groupes(ctx: &GraphqlContext) -> FieldResult<Vec<Groupe>> {
        let res = entities::accounts::Entity::find_by_id(ctx.user_id)
            .one(&ctx.db.database)
            .await?
            .unwrap()
            .find_related(entities::groupes::Entity)
            .all(&ctx.db.database)
            .await?;
        Ok(res.into_iter().map(|x| x.into()).collect())
    }

    async fn get_character(character_id: i32, ctx: &GraphqlContext) -> FieldResult<Character> {
        let res = entities::characters::Entity::find_by_id(character_id)
            .one(&ctx.db.database)
            .await?;
        if let Some(res) = res {
            return Ok(res.into());
        }
        Err("Groupe not found".into())
    }

    async fn make_roll(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<Vec<CharacterRoll>> {
        let groupe = entities::groupes::Entity::find_by_id(groupe_id)
            .one(&ctx.db.database)
            .await?;

        let character = groupe
            .unwrap()
            .find_related(entities::characters::Entity)
            .join(
                sea_orm::JoinType::InnerJoin,
                entities::active_in_groups::Relation::Characters.def(),
            )
            .filter(entities::active_in_groups::Column::Active.eq(true))
            .all(&ctx.db.database)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect();

        let rolls = roll_initiative(&character);

        Ok(rolls)
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphqlContext)]
impl Mutation {
    async fn create_groupe(groupe: InputGroupe, ctx: &GraphqlContext) -> FieldResult<Groupe> {
        let groupe = entities::groupes::ActiveModel {
            name: Set(groupe.name),
            owner_id: Set(ctx.user_id),
            ..Default::default()
        }
        .insert(&ctx.db.database)
        .await?;
        Ok(groupe.into())
    }

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

    async fn delete_groupe(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        entities::groupes::Entity::delete_by_id(groupe_id)
            .exec(&ctx.db.database)
            .await?;
        Ok(true)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphqlContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

async fn can_access_groupe(
    groupe_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, AppErrors> {
    let res = entities::groupes_access::Entity::find()
        .filter(entities::groupes_access::Column::IdGroupe.eq(groupe_id))
        .filter(entities::groupes_access::Column::IdUser.eq(user_id))
        .one(db)
        .await?;
    if let Some(res) = res {
        return Ok(true);
    }
    Ok(false)
}

async fn can_edit_character(
    character_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, AppErrors> {
    let res = entities::characters::Entity::find_by_id(character_id)
        .one(db)
        .await?;
    if let Some(res) = res {
        if res.user_id == user_id {
            return Ok(true);
        }
    }
    Ok(false)
}
