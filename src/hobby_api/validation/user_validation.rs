use std::borrow::Cow;
use sqlx::PgPool;
use validator::ValidationError;

use crate::hobby_api::errors;
use crate::hobby_api::hapi_user::CreateUserInput;
use crate::psql::user_psql_model::UserModel;

/// We need this custom method validation cuz #[Validator] does not support async custom functions
pub async fn validate_user(create_user_input: &CreateUserInput, pg_pool: &PgPool) -> Result<(), ValidationError> {
    if !create_user_input.consent {
        return Err(ValidationError::with_message(ValidationError::new(""), Cow::from(errors::CreateUserErrors::CONSENT_NOT_AGREED)));
    }
    validate_unique_username(&create_user_input.name, pg_pool).await?;
    validate_unique_email(&create_user_input.email, pg_pool).await?;
    Ok(())
}

async fn validate_unique_username(user_name: &String, pg_pool: &PgPool) -> Result<(), ValidationError> {
    match UserModel::get_user_for_name(user_name, pg_pool).await {
        Ok(_) => {
            Err(ValidationError::with_message(ValidationError::new(""), Cow::from(errors::CreateUserErrors::NAME_TAKEN)))
        }
        Err(_) => { Ok(()) }
    }
}


async fn validate_unique_email(email: &String, pg_pool: &PgPool) -> Result<(), ValidationError> {
    match UserModel::get_user_for_email(email, pg_pool).await {
        Ok(_) => {
            Err(ValidationError::with_message(ValidationError::new(""), Cow::from(errors::CreateUserErrors::EMAIL_TAKEN)))
        }
        Err(_) => { Ok(()) }
    }
}