use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::front_api::gql_models::user_gql_model::GqlUser;
use crate::hobby_api::hapi_user::CreateUserInput;
use crate::psql::hobby_psql_model::HobbyModel;
use crate::psql::task_psql_model::TaskModel;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
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
    pub async fn create_user(
        create_user: &CreateUserInput,
        pg_pool: &PgPool,
    ) -> Result<UserModel, sqlx::Error> {
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

    pub async fn get_all(pg_pool: &PgPool) -> Result<Vec<UserModel>, sqlx::Error> {
        let r_user: Vec<UserModel> = sqlx::query_as!(UserModel, "SELECT * FROM l_user")
            .fetch_all(pg_pool)
            .await?;
        Ok(r_user)
    }

    pub async fn get_user_for_name(
        name: &String,
        pg_pool: &PgPool,
    ) -> Result<UserModel, sqlx::Error> {
        let r_user: UserModel =
            sqlx::query_as!(UserModel, "SELECT * FROM l_user WHERE name = $1", name)
                .fetch_one(pg_pool)
                .await?;
        Ok(r_user)
    }

    pub async fn get_user_for_email(
        email: &String,
        pg_pool: &PgPool,
    ) -> Result<UserModel, sqlx::Error> {
        let r_user: UserModel =
            sqlx::query_as!(UserModel, "SELECT * FROM l_user WHERE email = $1", email)
                .fetch_one(pg_pool)
                .await?;
        Ok(r_user)
    }

    pub async fn login_user(
        name: &String,
        password: &String,
        pg_pool: &PgPool,
    ) -> Result<UserModel, sqlx::Error> {
        let r_user: UserModel = sqlx::query_as!(
            UserModel,
            "SELECT * FROM l_user WHERE name = $1 AND password = $2",
            name,
            password
        )
        .fetch_one(pg_pool)
        .await?;
        Ok(r_user)
    }

    pub async fn add_hobby_to_user(
        user_name: &String,
        hobby_name: &String,
        pg_pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            "INSERT INTO rel_user2hobby(user_name, hobby_name) VALUES($1,$2)",
            user_name,
            hobby_name,
        )
        .execute(pg_pool)
        .await?;
        Ok(())
    }

    pub async fn get_hobbies_for_user_name(
        user_name: &String,
        pg_pool: &PgPool,
    ) -> Result<Vec<HobbyModel>, sqlx::Error> {
        let r_hobbies: Vec<HobbyModel> =
            sqlx::query_as!(HobbyModel, "SELECT hobby.* FROM (l_hobby AS hobby LEFT JOIN rel_user2hobby AS r_u2h ON hobby.name = r_u2h.hobby_name) WHERE r_u2h.user_name = $1", user_name)
                .fetch_all(pg_pool)
                .await?;
        Ok(r_hobbies)
    }

    pub async fn get_tasks_for_user_name(
        user_name: &String,
        page: i64,
        pg_pool: &PgPool,
    ) -> Result<Vec<TaskModel>, sqlx::Error> {
        let r_task_models: Vec<TaskModel> =
            sqlx::query_as!(TaskModel, "SELECT task.* FROM (l_task AS task JOIN l_user AS l_user ON task.user_name = l_user.name) WHERE l_user.name = $1 ORDER BY task.created_at DESC LIMIT $2 OFFSET $3",
                user_name,
                30,
                page * 30)
                .fetch_all(pg_pool)
                .await?;
        Ok(r_task_models)
    }
    pub fn convert_to_gql(&self) -> GqlUser {
        GqlUser {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            login_enabled: self.login_enabled,
            password: self.password.clone(),
            created_at: self.created_at,
            consent: self.consent,
        }
    }

    pub fn convert_all_to_gql(user_models: &Vec<UserModel>) -> Vec<GqlUser> {
        user_models
            .iter()
            .map(UserModel::convert_to_gql)
            .collect::<Vec<GqlUser>>()
    }
}
