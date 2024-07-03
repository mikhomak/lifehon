use async_graphql::{Context, FieldResult, Object};
use sqlx::PgPool;

use crate::front_api::gql_models::hobby_gql_model::GqlHobby;
use crate::front_api::gql_query::HobbyQuery;
use crate::front_api::utils;
use crate::psql::hobby_psql_model::HobbyModel;

#[Object(extends)]
impl HobbyQuery {
    async fn hobbies<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<GqlHobby>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_hobbies = HobbyModel::get_all_hobbies(pool).await;
        match r_hobbies {
            Ok(hobby_models) => {
                Ok(HobbyModel::convert_all_to_gql(&hobby_models))
            }
            Err(_) => {
                Err(async_graphql::Error::new("Cannot find hobbies"))
            }
        }
    }
}