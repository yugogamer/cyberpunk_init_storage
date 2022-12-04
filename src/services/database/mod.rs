use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::utils::{config::Config, errors::AppErrors};

mod auth;
mod user;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Database, AppErrors> {
        let pool = PgPoolOptions::new().connect(&config.db_url).await?;
        migrate(&pool).await?;
        Ok(Database { pool })
    }
}

async fn migrate(pool: &Pool<Postgres>) -> Result<(), AppErrors> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
