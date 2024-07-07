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

use crate::hobby_api::HabiResult;
use crate::psql::user_psql_model::UserModel;
use crate::services::site_service;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginInput {
    #[validate(length(min = 1, message = "[LOGIN_001] - the name is empty"))]
    pub user_name: String,
    #[validate(length(min = 1, message = "[LOGIN_002] - the password is empty"))]
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SuccessLogin {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HapiClaims {
    pub id: String,
    pub email: String,
    exp: usize,
    iat: i64,
}

pub fn create_token(id: &String, email: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = HapiClaims {
        id: id.clone(),
        email: email.clone(),
        exp: 3600000000000,
        iat: chrono::offset::Local::now().timestamp(),
    };
    let auth_secret = dotenv::var("TOKEN_SECRET").expect("Auth secret is not set!");
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(auth_secret.as_ref()),
    )?;
    Ok(token)
}

pub async fn login_user(
    State(pg_pool): State<PgPool>,
    Valid(Json(login_input)): Valid<Json<LoginInput>>,
) -> HabiResult<SuccessLogin> {
    if !site_service::is_login_enabled(&pg_pool) {
        return Err((StatusCode::SERVICE_UNAVAILABLE, "[LOGIN_006] Service in not available".to_string()));
    }
    match UserModel::login_user(&login_input.user_name, &login_input.password, &pg_pool).await {
        Ok(user_model) => {
            let Ok(token) = create_token(&user_model.name, &user_model.email) else {
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    "[LOGIN_003] Something went wrong...".to_string(),
                ));
            };

            Ok(Json(SuccessLogin { token }))
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            "[LOGIN_004] Credentials are not correct".to_string(),
        )),
    }
}

pub async fn check_token(token: String) -> HabiResult<SuccessLogin> {
    match decode_token(&token) {
        Ok(_) => Ok(Json(SuccessLogin { token })),
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            "[LOGIN_005] Bad token".to_string(),
        )),
    }
}

fn decode_token(token: &String) -> Result<TokenData<HapiClaims>, Error> {
    let auth_secret = dotenv::var("TOKEN_SECRET").expect("Auth secret is not set!");
    let token: TokenData<HapiClaims> = decode::<HapiClaims>(
        token,
        &DecodingKey::from_secret(auth_secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token)
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
    if let Ok(claims) = decode_token(&o_token.unwrap().to_string()) {
        if let Ok(user) = UserModel::get_user_for_email(&claims.claims.email, &pg_pool).await {
            request.extensions_mut().insert(claims.claims);
            request.extensions_mut().insert(user);
        }
    }
    Ok(next.run(request).await)
}
