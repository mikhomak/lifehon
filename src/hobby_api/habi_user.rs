use axum::{extract, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::psql::user_psql_model::UserModel;

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub consent: bool,
    pub public_profile: bool,
    pub email: String,
    pub password: String,
}

pub async fn get_user_for_name(Path(name): Path<String>, State(pg_pool): State<PgPool>)
                               -> Result<Json<UserModel>, (StatusCode, String)> {
    match UserModel::get_user_for_name(&name, &pg_pool).await {
        Ok(user_model) => { Ok(Json(user_model)) }
        Err(error) => {
            error!("Error at fetching the user with name [{}], the error is [{}]", name, error.to_string());
            Err((StatusCode::NO_CONTENT, "[USER_001] user is not found!".to_string()))
        }
    }
}

pub async fn create_user(State(pg_pool): State<PgPool>,
                         Json(new_user): Json<CreateUser>)
                         -> Result<Json<UserModel>, (StatusCode, String)> {
    match UserModel::create_user(&new_user, &pg_pool).await {
        Ok(user_model) => {
            info!("User has been created with name [{}] and id [{}]", user_model.name, user_model.id.to_string());
            Ok(Json(user_model))
        }
        Err(error) => {
            error!("Error at creating the user, the error is [{}]", error.to_string());
            Err((StatusCode::INTERNAL_SERVER_ERROR, "[USER_002] Something went wrong!".to_string()))
        }
    }
}