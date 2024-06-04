use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct UserModel {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub login_enabled: bool,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub consent: bool,
    pub public_profile: bool,
    pub exp: i64,
}

impl UserModel {
    pub async fn get_user_for_name(name: &String, pg_pool: &PgPool) -> Result<UserModel, sqlx::Error> {
        let r_user: UserModel = sqlx::query_as!(UserModel, "SELECT * FROM l_user WHERE name = $1", name)
            .fetch_one(pg_pool)
            .await?;
        Ok(r_user)
    }
}