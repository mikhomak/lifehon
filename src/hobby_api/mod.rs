
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

pub mod hapi_auth;
pub mod hapi_hobby;
pub mod hapi_task;
pub mod hapi_user;
mod validation;
mod errors;

#[derive(Clone, Debug)]
pub enum HabiErrorCode {
    Login001,
    User001,
    User002,
}

#[derive(Debug, Clone)]
pub struct HabiError {
    pub status_code: StatusCode,
    pub message: String,
    pub custom_error_code: HabiErrorCode,
}

pub type HabiResult<T> = Result<Json<T>, (StatusCode, String)>;

#[derive(Debug, Clone)]
pub struct ValidationContext<'a> {
    pub pg_pool: &'a PgPool,
}

impl<'a> ValidationContext<'a> {
    pub fn new(pg_pool: &'a PgPool) -> Self {
        Self { pg_pool }
    }
}