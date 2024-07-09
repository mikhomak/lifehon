use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Local, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;
use crate::hobby_api::{hapi_hobby_auth, HapiResult};
use crate::psql::hobby_psql_model::HobbyModel;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateHobbyInput {
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub external_link: Option<String>,
    pub password: String,
}

pub async fn get_all_hobbies(
    State(pg_pool): State<PgPool>,
) -> HapiResult<Vec<HobbyModel>> {
    match HobbyModel::get_all_hobbies(&pg_pool).await {
        Ok(hobbies) => Ok(Json(hobbies)),
        Err(_) => Err((
            StatusCode::NO_CONTENT,
            "[HOBBY_001] Hobbies are not found",
        )),
    }
}

pub async fn create_hobby(
    State(pg_pool): State<PgPool>,
    Json(new_hobby): Json<CreateHobbyInput>,
) -> HapiResult<HobbyModel> {
    let r_token = hapi_hobby_auth::encode_hapi_token(&new_hobby.name);

    let Ok(token) = r_token else {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "Something went wrong..."));
    };

    match HobbyModel::create_token(&HobbyModel {
        name: new_hobby.name.clone(),
        created_at: new_hobby.created_at.unwrap_or(DateTime::from(Local::now())),
        enabled: true,
        external_link: None,
        token,
    }, &pg_pool).await {
        Ok(hobby) => { Ok(Json(hobby)) }
        Err(error) => {
            error!("Error while creating a hobby [{}], the error is [{}]", new_hobby.name, error.to_string());
            Err((StatusCode::BAD_REQUEST, "The hobby was not created!"))
        }
    }
}
