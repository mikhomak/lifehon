use async_graphql::{Context, FieldResult, Object};
use log::error;
use sqlx::PgPool;
use crate::front_api::gql_query::LoginQuery;
use crate::front_api::{gql_auth, utils};
use crate::front_api::gql_models::user_gql_model::GqlUser;
use crate::psql::user_psql_model::UserModel;
use crate::services;

#[Object(extends)]
impl LoginQuery {
    async fn check_token(
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> FieldResult<GqlUser> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let is_login_enabled: bool =
            services::site_service::is_login_enabled(pool).await;

        if is_login_enabled == false {
            return Err(async_graphql::Error::new("Login failed!"));
        }

        let r_token = gql_auth::get_token(&token);

        let Ok(user_claims) = r_token else {
            error!("Error while checking the token [{}], the error is [{}]", token, r_token.err().unwrap().message);
            return Err(async_graphql::Error::new("Token is invalid!"));
        };

        match UserModel::get_user_for_name(&user_claims.claims.name, pool).await {
            Ok(user_model) => Ok(UserModel::convert_to_gql(&user_model)),
            Err(error) => {
                error!("Token is wrong [{}] the error is [{}]", token, error.to_string());
                Err(async_graphql::Error::new("Token is wrong!"))
            }
        }
    }
}
