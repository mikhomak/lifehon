use crate::hobby_api::validation::task_validation;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::psql::task_psql_model::TaskModel;
use crate::psql::user_psql_model::UserModel;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateTaskInput {
    #[validate(length(min = 1, message = ""))]
    pub name: String,
    #[validate(length(min = 1, message = ""))]
    pub hobby_name: String,
    #[validate(length(min = 1, message = ""))]
    pub external_id: String,
    pub description: Option<String>,
    pub public: bool,
    #[validate(range(min = 0, message = ""))]
    pub given_exp: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

pub async fn create_task(
    State(pg_pool): State<PgPool>,
    user_model: Extension<UserModel>,
    Json(new_task): Json<CreateTaskInput>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    if let Err(validation_error) =
        task_validation::validate_create_task(&user_model.name, &new_task, &pg_pool).await
    {
        return Err((StatusCode::BAD_REQUEST, validation_error.to_string()));
    }
    match TaskModel::create_task(&user_model.name, &new_task, &pg_pool).await {
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
                user_model.name,
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
