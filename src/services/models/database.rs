use super::{
    auth::AuthStore, character::CharacterStore, groupes::GroupeStore, token::TokenStore,
    user::UserStore,
};
use crate::utils::{config::Config, errors::AppErrors};
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseTrait<T>: Sync {
    async fn new(config: &Config) -> Result<T, AppErrors>;
    fn auth_service(&self) -> Box<dyn AuthStore>;
    fn character_service(&self) -> Box<dyn CharacterStore>;
    fn group_service(&self) -> Box<dyn GroupeStore>;
    fn token_store(&self) -> Box<dyn TokenStore>;
    fn user_store(&self) -> Box<dyn UserStore>;
}
