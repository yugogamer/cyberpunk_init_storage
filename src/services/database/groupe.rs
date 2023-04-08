use crate::services::models::groupes::GroupeStore;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct GroupesService {
    pub pool: Pool<Postgres>,
}

impl GroupesService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl GroupeStore for GroupesService {
    async fn get_groupe(
        &self,
        id: i32,
    ) -> Result<crate::services::models::groupes::Groupe, crate::utils::errors::AppErrors> {
        let groupe = sqlx::query_as!(
            crate::services::models::groupes::Groupe,
            r#"
            SELECT * FROM groupes WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        return Ok(groupe);
    }

    async fn get_groupe_secured(
        &self,
        id: i32,
        owner_id: i32,
    ) -> Result<crate::services::models::groupes::Groupe, crate::utils::errors::AppErrors> {
        let groupe = sqlx::query_as!(
            crate::services::models::groupes::Groupe,
            r#"
            SELECT * FROM groupes WHERE id = $1 AND owner_id = $2
            "#,
            id,
            owner_id
        )
        .fetch_one(&self.pool)
        .await?;

        return Ok(groupe);
    }

    async fn get_groupe_by_owner(
        &self,
        id: i32,
    ) -> Result<Vec<crate::services::models::groupes::Groupe>, crate::utils::errors::AppErrors>
    {
        let groupes = sqlx::query_as!(
            crate::services::models::groupes::Groupe,
            r#"
            SELECT * FROM groupes WHERE owner_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        return Ok(groupes);
    }

    async fn create_groupe(
        &self,
        groupe: crate::services::models::groupes::InputGroupe,
        owner_id: i32,
    ) -> Result<crate::services::models::groupes::Groupe, crate::utils::errors::AppErrors> {
        let groupe = sqlx::query_as!(
            crate::services::models::groupes::Groupe,
            r#"
            INSERT INTO groupes (name, owner_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            groupe.name,
            owner_id
        )
        .fetch_one(&self.pool)
        .await?;

        return Ok(groupe);
    }

    async fn update_groupe(
        &self,
        groupe: crate::services::models::groupes::Groupe,
    ) -> Result<crate::services::models::groupes::Groupe, crate::utils::errors::AppErrors> {
        let groupe = sqlx::query_as!(
            crate::services::models::groupes::Groupe,
            r#"
            UPDATE groupes SET name = $1, owner_id = $2
            WHERE id = $3
            RETURNING *
            "#,
            groupe.name,
            groupe.owner_id,
            groupe.id
        )
        .fetch_one(&self.pool)
        .await?;

        return Ok(groupe);
    }

    async fn delete_groupe(&self, id: i32) -> Result<(), crate::utils::errors::AppErrors> {
        sqlx::query!(
            r#"
            DELETE FROM groupes WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        return Ok(());
    }
}
