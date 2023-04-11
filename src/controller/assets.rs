
use actix_web::put;
use actix_web::{
    http::StatusCode,
    web::{self},
    HttpResponse,
};

use crate::services::models::auth::LightUser;
use crate::services::models::database::DatabaseTrait;
use crate::{
    services::database::Database,
};

#[put("/{id}/{filename}")]
pub async fn create(
    _db: web::Data<Database>,
    _path: web::Path<(i32, String)>,
    _payload: web::Bytes,
    _user: LightUser,
) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}
