use crate::front_api::gql_models::user_gql_model::GqlUser;
use crate::front_api::gql_mutations::UserMutations;
use crate::front_api::utils;
use crate::hobby_api::hapi_user::CreateUserInput;
use crate::psql::user_psql_model::UserModel;
use crate::services;
use async_graphql::{Context, FieldResult, InputObject};
use log::error;
use sqlx::PgPool;

#[derive(InputObject)]
pub struct UserRegistrationInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub consent: bool,
    pub public_profile: bool,
}

#[async_graphql::Object]
impl UserMutations {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        user_input: UserRegistrationInput,
    ) -> FieldResult<GqlUser> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let is_registration_enabled: bool =
            services::site_service::is_registration_allowed(pool).await;
        if is_registration_enabled == false {
            return Err(async_graphql::Error::new("Registration failed!"));
        }

        let create_user_input: CreateUserInput = CreateUserInput {
            name: user_input.name,
            consent: user_input.consent,
            public_profile: user_input.public_profile,
            email: user_input.email,
            password: user_input.password,
        };
        let r_created_user: Result<UserModel, sqlx::Error> =
            UserModel::create_user(&create_user_input, &pool).await;

        match r_created_user {
            Ok(created_user) => Ok(UserModel::convert_to_gql(&created_user)),
            Err(_) => {
                error!("Cannot create a user due to error");
                Err(async_graphql::Error::new("Registration failed!"))
            }
        }
    }
}
