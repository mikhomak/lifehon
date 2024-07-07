use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::Json;
use axum::middleware::Next;
use axum::response::Response;
use sqlx::PgPool;
use crate::psql::user_psql_model::UserModel;
use crate::services;

mod errors;
pub mod hapi_auth;
pub mod hapi_hobby;
pub mod hapi_task;
pub mod hapi_user;
mod validation;


pub type HabiResult<T> = Result<Json<T>, (StatusCode, &'static str)>;


pub async fn is_hapi_enabled_middleware(
    State(pg_pool): State<PgPool>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if services::site_service::is_hapi_allowed(&pg_pool) {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    Ok(next.run(request).await)
}
