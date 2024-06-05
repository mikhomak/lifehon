use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::hobby_api::habi_task::CreateTask;

#[derive(FromRow, Deserialize, Serialize)]
pub struct TaskModel {
    pub id: sqlx::types::Uuid,
    pub user_name: String,
    pub hobby_name: String,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub given_exp: i64,
    pub created_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

impl TaskModel {

    pub async fn create_task(user_name: &String, create_task: &CreateTask, pg_pool: &PgPool) -> Result<TaskModel, sqlx::Error>{

        let r_task = sqlx::query_as!(
            TaskModel,
            "INSERT INTO l_task(name, user_name, hobby_name, external_id, description, created_at, finished_at, given_exp, public) \
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) \
            RETURNING *",
            create_task.name,
            user_name,
            create_task.hobby_name,
            create_task.external_id,
            create_task.description.clone().unwrap_or(String::new()),
            create_task.created_at.unwrap_or(DateTime::from(Local::now())),
            create_task.finished_at.unwrap_or(DateTime::from(Local::now())),
            create_task.given_exp,
            create_task.public)
            .fetch_one(pg_pool)
            .await?;
        Ok(r_task)
    }
}