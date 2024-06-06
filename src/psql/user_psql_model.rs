use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::hobby_api::habi_user::CreateUser;

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
    pub async fn create_user(create_user: &CreateUser, pg_pool: &PgPool) -> Result<UserModel, sqlx::Error> {
        let r_user: UserModel = sqlx::query_as!(
            UserModel,
            "INSERT INTO l_user(name, display_name, email, password, login_enabled, consent, public_profile) \
            VALUES ($1,$2,$3,$4,$5,$6,$7) \
            RETURNING *",
            create_user.name,
            create_user.name,
            create_user.email,
            create_user.password,
            true,
            true,
            create_user.public_profile)
            .fetch_one(pg_pool)
            .await?;
        Ok(r_user)
    }

    pub async fn get_user_for_name(name: &String, pg_pool: &PgPool) -> Result<UserModel, sqlx::Error> {
        let r_user: UserModel = sqlx::query_as!(UserModel, "SELECT * FROM l_user WHERE name = $1", name)
            .fetch_one(pg_pool)
            .await?;
        Ok(r_user)
    }

    pub async fn login_user(name: &String, password: &String, pg_pool: &PgPool) -> Result<UserModel, sqlx::Error> {
        let r_user: UserModel = sqlx::query_as!(UserModel, "SELECT * FROM l_user WHERE name = $1 AND password = $2",
            name,
            password)
            .fetch_one(pg_pool)
            .await?;
        Ok(r_user)
    }

    pub async fn add_hobby_to_user(user_name: &String, hobby_name: &String, pg_pool: &PgPool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!("INSERT INTO rel_user2hobby(user_name, hobby_name)\
        VALUES($1,$2)",
            user_name,
            hobby_name,
        ).execute(pg_pool).await?;
        Ok(())
    }
}