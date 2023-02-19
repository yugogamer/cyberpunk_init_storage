use actix_web::cookie::Cookie;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self},
    HttpResponse,
};
use actix_web::{route, HttpRequest, Responder};

use crate::services::models::character::CharacterStore;
use crate::services::models::query::Schema;
use crate::services::models::roll::{roll_initiative, CharacterRoll};
use crate::{
    services::database::Database,
    services::models::{
        auth::{AuthStore, Login},
        user::{InputUser, UserStore},
    },
    utils::{config::Config, errors::AppErrors},
};

#[get("/roll/{groupe_id}")]
pub async fn roll(
    pool: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppErrors> {
    let groupe_id = path.into_inner();
    let characters = pool.get_active_character_in_group(groupe_id).await?;

    let rolls = roll_initiative(&characters);
    let response = HttpResponse::Ok().json(rolls);
    Ok(response)
}
