use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Deserialize, Serialize)]
pub struct HobbyModel {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub enabled: bool,
    pub external_link: Option<String>,
}

impl HobbyModel {
    pub async fn get_all_hobbies(pool: &PgPool) -> Result<Vec<HobbyModel>, sqlx::Error> {
        let r_hobbies: Vec<HobbyModel> =
            sqlx::query_as!(HobbyModel, "SELECT * FROM l_hobby ORDER BY created_at")
                .fetch_all(pool)
                .await?;
        Ok(r_hobbies)
    }

    pub async fn get_hobby_for_name(
        name: &String,
        pool: &PgPool,
    ) -> Result<HobbyModel, sqlx::Error> {
        let r_hobby: HobbyModel =
            sqlx::query_as!(HobbyModel, "SELECT * FROM l_hobby WHERE name = $1", name)
                .fetch_one(pool)
                .await?;
        Ok(r_hobby)
    }
}
