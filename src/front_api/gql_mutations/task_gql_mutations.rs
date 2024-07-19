use async_graphql::{Context, ErrorExtensions, FieldResult, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

use crate::front_api::gql_models::task_gql_model::GqlTask;
use crate::front_api::gql_models::user_gql_model::GqlUser;
use crate::front_api::gql_mutations::TaskMutations;
use crate::front_api::utils;
use crate::hobby_api::hapi_task::CreateTaskInput;
use crate::psql::task_psql_model::TaskModel;
use crate::psql::user_psql_model::UserModel;

#[derive(InputObject)]
pub struct GqlCreateTaskInput {
    pub id: sqlx::types::Uuid,
    pub user_name: String,
    pub hobby_name: String,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub given_exp: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[async_graphql::Object]
impl TaskMutations {
    async fn add_task(
        &self,
        ctx: &Context<'_>,
        create_task_input: GqlCreateTaskInput,
    ) -> FieldResult<GqlTask> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_user_model: Result<&UserModel, async_graphql::Error> = ctx.data::<UserModel>();

        let Ok(user_model) = r_user_model else {
            error!("Error at creating a task. User is not set!");
            return Err(async_graphql::Error::new("[SERVER_001] Server error!")
                .extend_with(|_, e| e.set("error_code", "SERVER_001")));
        };
        match TaskModel::create_task(
            &user_model.email,
            &create_task_input.hobby_name.clone(),
            &CreateTaskInput {
                name: create_task_input.name.clone(),
                external_id: create_task_input.external_id.clone(),
                description: create_task_input.description,
                public: create_task_input.public,
                given_exp: create_task_input.given_exp,
                created_at: create_task_input.created_at,
                finished_at: create_task_input.finished_at,
            },
            &pool,
        )
            .await
        {
            Ok(task_model) => Ok(TaskModel::convert_to_gql(&task_model)),
            Err(error) => {
                error!("Error while creating a task! User name is [{}], task name is [{}], hobby is [{}], external id is [{}]. The error is [{}]",
               create_task_input.user_name,
               create_task_input.name,
               create_task_input.hobby_name,
               create_task_input.external_id,
               error.to_string());

                Err(async_graphql::Error::new("").extend_with(|_, e| e.set("error_code", "asdsad")))
            }
        }
    }
}
