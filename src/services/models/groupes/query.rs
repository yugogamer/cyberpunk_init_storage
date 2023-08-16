use entities::{groupes::Relation, prelude::Groupes};
use juniper::FieldResult;
use juniper_compose::composable_object;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use crate::{
    controller::graphql::GraphqlContext,
    services::models::{access::can_access_groupe, groupes::Groupe},
};

#[derive(Default)]
pub struct QueryGroupes;

#[composable_object]
#[juniper::graphql_object(Context = GraphqlContext)]
impl QueryGroupes {
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
        let mut res = Vec::new();

        let totalGroupes: Vec<Vec<entities::groupes::Model>> =
            entities::groupes_access::Entity::find()
                .filter(entities::groupes_access::Column::IdUser.eq(ctx.user_id))
                .find_with_related(Groupes)
                .all(&ctx.db.database)
                .await?
                .into_iter()
                .map(|x| x.1)
                .collect::<Vec<_>>();

        for mut ele in totalGroupes {
            res.append(&mut ele);
        }

        Ok(res.into_iter().map(|x| x.into()).collect())
    }

    async fn get_invitation(ctx: &GraphqlContext) -> FieldResult<Vec<Groupe>> {
        let invitations = entities::accounts::Entity::find_by_id(ctx.user_id)
            .one(&ctx.db.database)
            .await?
            .unwrap()
            .find_related(entities::invitations::Entity)
            .all(&ctx.db.database)
            .await?;

        let mut groupes = Vec::with_capacity(invitations.len());

        for inv in invitations {
            let groupe = entities::groupes::Entity::find_by_id(inv.groupe_id.unwrap())
                .one(&ctx.db.database)
                .await?
                .unwrap();

            groupes.push(groupe);
        }

        Ok(groupes.into_iter().map(|x| x.into()).collect())
    }
}
