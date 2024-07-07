use async_graphql::{Context, FieldResult, InputObject};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::front_api::gql_models::task_gql_model::GqlTask;
use crate::front_api::gql_mutations::TaskMutations;
use crate::front_api::utils;
use crate::psql::task_psql_model::TaskModel;

#[derive(InputObject)]
pub struct CreateTaskInput {
    pub id: sqlx::types::Uuid,
    pub user_name: String,
    pub hobby_name: String,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub given_exp: i64,
    pub created_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

#[async_graphql::Object]
impl TaskMutations {
    async fn add_task(
        &self,
        ctx: &Context<'_>,
        create_task_input: CreateTaskInput,
    ) -> FieldResult<GqlTask> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        TaskModel::create_task()
    }
}