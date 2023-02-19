use actix_web::{
    get,
    web::{self},
    HttpResponse,
};

use crate::services::models::auth::LightUser;
use crate::services::models::character::CharacterStore;
use crate::services::models::roll::roll_initiative;
use crate::{services::database::Database, utils::errors::AppErrors};

#[get("/roll/{groupe_id}")]
pub async fn roll(
    pool: web::Data<Database>,
    user: LightUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppErrors> {
    let groupe_id = path.into_inner();
    let characters = pool.get_active_character_in_group(groupe_id).await?;
    if characters.is_empty() {
        return Err(AppErrors::NotFound(
            "Groups doesn't have any active characters".to_string(),
        ));
    }
    if characters[0].user_id != user.id {
        return Err(AppErrors::NotFound(
            "You are not the owner of this group".to_string(),
        ));
    }

    let rolls = roll_initiative(&characters);
    let response = HttpResponse::Ok().json(rolls);
    Ok(response)
}
