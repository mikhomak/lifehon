use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use chrono;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::front_api::gql_models::hobby_gql_model::GqlHobby;
use crate::front_api::utils;
use crate::psql::hobby_psql_model::HobbyModel;
use crate::psql::user_psql_model::UserModel;

#[derive(SimpleObject, Deserialize, Serialize)]
#[graphql(complex)]
pub struct GqlUser {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub email: String,
    pub login_enabled: bool,
    #[graphql(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub consent: bool,
}

#[ComplexObject]
impl GqlUser {
    async fn hobbies(&self, ctx: &Context<'_>) -> FieldResult<Vec<GqlHobby>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_hobby_models: Result<Vec<HobbyModel>, sqlx::Error> =
            UserModel::get_hobbies_for_user_name(&self.name, pool).await;
        match r_hobby_models {
            Ok(hobby_models) => Ok(HobbyModel::convert_all_to_gql(&hobby_models)),
            Err(_) => Err(async_graphql::Error::new("Hobbies not found!")),
        }
    }
}