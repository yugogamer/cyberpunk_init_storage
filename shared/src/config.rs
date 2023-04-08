use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::errors::AppErrors;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub jwt_secret: Hmac<Sha256>,
    pub argon2_config: argon2::Config<'static>,
}

impl Config {
    pub fn new() -> Result<Config, AppErrors> {
        let mut config = Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            db_url: "postgres://api_save:api_pswd@localhost:5432/character_storage".to_string(),
            jwt_secret: Hmac::new_from_slice(b"secret")?,
            argon2_config: argon2::Config::default(),
        };

        config.load_config()?;
        Ok(config)
    }

    fn load_config(&mut self) -> Result<(), AppErrors> {
        if let Ok(host) = std::env::var("HOST") {
            self.host = host;
        }
        if let Ok(port) = std::env::var("PORT") {
            self.port = port.parse::<u16>().unwrap();
        }
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            self.db_url = db_url;
        }
        if let Ok(db_url) = std::env::var("POSTGRESQL_ADDON_URI") {
            self.db_url = db_url;
        }
        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            self.jwt_secret = Hmac::new_from_slice(jwt_secret.as_bytes())?;
        }
        Ok(())
    }
}
