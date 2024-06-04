use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::GraphQL;
use axum::{
    routing::get,
    Router,
};
use axum::response::Html;
use tokio::net::TcpListener;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello world"
    }
}
#[tokio::main]
async fn main() {

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();


    let app = Router::new().route("/", get(Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))).post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8600");

    axum::serve(TcpListener::bind("127.0.0.1:8600").await.unwrap(), app)
        .await
        .unwrap();
}