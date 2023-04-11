use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::utils::errors::AppErrors;

pub async fn can_access_groupe(
    groupe_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, AppErrors> {
    let res = entities::groupes_access::Entity::find()
        .filter(entities::groupes_access::Column::IdGroupe.eq(groupe_id))
        .filter(entities::groupes_access::Column::IdUser.eq(user_id))
        .one(db)
        .await?;
    if let Some(_res) = res {
        return Ok(true);
    }
    Ok(false)
}

pub async fn can_edit_groupe(
    groupe_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, AppErrors> {
    let res = entities::groupes::Entity::find_by_id(groupe_id)
        .one(db)
        .await?;
    if let Some(res) = res {
        if res.owner_id == user_id {
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn can_edit_character(
    character_id: i32,
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, AppErrors> {
    let res = entities::characters::Entity::find_by_id(character_id)
        .one(db)
        .await?;
    if let Some(res) = res {
        if res.user_id == user_id {
            return Ok(true);
        }
    }
    Ok(false)
}
