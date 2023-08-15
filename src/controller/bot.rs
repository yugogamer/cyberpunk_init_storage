use actix_web::{
    get,
    web::{self},
    HttpResponse,
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use crate::services::models::auth::LightUser;
use crate::services::models::roll::roll_initiative;
use crate::{services::database::Database, utils::errors::AppErrors};

#[get("/roll/{groupe_id}")]
pub async fn roll(
    pool: web::Data<Database>,
    _user: LightUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppErrors> {
    let groupe_id = path.into_inner();
    let groupe = entities::groupes::Entity::find_by_id(groupe_id)
        .one(&pool.database)
        .await?;

    let character = groupe
        .unwrap()
        .find_related(entities::characters::Entity)
        .filter(entities::active_in_groups::Column::Active.eq(true))
        .all(&pool.database)
        .await?
        .into_iter()
        .map(|c| c.into())
        .collect();

    let rolls = roll_initiative(&character);

    let response = HttpResponse::Ok().json(rolls);
    Ok(response)
}
