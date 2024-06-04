use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

use crate::psql::user_psql_model::UserModel;

pub async fn get_user_for_name(name: &String, State(pg_pool): State<PgPool>) -> Result<Json<UserModel>, (StatusCode, String)> {
    match UserModel::get_user_for_name(name, &pg_pool).await {
        Ok(user_model) => { Ok(Json(user_model)) }
        Err(_) => { Err((StatusCode::NO_CONTENT, "user is not found!".to_string())) }
    }
}