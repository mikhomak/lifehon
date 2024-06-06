use std::fmt::Display;

use axum::http::StatusCode;
use axum::Json;

pub mod hapi_auth;
pub mod hapi_hobby;
pub mod hapi_task;
pub mod hapi_user;

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
