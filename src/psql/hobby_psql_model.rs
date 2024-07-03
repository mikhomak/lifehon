use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::front_api::gql_models::hobby_gql_model::GqlHobby;

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


    pub fn convert_to_gql(&self) -> GqlHobby {
        GqlHobby {
            name: self.name.clone(),
            created_at: self.created_at,
            enabled: self.enabled,
            external_link: self.external_link.clone(),
        }
    }

    pub fn convert_all_to_gql(hobby_models: &Vec<HobbyModel>) -> Vec<GqlHobby> {
        hobby_models
            .iter()
            .map(HobbyModel::convert_to_gql)
            .collect::<Vec<GqlHobby>>()
    }
}
