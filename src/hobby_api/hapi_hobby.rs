use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

use crate::psql::hobby_psql_model::HobbyModel;

pub async fn get_all_hobbies(
    State(pg_pool): State<PgPool>,
) -> Result<Json<Vec<HobbyModel>>, (StatusCode, String)> {
    match HobbyModel::get_all_hobbies(&pg_pool).await {
        Ok(hobbies) => Ok(Json(hobbies)),
        Err(_) => Err((
            StatusCode::NO_CONTENT,
            "[HOBBY_001] Hobbies are not found".to_string(),
        )),
    }
}
