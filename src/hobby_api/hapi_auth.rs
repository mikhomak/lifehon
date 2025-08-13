use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::Json;
use axum_valid::Valid;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;
use log::info;

use crate::hobby_api::HapiResult;
use crate::psql::user_psql_model::UserModel;
use crate::services::site_service;
use crate::front_api::gql_auth::get_token;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginInput {
    #[validate(length(min = 1, message = "[LOGIN_001] - the name is empty"))]
    pub user_name: String,
    #[validate(length(min = 1, message = "[LOGIN_002] - the password is empty"))]
    pub password: String,
}


#[derive(Deserialize, Serialize, Validate)]
pub struct CheckTokenInput {
    #[validate(length(min = 1, message = "[LOGIN_005] - Bad token"))]
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct SuccessLogin {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub user_name: String,
    pub id: i64,
    pub public_profile: bool,
}

pub async fn check_token(Valid(Json(tokenInput)): Valid<Json<CheckTokenInput>>) -> HapiResult<UserInfo> {
    match get_token(&tokenInput.token) {
        Ok(Claims) => Ok(Json(UserInfo { 
            user_name: Claims.claims.name,
            id: Claims.claims.id,
            public_profile: true
        })),
        Err(_) => Err((StatusCode::UNAUTHORIZED, "[LOGIN_005] Bad token")),
    }
}
pub async fn auth_middleware(
    State(pg_pool): State<PgPool>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let o_token: Option<&str> = request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok());

    if o_token.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if let Ok(claims) = get_token(&o_token.unwrap().to_string()) {
        if let Ok(user) = UserModel::get_user_for_email(&claims.claims.email, &pg_pool).await {
            request.extensions_mut().insert(claims.claims);
            request.extensions_mut().insert(user);
        }
    }
    Ok(next.run(request).await)
}
