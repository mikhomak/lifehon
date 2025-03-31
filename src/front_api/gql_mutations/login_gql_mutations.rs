use async_graphql::{Context, FieldResult, InputObject};
use log::error;
use sqlx::PgPool;

use crate::front_api::utils;
use crate::front_api::gql_auth::create_token;
use crate::front_api::gql_mutations::LoginMutations;
use crate::psql::user_psql_model::UserModel;
use crate::services;

#[derive(InputObject)]
pub struct LoginInput {
    pub name: String,
    pub password: String,
}

#[async_graphql::Object]
impl LoginMutations {
    async fn login_user(
        &self,
        ctx: &Context<'_>,
        login_input: LoginInput,
    ) -> FieldResult<String> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let is_login_enabled: bool =
            services::site_service::is_login_enabled(pool).await;

        if is_login_enabled == false {
            return Err(async_graphql::Error::new("Login failed!"));
        }

        let r_user: Result<UserModel, sqlx::Error> =
            UserModel::login_user(&login_input.name, &login_input.password, &pool).await;

        match r_user {
            Ok(user_model) => Ok(return match create_token(&user_model.name, &user_model.email) {
                Ok(token) => Ok(token),
                Err(_error) => Err(async_graphql::Error::new("Login failed!"))
            }),
            Err(_) => {
                error!("Cannot login user!");
                Err(async_graphql::Error::new("Login failed!"))
            }
        }
    }

}
