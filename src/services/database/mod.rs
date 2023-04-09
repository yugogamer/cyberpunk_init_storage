use async_trait::async_trait;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

use crate::utils::{config::Config, errors::AppErrors};

use self::{auth::Auth, character::CharacterService, groupe::GroupesService, user::UserService};

use super::models::database::DatabaseTrait;

mod auth;
mod character;
mod groupe;
mod orm;
mod user;

#[derive(Clone)]
pub struct Database {
    _pool: Pool<Postgres>,
    auth_service: Auth,
    character_service: CharacterService,
    group_service: GroupesService,
    user_service: UserService,
}

impl juniper::Context for Database {}

#[async_trait]
impl DatabaseTrait<Self> for Database {
    async fn new(config: &Config) -> Result<Database, AppErrors> {
        let connection = PgConnectOptions::new()
            .host(&config.db_host)
            .port(config.db_port)
            .username(&config.db_user)
            .password(&config.db_password)
            .database(&config.db_name);

        let pool = PgPoolOptions::new().connect_with(connection).await?;
        migrate(&pool).await?;
        Ok(Database {
            auth_service: Auth::new(pool.clone()),
            character_service: CharacterService::new(pool.clone()),
            group_service: GroupesService::new(pool.clone()),
            user_service: UserService::new(pool.clone()),
            _pool: pool,
        })
    }

    fn auth_service(&self) -> Box<dyn super::models::auth::AuthStore> {
        Box::new(self.auth_service.clone())
    }

    fn character_service(&self) -> Box<dyn super::models::character::CharacterStore> {
        Box::new(self.character_service.clone())
    }

    fn group_service(&self) -> Box<dyn super::models::groupes::GroupeStore> {
        Box::new(self.group_service.clone())
    }

    fn token_store(&self) -> Box<dyn super::models::token::TokenStore> {
        todo!()
    }

    fn user_store(&self) -> Box<dyn super::models::user::UserStore> {
        Box::new(self.user_service.clone())
    }
}

async fn migrate(pool: &Pool<Postgres>) -> Result<(), AppErrors> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
