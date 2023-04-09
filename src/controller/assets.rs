use actix_web::cookie::Cookie;
use actix_web::put;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self},
    HttpResponse,
};

use crate::services::models::auth::LightUser;
use crate::services::models::database::DatabaseTrait;
use crate::{
    services::database::Database,
    services::models::{auth::Login, user::InputUser},
    utils::{config::Config, errors::AppErrors},
};

#[put("/{id}/{filename}")]
pub async fn create(
    db: web::Data<Database>,
    path: web::Path<(i32, String)>,
    payload: web::Bytes,
    user: LightUser,
) -> HttpResponse {
    let image = payload.to_vec();
}
