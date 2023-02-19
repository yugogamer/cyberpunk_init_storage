use std::{future::Future, pin::Pin};

use super::{config::Config, errors::AppErrors};
use crate::services::models::auth::LightUser;

use actix_web::{web::Data, FromRequest};
use jwt::{SignWithKey, VerifyWithKey};

fn generate_random_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    for i in salt.iter_mut() {
        *i = rand::random::<u8>();
    }
    salt
}

pub fn hash_password(password: &str, config: &argon2::Config) -> Result<String, AppErrors> {
    let password = password.as_bytes();
    let salt = generate_random_salt();
    let hash = argon2::hash_encoded(password, &salt, config)?;
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppErrors> {
    let password = password.as_bytes();
    let result = argon2::verify_encoded(hash, password)?;
    Ok(result)
}

pub fn generate_jwt(
    accounts: &LightUser,
    key: &hmac::Hmac<sha2::Sha256>,
) -> Result<String, AppErrors> {
    let token = accounts.sign_with_key(key)?;
    Ok(token)
}

pub fn verify_jwt(token: &str, key: &hmac::Hmac<sha2::Sha256>) -> Result<LightUser, AppErrors> {
    let accounts: LightUser = token.verify_with_key(key)?;
    Ok(accounts)
}

impl FromRequest for LightUser {
    type Error = AppErrors;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();
        let config = req.app_data::<Data<Config>>().unwrap().clone();
        Box::pin(async move {
            let token = req.cookie("session");
            if token.is_none() {
                let header_token = req.headers().get("session");
                if header_token.is_none() {
                    return Err(AppErrors::Unauthorized);
                }
                let token = header_token.unwrap().to_str().unwrap().to_string();
                let accounts = verify_jwt(&token, &config.jwt_secret)?;
                Ok(accounts)
            } else {
                let token = token.unwrap().value().to_string();
                let accounts = verify_jwt(&token, &config.jwt_secret)?;
                Ok(accounts)
            }
        })
    }
}

//test argon2
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_password() {
        let config = argon2::Config::default();
        let hash = hash_password("yugogamer3", &config).unwrap();
        assert!(verify_password("yugogamer3", &hash).unwrap());
    }
}
