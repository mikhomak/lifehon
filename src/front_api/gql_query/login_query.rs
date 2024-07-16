use async_graphql::{Context, FieldResult, Object};
use log::error;
use sqlx::PgPool;
use crate::front_api::gql_query::LoginQuery;
use crate::front_api::{gql_auth, utils};
use crate::services;

#[Object(extends)]
impl LoginQuery {
    async fn check_token(
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> FieldResult<bool> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let is_login_enabled: bool =
            services::site_service::is_login_enabled(pool).await;

        if is_login_enabled == false {
            return Err(async_graphql::Error::new("Login failed!"));
        }

        match gql_auth::get_token(&token) {
            Ok(_) => Ok(true),
            Err(error) => {
                error!("Error while checking the token [{}], the error is [{}]", token, error.message);
                Err(async_graphql::Error::new("Token is invalid!"))
            }
        }
    }
}