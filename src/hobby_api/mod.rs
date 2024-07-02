use axum::http::StatusCode;
use axum::Json;

mod errors;
pub mod hapi_auth;
pub mod hapi_hobby;
pub mod hapi_task;
pub mod hapi_user;
mod validation;


pub type HabiResult<T> = Result<Json<T>, (StatusCode, String)>;
