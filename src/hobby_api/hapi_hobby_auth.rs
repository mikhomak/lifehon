use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use log::error;
use sqlx::PgPool;

use crate::psql::hobby_psql_model::HobbyModel;

fn decode_hapi_token(token: &str) -> Result<String, StatusCode> {
    let auth_secret: String =
        dotenv::var("HAPI_TOKEN_SECRET").expect("HAPI token secret is not set!");
    let Ok(token) = simple_crypt::decrypt(token.as_bytes(), auth_secret.as_bytes()) else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };
    let Ok(hobby_name) = String::from_utf8(token.clone()) else {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    };
    Ok(hobby_name)
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
    if let Ok(hobby_name) = decode_hapi_token(o_token.unwrap()) {
        if HobbyModel::get_hobby_for_name(&hobby_name, &pg_pool)
            .await
            .is_err()
        {
            error!("While making a request to hapi, the hapi token is not correct! Decoded name - [{}], token - [{}]", hobby_name, o_token.unwrap());
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        return Ok(next.run(request).await);
    }
    error!("While accessing the hapi, there was no token!");
    Err(StatusCode::SERVICE_UNAVAILABLE)
}
