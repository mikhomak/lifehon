use crate::front_api::gql_models::hobby_gql_model::GqlHobby;
use crate::front_api::gql_models::task_gql_model::GqlTasksPagination;
use crate::front_api::utils;
use crate::psql::hobby_psql_model::HobbyModel;
use crate::psql::task_psql_model::TaskModel;
use crate::psql::user_psql_model::UserModel;
use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use chrono;
use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Deserialize, Serialize)]
#[graphql(complex)]
pub struct GqlUser {
    pub id: i64,
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

    async fn all_available_hobbies(&self, ctx: &Context<'_>) -> FieldResult<Vec<GqlHobby>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_hobby_models: Result<Vec<HobbyModel>, sqlx::Error> = UserModel::get_available_hobbies_for_user_name(&self.name, pool).await;

        match r_hobby_models {
            Ok(hobby_models) => Ok(HobbyModel::convert_all_to_gql(&hobby_models)),
            Err(_) => Err(async_graphql::Error::new("Hobbies not found!")),
        }
    }


    async fn tasks(&self, ctx: &Context<'_>, page: i64) -> FieldResult<GqlTasksPagination> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_tasks_models: Result<Vec<TaskModel>, sqlx::Error> =
            UserModel::get_tasks_for_user_name(&self.name, page, pool).await;
        let task_count: i64 = TaskModel::count_tasks_for_user(&self.name, pool).await;
        match r_tasks_models {
            Ok(task_models) => Ok(GqlTasksPagination {
                tasks: TaskModel::convert_all_to_gql(&task_models),
                pages: task_count / 30,
                total_amount: Some(task_count),
            }),
            Err(error) => {
                error!("There was an error while getting the task for the user [{}], the error is [{}]", "", error.to_string());
                Err(async_graphql::Error::new("[] Error"))
            }
        }
    }
}
