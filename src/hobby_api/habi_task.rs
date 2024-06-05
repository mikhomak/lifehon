use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::psql::hobby_psql_model::HobbyModel;
use crate::psql::task_psql_model::TaskModel;
use crate::psql::user_psql_model::UserModel;

#[derive(Deserialize, Serialize)]
pub struct CreateTask {
    pub user_name: String,
    pub hobby_name: String,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub given_exp: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

pub async fn create_task(Path(user_name): Path<String>,
                         State(pg_pool): State<PgPool>,
                         Json(mut new_task): Json<CreateTask>)
                         -> Result<Json<TaskModel>, (StatusCode, String)> {
    new_task.user_name = user_name;
    match TaskModel::create_task(&new_task, &pg_pool).await {
        Ok(task_model) => {
            info!("Task has been created with name [{}] and external id [{}] for user [{}]", task_model.name, task_model.external_id, task_model.user_name);
            Ok(Json(task_model))
        }
        Err(error) => {
            error!("Error at creating the task for user [{}] and external id [{}], the error is [{}]",new_task.user_name, new_task.external_id, error.to_string());
            Err((StatusCode::INTERNAL_SERVER_ERROR, "[TASK_001] Something went wrong!".to_string()))
        }
    }
}