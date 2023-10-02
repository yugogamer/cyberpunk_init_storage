use actix_web::{get, put};
use actix_web::{
    http::StatusCode,
    web::{self},
    HttpResponse,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

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

#[get("/{filename}")]
pub async fn get(
    db: web::Data<Database>,
    path: web::Path<String>,
    storage: web::Data<crate::services::bucket::BucketHandler>,
) -> Result<HttpResponse, AppErrors> {
    let filename = path.into_inner();
    let asset = entities::assets::Entity::find()
        .filter(entities::assets::Column::BucketName.contains(&filename))
        .one(&db.database)
        .await?;
    if let Some(asset) = asset {
        let file = storage.download(&asset.bucket_name).await;
        Ok(HttpResponse::Ok()
            .insert_header(("Cache-Control", "max-age=604800"))
            .body(file))
    } else {
        Err(AppErrors::NotFound(format!("Asset {} not found", filename)))
    }
}
