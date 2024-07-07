use std::env;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::parser::types::OperationType::Mutation;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{middleware, Router, routing::get};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use axum::routing::post;
use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use crate::front_api::gql_mutations::Mutations;

use crate::front_api::gql_query::Query;

mod hobby_api;
mod psql;
mod services;
mod front_api;

pub type LifehonSchema = Schema<Query, Mutations, EmptySubscription>;

async fn graphql_handler(
    State(schema): State<LifehonSchema>,
    _headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let hapi_routes = Router::new()
        .route("/user/:name", get(hobby_api::hapi_user::get_user_for_name))
        .route("/user/task/", post(hobby_api::hapi_task::create_task))
        .route("/user/hobby/", post(hobby_api::hapi_user::add_hobby))
        .route_layer(middleware::from_fn_with_state(
            db_pool.clone(),
            hobby_api::hapi_auth::auth_middleware,
        ))
        .route("/user/", post(hobby_api::hapi_user::create_user))
        .route("/hobbies", get(hobby_api::hapi_hobby::get_all_hobbies))
        .route(
            "/user/login/token/",
            post(hobby_api::hapi_auth::check_token),
        )
        .route("/user/login/", post(hobby_api::hapi_auth::login_user))
        .with_state(db_pool.clone())
        .route_layer(middleware::from_fn_with_state(
            db_pool.clone(),
            hobby_api::is_hapi_enabled_middleware,
        ));

    let schema: LifehonSchema =
        Schema::build(Query::default(), Mutations::default(), EmptySubscription)
            .data(db_pool)
            .finish();

    let app = Router::new()
        .nest("/api/v1/", hapi_routes)
        .route("/front-api/v1/", post(graphql_handler))
        .route("/front-api/v1/playground", get(graphql_playground))
        .with_state(schema);


    println!("GraphQL IDE: http://localhost:8600/front-api/v1/playground");

    axum::serve(TcpListener::bind("127.0.0.1:8600").await.unwrap(), app)
        .await
        .unwrap();
}
