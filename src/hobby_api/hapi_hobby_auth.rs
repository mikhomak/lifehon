use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::psql::hobby_psql_model::HobbyModel;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct HobbyClaims {
    pub token: String,
}
fn decode_hapi_token(token: &str) -> Result<String, StatusCode> {
    let auth_secret: String =
        dotenv::var("HAPI_TOKEN_SECRET").expect("HAPI token secret is not set!");

    let mut validation = Validation::default();
    validation.validate_exp = false;
    validation.required_spec_claims.remove("exp");
    let Ok(decoded_token) = jsonwebtoken::decode::<HobbyClaims>(token, &DecodingKey::from_secret(auth_secret.as_bytes()), &validation) else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };
    Ok(decoded_token.claims.token)
}

pub fn encode_hapi_token(token: &String) -> Result<String, StatusCode> {
    let auth_secret: String =
        dotenv::var("HAPI_TOKEN_SECRET").expect("HAPI token secret is not set!");
    let Ok(generated_token) = jsonwebtoken::encode(&Header::default(), &HobbyClaims { token: token.to_string() }, &EncodingKey::from_secret(auth_secret.as_bytes())) else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };
    Ok(generated_token)
}

pub async fn hapi_token_middleware(
    State(pg_pool): State<PgPool>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let o_token: Option<&str> = request
        .headers()
        .get("hobby")
        .and_then(|value| value.to_str().ok());

    if o_token.is_none() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    if let Ok(token) = decode_hapi_token(o_token.unwrap()) {
        if HobbyModel::get_hobby_for_token(&o_token.unwrap().to_string(), &pg_pool)
            .await
            .is_err()
        {
            error!("While making a request to hapi, the hapi token is not correct! Decoded name - [{}], token - [{}]", token, o_token.unwrap());
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        return Ok(next.run(request).await);
    }
    error!("While accessing the hapi, there was no token!");
    Err(StatusCode::SERVICE_UNAVAILABLE)
}
