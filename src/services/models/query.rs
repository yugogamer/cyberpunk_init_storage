use juniper::{EmptySubscription, FieldResult, RootNode};
use juniper_compose::composite_object;

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

use super::{
    character::{mutation::CharacterMutation, query::CharacterQuery, UpdateCharacter},
    groupes::{mutation::GroupesMutation, query::QueryGroupes, InputGroupe},
    roll::CharacterRoll,
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphqlContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

composite_object!(pub Query < Context = GraphqlContext > (CharacterQuery, QueryGroupes));
composite_object!(pub Mutation < Context = GraphqlContext > (CharacterMutation, GroupesMutation));
