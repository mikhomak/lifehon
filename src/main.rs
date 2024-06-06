use std::env;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::GraphQL;
use axum::response::{Html, IntoResponse};
use axum::routing::post;
use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod hobby_api;
mod psql;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello world"
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app = Router::new()
        .route(
            "/",
            get(Html(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
            )))
            .post_service(GraphQL::new(schema)),
        )
        .route("/hobbies", get(hobby_api::hapi_hobby::get_all_hobbies))
        .route("/user/:name", get(hobby_api::hapi_user::get_user_for_name))
        .route("/user/:name/task/", post(hobby_api::hapi_task::create_task))
        .route("/user/", post(hobby_api::hapi_user::create_user))
        .route("/user/login/", post(hobby_api::hapi_auth::login_user))
        .route("/user/hobby/", post(hobby_api::hapi_user::add_hobby))
        .with_state(db_pool);

    println!("GraphiQL IDE: http://localhost:8600");

    axum::serve(TcpListener::bind("127.0.0.1:8600").await.unwrap(), app)
        .await
        .unwrap();
}
