use juniper::FieldResult;
use juniper_compose::composable_object;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

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

    async fn change_status_in_groupe(
        character_id: i32,
        groupe_id: i32,
        active: bool,
        ctx: &GraphqlContext,
    ) -> FieldResult<bool> {
        if !can_edit_groupe(groupe_id, ctx.user_id, &ctx.db.database).await? {
            return Err("You can't edit this groupe".into());
        }
        let status = entities::active_in_groups::Entity::find_by_id((character_id, groupe_id))
            .one(&ctx.db.database)
            .await?;
        if let Some(status) = status {
            let mut status = status.into_active_model();
            status.set(entities::active_in_groups::Column::Active, active.into());
            status.update(&ctx.db.database).await?;
            return Ok(true);
        }

        Ok(false)
    }

    async fn delete_groupe(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        entities::groupes::Entity::delete_by_id(groupe_id)
            .exec(&ctx.db.database)
            .await?;
        Ok(true)
    }

    async fn invite_user(
        groupe_id: i32,
        user_name: String,
        ctx: &GraphqlContext,
    ) -> FieldResult<bool> {
        if !can_edit_groupe(groupe_id, ctx.user_id, &ctx.db.database).await? {
            return Err("You can't edit this groupe".into());
        }

        let other_user = entities::accounts::Entity::find()
            .filter(entities::accounts::Column::Username.eq(user_name))
            .one(&ctx.db.database)
            .await?;
        if let Some(other_user) = other_user {
            entities::invitations::ActiveModel {
                groupe_id: Set(Some(other_user.id)),
                user_id: Set(Some(groupe_id)),
                accepted: Set(false),
                ..Default::default()
            }
            .insert(&ctx.db.database)
            .await?;
        } else {
            return Err("user not found".into());
        }
        Ok(true)
    }

    async fn accept_invitation(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        let invitation = entities::invitations::Entity::find()
            .filter(entities::invitations::Column::GroupeId.eq(groupe_id))
            .filter(entities::invitations::Column::UserId.eq(ctx.user_id))
            .one(&ctx.db.database)
            .await?;
        if let Some(invitation) = invitation {
            let invitation = invitation.into_active_model();
            invitation.delete(&ctx.db.database).await?;
            entities::groupes_access::ActiveModel {
                id_user: Set(ctx.user_id),
                id_groupe: Set(groupe_id),
                admin: Set(false),
                ..Default::default()
            }
            .save(&ctx.db.database)
            .await?;
        }
        Ok(true)
    }

    async fn refuse_invitation(groupe_id: i32, ctx: &GraphqlContext) -> FieldResult<bool> {
        let invitation = entities::invitations::Entity::find()
            .filter(entities::invitations::Column::GroupeId.eq(groupe_id))
            .filter(entities::invitations::Column::UserId.eq(ctx.user_id))
            .one(&ctx.db.database)
            .await?;
        if let Some(invitation) = invitation {
            let invitation: entities::invitations::ActiveModel = invitation.into_active_model();
            invitation.delete(&ctx.db.database).await?;
        }
        Ok(true)
    }
}
