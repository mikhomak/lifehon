use async_graphql::ErrorExtensions;
use log::error;

pub fn error_database_not_setup() -> async_graphql::Error {
    error!("Error at logging a user. Database is not set in context!");
    async_graphql::Error::new("[SERVER_001] Server error!")
        .extend_with(|_, e| e.set("error_code", "SERVER_001"))
}
