use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct GqlTask {
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

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct GqlTasksPagination {
    pub tasks: Vec<GqlTask>,
    pub pages: i64,
    pub total_amount: Option<i64>,
}
