use super::{
    auth::AuthStore,
};
use crate::utils::{config::Config, errors::AppErrors};
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseTrait<T>: Sync {
    async fn new(config: &Config) -> Result<T, AppErrors>;
    fn auth_service(&self) -> Box<dyn AuthStore>;
}
