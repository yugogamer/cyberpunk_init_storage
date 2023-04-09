use crate::services::models::character::*;
use crate::utils::errors::AppErrors;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct CharacterService {
    pub pool: Pool<Postgres>,
}

impl CharacterService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CharacterStore for CharacterService {
    async fn get_character(&self, id: i32) -> Result<Character, AppErrors> {
        let character = sqlx::query_as!(
            Character,
            r#"
            SELECT * FROM characters WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(character)
    }
    async fn get_active_character_in_group(&self, id: i32) -> Result<Vec<Character>, AppErrors> {
        let characters = sqlx::query_as!(
            Character,
            r#"
            SELECT * FROM characters 
            INNER JOIN groupes ON groupes.id = characters.groupe_id
             WHERE groupe_id = $1 AND active = true
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(characters)
    }
    async fn get_character_by_group(&self, id: i32) -> Result<Vec<Character>, AppErrors> {
        let characters = sqlx::query_as!(
            Character,
            r#"
            SELECT * FROM characters WHERE groupe_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(characters)
    }
    async fn get_character_by_user(&self, id: i32) -> Result<Vec<Character>, AppErrors> {
        let characters = sqlx::query_as!(
            Character,
            r#"
            SELECT * FROM characters WHERE user_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(characters)
    }
    async fn create_character(
        &self,
        character: InputCharacter,
        user_id: i32,
    ) -> Result<Character, AppErrors> {
        let character = sqlx::query_as!(
            Character,
            r#"
            INSERT INTO characters (name, user_id, groupe_id, base_ref, modifier, active)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            character.name,
            user_id,
            character.groupe_id,
            character.base_ref,
            character.modifier,
            character.active
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(character)
    }
    async fn update_character(
        &self,
        character: UpdateCharacter,
        id_character: i32,
    ) -> Result<Character, AppErrors> {
        let current_character = self.get_character(id_character).await?;
        let mut character = character;
        if character.name.is_none() {
            character.name = Some(current_character.name);
        }
        if character.user_id.is_none() {
            character.user_id = Some(current_character.user_id);
        }
        if character.groupe_id.is_none() {
            character.groupe_id = Some(current_character.groupe_id);
        }
        if character.base_ref.is_none() {
            character.base_ref = Some(current_character.base_ref);
        }
        if character.modifier.is_none() {
            character.modifier = Some(current_character.modifier);
        }
        if character.active.is_none() {
            character.active = Some(current_character.active);
        }

        let character = sqlx::query_as!(
            Character,
            r#"
            UPDATE characters
            SET name = $1, user_id = $2, groupe_id = $3, base_ref = $4, modifier = $5, active = $6
            WHERE id = $7
            RETURNING *
            "#,
            character.name,
            character.user_id,
            character.groupe_id,
            character.base_ref,
            character.modifier,
            character.active,
            id_character
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(character)
    }
    async fn delete_character(&self, id: i32) -> Result<(), AppErrors> {
        sqlx::query!(
            r#"
            DELETE FROM characters WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
