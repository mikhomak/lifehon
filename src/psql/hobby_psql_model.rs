use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::front_api::gql_models::hobby_gql_model::GqlHobby;

#[derive(FromRow, Deserialize, Serialize, Clone)]
pub struct HobbyModel {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub enabled: bool,
    pub external_url: String,
    pub create_user_callback: String,
    pub token: String,
}

impl HobbyModel {

    pub async fn create_token(
        new_hobby: &HobbyModel,
        pg_pool: &PgPool,
    ) -> Result<HobbyModel, sqlx::Error> {
        let r_hobby = sqlx::query_as!(
            HobbyModel,
            "INSERT INTO l_hobby(name, created_at, token, external_url, create_user_callback) \
            VALUES ($1,$2,$3,$4,$5) \
            RETURNING *",
            new_hobby.name,
            new_hobby.created_at,
            new_hobby.token,
            new_hobby.external_url,
            new_hobby.create_user_callback
            )
            .fetch_one(pg_pool)
            .await?;
        Ok(r_hobby)
    }

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


    pub async fn get_hobby_for_token(
        token: &String,
        pool: &PgPool,
    ) -> Result<HobbyModel, sqlx::Error> {
        let r_hobby: HobbyModel =
            sqlx::query_as!(HobbyModel, "SELECT * FROM l_hobby WHERE token = $1", token)
                .fetch_one(pool)
                .await?;
        Ok(r_hobby)
    }

    pub fn convert_to_gql(&self) -> GqlHobby {
        GqlHobby {
            name: self.name.clone(),
            created_at: self.created_at,
            enabled: self.enabled,
            external_url: self.external_url.clone(),
            create_user_callback: self.create_user_callback.clone(),
        }
    }

    pub fn convert_all_to_gql(hobby_models: &Vec<HobbyModel>) -> Vec<GqlHobby> {
        hobby_models
            .iter()
            .map(HobbyModel::convert_to_gql)
            .collect::<Vec<GqlHobby>>()
    }
}
