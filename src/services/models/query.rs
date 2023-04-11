use juniper::{EmptySubscription, RootNode};
use juniper_compose::composite_object;

use crate::controller::graphql::GraphqlContext;

use super::{
    character::{mutation::CharacterMutation, query::CharacterQuery},
    groupes::{mutation::GroupesMutation, query::QueryGroupes},
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphqlContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

composite_object!(pub Query < Context = GraphqlContext > (CharacterQuery, QueryGroupes));
composite_object!(pub Mutation < Context = GraphqlContext > (CharacterMutation, GroupesMutation));
