use hmac::{Hmac, Mac};
use log::{info, warn};
use sha2::Sha256;

use super::errors::AppErrors;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub jwt_secret: Hmac<Sha256>,
    pub argon2_config: argon2::Config<'static>,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,

    pub bucket_name: String,
    pub bucket_region: String,
    pub bucket_endpoint: String,
    pub bucket_access: String,
    pub bucket_secret: String,
}

impl Config {
    pub fn new() -> Result<Config, AppErrors> {
        let mut config = Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            db_host: "localhost".to_string(),
            db_user: "api_save".to_string(),
            db_password: "api_pswd".to_string(),
            db_port: 5432,
            db_name: "character_storage".to_string(),
            jwt_secret: Hmac::new_from_slice(b"secret")?,
            argon2_config: argon2::Config::default(),
            bucket_name: "raina-test-dev".to_string(),
            bucket_region: "fr-par".to_string(),
            bucket_endpoint: "https://s3.fr-par.scw.cloud".to_string(),
            bucket_access: "SCWPAPF1X1VVK7109MP0".to_string(),
            bucket_secret: "7605596e-2438-4e97-948d-a14e9d39eebf".to_string(),
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
        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            self.jwt_secret = Hmac::new_from_slice(jwt_secret.as_bytes())?;
        }
        if let Ok(bucket_name) = std::env::var("BUCKET_NAME") {
            self.bucket_name = bucket_name;
        }
        if let Ok(bucket_region) = std::env::var("BUCKET_REGION") {
            self.bucket_region = bucket_region;
        }
        if let Ok(bucket_endpoint) = std::env::var("BUCKET_ENDPOINT") {
            self.bucket_endpoint = bucket_endpoint;
        }
        if let Ok(bucket_access) = std::env::var("BUCKET_ACCESS") {
            self.bucket_access = bucket_access;
        }
        if let Ok(bucket_secret) = std::env::var("BUCKET_SECRET") {
            self.bucket_secret = bucket_secret;
        }
        if let Ok(db_host) = std::env::var("DB_HOST") {
            self.db_host = db_host;
        }
        if let Ok(db_port) = std::env::var("DB_PORT") {
            self.db_port = db_port.parse::<u16>().unwrap();
        }
        if let Ok(db_user) = std::env::var("DB_USER") {
            self.db_user = db_user;
        }
        if let Ok(db_password) = std::env::var("DB_PASSWORD") {
            self.db_password = db_password;
        }
        if let Ok(db_name) = std::env::var("DB_NAME") {
            self.db_name = db_name;
        }
        Ok(())
    }
}
