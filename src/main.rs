mod psql;

use std::env;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::GraphQL;
use axum::{routing::get, Router, Json};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use crate::psql::hobby_psql_model::HobbyModel;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello world"
    }
}

async fn root(State(pg_pool): State<PgPool>) -> Result<Json<String>, (String)> {
    Ok(Json(HobbyModel::get_all_hobbies(&pg_pool).await.unwrap().get(0).unwrap().name.clone()))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app = Router::new().route("/",
                                  get(Html(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws")))).post_service(GraphQL::new(schema)))
        .route("/check", get(root))
        .with_state(db_pool);

    println!("GraphiQL IDE: http://localhost:8600");

    axum::serve(TcpListener::bind("127.0.0.1:8600").await.unwrap(), app)
        .await
        .unwrap();
}