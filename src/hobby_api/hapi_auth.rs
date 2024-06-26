use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::Json;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_valid::Valid;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::hobby_api::HabiResult;
use crate::psql::user_psql_model::UserModel;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct HapiClaims {
    pub id: String,
    pub email: String,
    exp: usize,
    iat: i64,
}

pub fn get_claims_from_token(
    token: &String,
) -> Result<TokenData<HapiClaims>, jsonwebtoken::errors::Error> {
    let auth_secret = dotenv::var("TOKEN_SECRET").expect("Token secret is not set!");
    let token: TokenData<HapiClaims> = decode::<HapiClaims>(
        token,
        &DecodingKey::from_secret(auth_secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token)
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
        Ok(_) => { Ok(Json(SuccessLogin { token })) }
        Err(_) => { Err((StatusCode::UNAUTHORIZED, "[LOGIN_005] Bad token".to_string())) }
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

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let option = request.headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok());

    if option.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}