use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct GqlHobby {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub enabled: bool,
    pub external_url: String,
}
