use crate::services;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::Json;
use log::error;
use sqlx::PgPool;

mod errors;
pub mod hapi_auth;
pub mod hapi_hobby;
pub mod hapi_hobby_auth;
pub mod hapi_task;
pub mod hapi_user;
mod validation;

pub type HapiResult<T> = Result<Json<T>, (StatusCode, &'static str)>;

pub async fn is_hapi_enabled_middleware(
    State(pg_pool): State<PgPool>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !services::site_service::is_hapi_allowed(&pg_pool).await {
        error!("Hapi at the site level is not allowed!");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    Ok(next.run(request).await)
}
