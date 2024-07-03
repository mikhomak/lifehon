use async_graphql::Context;
use async_graphql::FieldResult;
use async_graphql::Object;
use log::error;
use sqlx::PgPool;

use crate::front_api::gql_models::user_gql_model::GqlUser;
use crate::front_api::gql_query::UserQuery;
use crate::front_api::utils;
use crate::psql::user_psql_model::UserModel;

#[Object(extends)]
impl UserQuery {
    async fn users<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<GqlUser>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_users: Result<Vec<UserModel>, sqlx::Error> = UserModel::get_all(&pool).await;
        match r_users {
            Ok(users) => Ok(UserModel::convert_all_to_gql(&users)),
            Err(error) => {
                error!(
                            "Users couldn't be fetched from the db due to error {}",
                            error.to_string()
                        );
                Err(async_graphql::Error::new(
                    "Users not found, error encountered",
                ))
            }
        }
    }

    async fn tasks<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<>>
}