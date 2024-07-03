use async_graphql::{ComplexObject, SimpleObject};
use chrono;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
impl GqlUser{

}