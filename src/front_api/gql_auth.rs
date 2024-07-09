use crate::front_api::gql_guards::Role;
use crate::psql::user_psql_model::UserModel;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct Token(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GqlLifehonClaims {
    pub id: String,
    pub email: String,
    exp: usize,
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

pub async fn gql_auth_middleware(
    State(pg_pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header = req.headers().clone();
    let extensions = req.extensions_mut();
    if let Some(token) = get_token_from_headers(&header) {
        let r_token = get_token(&token.0);
        match r_token {
            Ok(jsonwebtoken) => {
                if let Ok(user_model) =
                    UserModel::get_user_for_email(&jsonwebtoken.claims.email, &pg_pool).await
                {
                    extensions.insert(user_model);
                    extensions.insert(jsonwebtoken.claims);
                    extensions.insert(Role::User);
                }
            }
            Err(error) => {
                error!(
                    "Cannot decode a token for the user with token [{}] due to error [{}]",
                    &token.0, error.message
                );
                extensions.insert(Role::Anon);
            }
        };
    } else {
        extensions.insert(Role::Anon);
    }
    Ok(next.run(req).await)
}

pub fn create_token(id: &String, email: &String) -> Result<String, async_graphql::Error> {
    let my_claims = GqlLifehonClaims {
        id: id.clone(),
        email: email.clone(),
        exp: 100000000000000,
    };
    let auth_secret = dotenv::var("AUTH_SECRET").expect("Auth secret is not set!");
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(auth_secret.as_ref()),
    )?;
    Ok(token)
}

pub fn get_token(token: &String) -> Result<TokenData<GqlLifehonClaims>, async_graphql::Error> {
    let auth_secret = dotenv::var("AUTH_SECRET").expect("Auth secret is not set!");
    let token: TokenData<GqlLifehonClaims> = decode::<GqlLifehonClaims>(
        token,
        &DecodingKey::from_secret(auth_secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token)
}
