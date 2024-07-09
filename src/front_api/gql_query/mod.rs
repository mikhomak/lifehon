use async_graphql::MergedObject;

mod hobby_query;
mod user_query;

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct HobbyQuery;

#[derive(Default)]
pub struct LoginQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, HobbyQuery);
