use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::gql_models::post_gql_model::PostsPagination;
use crate::psql_models::post_psql_model::PostModel;
use crate::utils;

#[derive(SimpleObject, Deserialize, Serialize)]
#[graphql(complex)]
pub struct User {
    pub id: sqlx::types::Uuid,
}
