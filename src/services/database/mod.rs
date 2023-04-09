use async_trait::async_trait;
use sea_orm::{ConnectOptions, DatabaseConnection};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};

use crate::utils::{config::Config, errors::AppErrors};

use self::auth::Auth;

use super::models::database::DatabaseTrait;

mod auth;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
    pub database: DatabaseConnection,
    auth_service: Auth,
}

impl juniper::Context for Database {}

#[async_trait]
impl DatabaseTrait<Self> for Database {
    async fn new(config: &Config) -> Result<Database, AppErrors> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
        );

        let connection = PgConnectOptions::new()
            .host(&config.db_host)
            .port(config.db_port)
            .username(&config.db_user)
            .password(&config.db_password)
            .database(&config.db_name);

        let db = ConnectOptions::new(url);
        let database = sea_orm::Database::connect(db).await.unwrap();

        let pool = PgPoolOptions::new().connect_with(connection).await?;
        migrate(&pool).await?;
        Ok(Database {
            database,
            auth_service: Auth::new(pool.clone()),
            pool: pool,
        })
    }

    fn auth_service(&self) -> Box<dyn super::models::auth::AuthStore> {
        Box::new(self.auth_service.clone())
    }
}

async fn migrate(pool: &Pool<Postgres>) -> Result<(), AppErrors> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
