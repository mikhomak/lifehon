use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::hobby_api::HabiResult;
use crate::psql::user_psql_model::UserModel;

#[derive(Deserialize, Serialize)]
pub struct SuccessLogin {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HapiClaims {
    pub id: String,
    pub email: String,
    exp: usize,
}

pub fn get_claims_from_token(token: &String) -> Result<TokenData<HapiClaims>, jsonwebtoken::errors::Error> {
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
        exp: 100000000000000,
    };
    let auth_secret = dotenv::var("TOKEN_SECRET").expect("Auth secret is not set!");
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(auth_secret.as_ref()),
    )?;
    Ok(token)
}

pub async fn login_user(State(pg_pool): State<PgPool>,
                        Json((user_name, password)): Json<(String, String)>)
                        -> HabiResult<SuccessLogin> {
    match UserModel::login_user(&user_name, &password, &pg_pool).await {
        Ok(user_model) => {

            let Ok(token) = create_token(&user_model.name, &user_model.email) else {
                return Err((StatusCode::SERVICE_UNAVAILABLE, "[LOGIN_002] Something went wrong...".to_string()))
            };

            Ok(Json(SuccessLogin { token  }))
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "[LOGIN_001] Credentials are not correct".to_string()))
    }
}