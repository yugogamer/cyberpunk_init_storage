use actix_web::{
    get,
    web::{self},
    HttpResponse,
};

use crate::services::models::character::CharacterStore;

use crate::services::models::roll::roll_initiative;
use crate::{services::database::Database, utils::errors::AppErrors};

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
