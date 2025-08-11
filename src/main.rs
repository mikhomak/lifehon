use std::env;

use crate::front_api::gql_mutations::Mutations;
use crate::front_api::gql_query::Query;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use axum::routing::post;
use axum::{middleware, routing::get, Router};
use axum_macros::debug_handler;
use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod front_api;
mod hobby_api;
mod psql;
mod services;

pub type LifehonSchema = Schema<Query, Mutations, EmptySubscription>;

#[debug_handler]
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
        .route("/user/{name}", get(hobby_api::hapi_user::get_user_for_name))
        .route("/user/task", post(hobby_api::hapi_task::create_task))
        .route("/user/hobby", post(hobby_api::hapi_user::add_hobby))
        // user auth
        .route_layer(middleware::from_fn_with_state(
            db_pool.clone(),
            hobby_api::hapi_auth::auth_middleware,
        ))
        .route("/user", post(hobby_api::hapi_user::create_user))
        .route(
            "/user/login/token",
            post(hobby_api::hapi_auth::check_token),
        )
        // hapi auth
        .route_layer(middleware::from_fn_with_state(
            db_pool.clone(),
            hobby_api::is_hapi_enabled_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(
            db_pool.clone(),
            hobby_api::hapi_hobby_auth::hapi_token_middleware,
        ))
        .route("/hobbies", get(hobby_api::hapi_hobby::get_all_hobbies))
        // no auth
        .with_state(db_pool.clone());

    let admin_routes = Router::new()
        .route("/hobby", post(hobby_api::hapi_hobby::create_hobby))
        .with_state(db_pool.clone());


    let schema: LifehonSchema =
        Schema::build(Query::default(), Mutations::default(), EmptySubscription)
            .data(db_pool.clone())
            .finish();

    let app = Router::new()
        .route(
            "/front-api/v1/",
            post(graphql_handler).route_layer(middleware::from_fn_with_state(
                db_pool.clone(),
                front_api::gql_auth::gql_auth_middleware,
            )),
        )
        .route("/front-api/v1/playground", get(graphql_playground))
        .with_state(schema)
        .nest("/api/v1/", hapi_routes)
        .nest("/api/v1/admin/", admin_routes);

    println!("GraphQL IDE: http://localhost:8600/front-api/v1/playground");
    println!("Hapi is: http://localhost:8600/api/v1/");

    axum::serve(TcpListener::bind("127.0.0.1:8600").await.unwrap(), app)
        .await
        .unwrap();
}
