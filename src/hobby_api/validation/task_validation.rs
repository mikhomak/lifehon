use std::borrow::Cow;

use sqlx::PgPool;
use validator::ValidationError;

use crate::hobby_api::errors;
use crate::hobby_api::hapi_task::CreateTaskInput;
use crate::psql::hobby_psql_model::HobbyModel;
use crate::psql::user_psql_model::UserModel;

pub async fn validate_create_task(
    user_name: &String,
    create_task_input: &CreateTaskInput,
    pg_pool: &PgPool,
) -> Result<(), ValidationError> {
    validate_user(user_name, pg_pool).await?;
    validate_hobby(&create_task_input.hobby_name, pg_pool).await?;
    Ok(())
}

async fn validate_user(user_name: &String, pg_pool: &PgPool) -> Result<(), ValidationError> {
    match UserModel::get_user_for_name(user_name, pg_pool).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::with_message(
            ValidationError::new(""),
            Cow::from(errors::CreateUserErrors::NAME_TAKEN),
        )),
    }
}

async fn validate_hobby(hobby_name: &String, pg_pool: &PgPool) -> Result<(), ValidationError> {
    match HobbyModel::get_hobby_for_name(hobby_name, pg_pool).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::with_message(
            ValidationError::new(""),
            Cow::from(errors::CreateUserErrors::EMAIL_TAKEN),
        )),
    }
}
