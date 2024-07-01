use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::psql::task_psql_model::TaskModel;
use crate::psql::user_psql_model::UserModel;

#[derive(Deserialize, Serialize)]
pub struct CreateTask {
    pub name: String,
    pub hobby_name: String,
    pub external_id: String,
    pub description: Option<String>,
    pub public: bool,
    pub given_exp: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

pub async fn create_task(
    Path(user_name): Path<String>,
    State(pg_pool): State<PgPool>,
    Json(new_task): Json<CreateTask>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    if UserModel::get_user_for_name(&user_name, &pg_pool).await.is_err() {
        return Err((StatusCode::NOT_FOUND, "[TASK_002] User not found!".to_string()));
    }

    match TaskModel::create_task(&user_name, &new_task, &pg_pool).await {
        Ok(task_model) => {
            info!(
                "Task has been created with name [{}] and external id [{}] for user [{}]",
                task_model.name, task_model.external_id, task_model.user_name
            );
            Ok(Json(task_model))
        }
        Err(error) => {
            error!(
                "Error at creating the task for user [{}] and external id [{}], the error is [{}]",
                user_name,
                new_task.external_id,
                error.to_string()
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "[TASK_001] Something went wrong!".to_string(),
            ))
        }
    }
}
