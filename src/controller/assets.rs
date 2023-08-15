use actix_web::put;
use actix_web::{
    http::StatusCode,
    web::{self},
    HttpResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

use crate::services::database::Database;
use crate::services::models::auth::LightUser;
use crate::utils::errors::AppErrors;

#[put("/{id}/{filename}")]
pub async fn create(
    db: web::Data<Database>,
    path: web::Path<(i32, String)>,
    payload: web::Bytes,
    storage: web::Data<crate::services::bucket::BucketHandler>,
    user: LightUser,
) -> Result<HttpResponse, AppErrors> {
    let (id, filename) = path.into_inner();
    let character = entities::characters::Entity::find_by_id(id)
        .one(&db.database)
        .await?;
    if let Some(character) = character {
        if character.user_id != user.id {
            println!(
                "id user : {}, character user id : {}",
                user.id, character.user_id
            );
            return Err(AppErrors::Unauthorized);
        }
        if character.asset_id.is_some() {
            let asset = entities::assets::Entity::find_by_id(character.asset_id.unwrap())
                .one(&db.database)
                .await?;
            if let Some(asset) = asset {
                storage.remove(&asset.bucket_name).await;
                asset.into_active_model().delete(&db.database).await?;
            }
        }

        let bucket_name = storage.upload(&filename, &payload).await;
        let asset = entities::assets::ActiveModel {
            original_name: Set(filename),
            bucket_name: Set(bucket_name),
            uploader_id: Set(user.id),
            ..Default::default()
        }
        .insert(&db.database)
        .await?;
        let mut character = character.into_active_model();
        character.set(entities::characters::Column::AssetId, asset.id.into());
        character.save(&db.database).await?;

        Ok(HttpResponse::new(StatusCode::CREATED))
    } else {
        Err(AppErrors::NotFound(format!("Character {} not found", id)))
    }
}
